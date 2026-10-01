use std::fmt::Display;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use objc2_app_kit::NSWorkspace;
use serde_json::{json, Value};
use tauri::{ActivationPolicy, AppHandle, Emitter, Manager, State};
use tauri_nspanel::{
    tauri_panel, CollectionBehavior, ManagerExt, PanelLevel, StyleMask, WebviewWindowExt,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

use crate::timing::{OpenTimer, REQUIRED_OPENS};
use crate::{accessibility, keys, layout, pasteboard};

const PANEL_LABEL: &str = "panel";
const SENTINEL: &str = "R0-SENTINEL-7f3a";
const MODIFIER_RELEASE_TIMEOUT: Duration = Duration::from_millis(1000);
const FOCUS_SETTLE_DELAY: Duration = Duration::from_millis(50);
const RESTORE_DELAY: Duration = Duration::from_millis(250);
const RESTORE_DELAY_OVERRIDE_VARIABLE: &str = "R0_RESTORE_DELAY_MS";
const MILLIS_PER_SECOND: f64 = 1000.0;

tauri_panel! {
    panel!(R0Panel {
        config: {
            can_become_key_window: true,
            can_become_main_window: false,
            is_floating_panel: true
        }
    })
}

#[derive(Default)]
struct Spike {
    timer: OpenTimer,
    target_pid: Mutex<Option<i32>>,
    paste_running: Arc<AtomicBool>,
}

struct PasteSlot(Arc<AtomicBool>);

impl PasteSlot {
    fn claim(flag: &Arc<AtomicBool>) -> Option<Self> {
        (!flag.swap(true, Ordering::SeqCst)).then(|| Self(Arc::clone(flag)))
    }
}

impl Drop for PasteSlot {
    fn drop(&mut self) {
        self.0.store(false, Ordering::SeqCst);
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_nspanel::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    if event.state == ShortcutState::Pressed {
                        open_panel(app);
                    }
                })
                .build(),
        )
        .manage(Spike::default())
        .invoke_handler(tauri::generate_handler![
            panel_ready,
            panel_key,
            panel_hide,
            panel_paste,
            open_accessibility_settings
        ])
        .setup(|app| {
            app.set_activation_policy(ActivationPolicy::Accessory);
            convert_to_panel(app.handle())?;
            let chord = Shortcut::new(Some(Modifiers::SUPER | Modifiers::ALT), Code::KeyV);
            app.global_shortcut().register(chord)?;
            log(
                "started",
                json!({ "accessibility_trusted": accessibility::is_trusted() }),
            );
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("tauri application run");
}

fn convert_to_panel(app: &AppHandle) -> tauri::Result<()> {
    let window = app.get_webview_window(PANEL_LABEL).expect("panel window");
    let panel = window.to_panel::<R0Panel>()?;
    panel.set_level(PanelLevel::Floating.into());
    panel
        .add_style_mask(StyleMask::empty().nonactivating_panel().into())
        .expect("non-activating style mask");
    panel.set_collection_behavior(
        CollectionBehavior::new()
            .full_screen_auxiliary()
            .can_join_all_spaces()
            .into(),
    );
    panel.set_hides_on_deactivate(false);
    Ok(())
}

fn open_panel(app: &AppHandle) {
    let spike = app.state::<Spike>();
    spike.timer.shortcut_fired();
    let target = frontmost_pid();
    *spike.target_pid.lock().expect("target lock") = target;
    let state = if accessibility::is_trusted() {
        "ready"
    } else {
        "permission"
    };
    log("shortcut", json!({ "target_pid": target, "state": state }));
    let window = app.get_webview_window(PANEL_LABEL).expect("panel window");
    report_failure("center", window.center());
    app.get_webview_panel(PANEL_LABEL)
        .expect("panel")
        .show_and_make_key();
    report_failure(
        "emit panel:show",
        app.emit_to(PANEL_LABEL, "panel:show", state),
    );
}

#[tauri::command]
fn panel_ready(spike: State<Spike>, permission_shown: bool) {
    let Some(elapsed) = spike.timer.first_frame_acked() else {
        return;
    };
    let target = *spike.target_pid.lock().expect("target lock");
    log(
        "panel_ready",
        json!({
            "ms": elapsed.as_secs_f64() * MILLIS_PER_SECOND,
            "target_pid": target,
            "frontmost_pid": frontmost_pid(),
            "permission_shown": permission_shown,
        }),
    );
    let report = spike.timer.report();
    if report.opens >= REQUIRED_OPENS {
        log(
            "open_report",
            json!({
                "opens": report.opens,
                "p95_ms": report.p95.as_secs_f64() * MILLIS_PER_SECOND,
                "max_ms": report.max.as_secs_f64() * MILLIS_PER_SECOND,
                "passes": report.passes(),
            }),
        );
    }
}

#[tauri::command]
fn panel_key(key: String) {
    log("panel_key", json!({ "key": key }));
}

#[tauri::command]
fn panel_hide(app: AppHandle) {
    hide_panel(&app);
}

#[tauri::command]
fn panel_paste(app: AppHandle, spike: State<Spike>) -> Result<(), String> {
    let Some(slot) = PasteSlot::claim(&spike.paste_running) else {
        return Ok(());
    };
    if !accessibility::is_trusted() {
        log("paste_blocked", json!({ "reason": "accessibility" }));
        return Err("Accessibility permission is needed to paste".to_owned());
    }
    spawn(move || {
        let _slot = slot;
        if let Err(reason) = paste_sequence(&app) {
            log("paste_failed", json!({ "reason": reason }));
            report_failure(
                "emit paste:failed",
                app.emit_to(PANEL_LABEL, "paste:failed", reason),
            );
        }
    });
    Ok(())
}

#[tauri::command]
fn open_accessibility_settings() -> Result<(), String> {
    accessibility::open_settings().map_err(|error| error.to_string())
}

fn paste_sequence(app: &AppHandle) -> Result<(), String> {
    objc2::rc::autoreleasepool(|_| paste_steps(app))
}

fn paste_steps(app: &AppHandle) -> Result<(), String> {
    let entered = Instant::now();
    let target = *app.state::<Spike>().target_pid.lock().expect("target lock");
    let keycode = on_main_thread(app, layout::paste_keycode)??;
    let prior = pasteboard::snapshot();
    let written = pasteboard::write_transient_text(SENTINEL);
    let mut restoring = pasteboard::RestoreOnDrop::new(prior, written);
    let hiding_app = app.clone();
    on_main_thread(app, move || hide_panel(&hiding_app))?;
    let modifiers_released = keys::wait_modifiers_released(MODIFIER_RELEASE_TIMEOUT);
    sleep(FOCUS_SETTLE_DELAY);
    if frontmost_pid() != target {
        return Err("the target app lost focus before the paste".to_owned());
    }
    keys::post_command_chord(keycode)?;
    log(
        "paste_posted",
        json!({
            "ms": entered.elapsed().as_secs_f64() * MILLIS_PER_SECOND,
            "keycode": keycode,
            "modifiers_released": modifiers_released,
            "frontmost_pid": frontmost_pid(),
        }),
    );
    sleep(restore_delay());
    let restored = restoring.restore_now();
    log(
        "paste_done",
        json!({ "restored": restored, "restore_delay_ms": restore_delay().as_millis() as u64 }),
    );
    Ok(())
}

fn report_failure(step: &str, result: Result<(), impl Display>) {
    if let Err(error) = result {
        log(
            "step_failed",
            json!({ "step": step, "error": error.to_string() }),
        );
    }
}

fn restore_delay() -> Duration {
    std::env::var(RESTORE_DELAY_OVERRIDE_VARIABLE)
        .ok()
        .and_then(|millis| millis.parse().ok())
        .map_or(RESTORE_DELAY, Duration::from_millis)
}

fn on_main_thread<T: Send + 'static>(
    app: &AppHandle,
    work: impl FnOnce() -> T + Send + 'static,
) -> Result<T, String> {
    let (sender, receiver) = mpsc::channel();
    app.run_on_main_thread(move || {
        let _ = sender.send(work());
    })
    .map_err(|error| error.to_string())?;
    receiver.recv().map_err(|error| error.to_string())
}

fn hide_panel(app: &AppHandle) {
    app.get_webview_panel(PANEL_LABEL).expect("panel").hide();
    log("panel_hidden", json!({ "frontmost_pid": frontmost_pid() }));
}

fn frontmost_pid() -> Option<i32> {
    NSWorkspace::sharedWorkspace()
        .frontmostApplication()
        .map(|application| application.processIdentifier())
}

fn log(event: &str, fields: Value) {
    let epoch_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0.0, |elapsed| elapsed.as_secs_f64() * MILLIS_PER_SECOND);
    println!(
        "{}",
        json!({ "t": epoch_ms, "event": event, "fields": fields })
    );
}

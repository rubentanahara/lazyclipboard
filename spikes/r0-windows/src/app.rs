use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use r0_common::OpenTimer;
use tauri::{AppHandle, Emitter, Manager, State, WebviewWindow};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutEvent, ShortcutState};

use crate::config::{Config, FocusMode};
use crate::describe;
use crate::win32::{self, ClipboardSnapshot, Target};

const PANEL_LABEL: &str = "panel";
const PANEL_SHOWN_EVENT: &str = "panel-shown";
const LOG_FILE_NAME: &str = "lazyclipboard-r0-windows.log";
const CONFIG_ERROR_EXIT_CODE: i32 = 2;
const MODIFIER_RELEASE_TIMEOUT: Duration = Duration::from_millis(1000);
const STATE_CHANGE_TIMEOUT: Duration = Duration::from_millis(500);
const STATE_POLL_INTERVAL: Duration = Duration::from_millis(5);

struct Spike {
    config: Config,
    timer: OpenTimer,
    target: Mutex<Option<Target>>,
}

impl Spike {
    fn target(&self) -> MutexGuard<'_, Option<Target>> {
        self.target.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

pub fn run() {
    let config = match Config::from_lookup(|name| std::env::var(name).ok()) {
        Ok(config) => config,
        Err(message) => {
            log(&format!("config error: {message}"));
            std::process::exit(CONFIG_ERROR_EXIT_CODE);
        }
    };
    let result = tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(on_shortcut)
                .build(),
        )
        .manage(Spike {
            config,
            timer: OpenTimer::new(),
            target: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![panel_ready, panel_hide, paste])
        .setup(|app| {
            let spike = app.state::<Spike>();
            let panel = app
                .get_webview_window(PANEL_LABEL)
                .ok_or("panel window is missing")?;
            if spike.config.focus_mode == FocusMode::NoActivate {
                panel.set_focusable(false)?;
            }
            app.global_shortcut()
                .register(spike.config.shortcut.as_str())
                .inspect_err(|error| {
                    log(&format!(
                        "register {} failed: {error}",
                        spike.config.shortcut
                    ))
                })?;
            log(&format!(
                "ready shortcut={} focus={:?} settle_ms={} restore_ms={} log={}",
                spike.config.shortcut,
                spike.config.focus_mode,
                spike.config.focus_settle.as_millis(),
                spike.config.restore_delay.as_millis(),
                log_path().display(),
            ));
            Ok(())
        })
        .run(tauri::generate_context!());
    if let Err(error) = result {
        log(&format!("tauri exited with error: {error}"));
    }
}

fn on_shortcut(app: &AppHandle, _shortcut: &Shortcut, event: ShortcutEvent) {
    if event.state != ShortcutState::Pressed {
        return;
    }
    let spike = app.state::<Spike>();
    let Some(panel) = app.get_webview_window(PANEL_LABEL) else {
        log("shortcut fired but the panel window is missing");
        return;
    };
    if let Err(error) = toggle_panel(&panel, &spike) {
        log(&format!("toggle failed: {error}"));
    }
}

fn toggle_panel(panel: &WebviewWindow, spike: &Spike) -> Result<(), String> {
    if panel.is_visible().map_err(describe)? {
        return close_panel(panel, spike);
    }
    spike.timer.shortcut_fired();
    open_panel(panel, spike)
}

fn open_panel(panel: &WebviewWindow, spike: &Spike) -> Result<(), String> {
    *spike.target() = win32::foreground_target();
    panel.center().map_err(describe)?;
    panel.show().map_err(describe)?;
    win32::add_tool_window_style(panel_hwnd(panel)?)?;
    if spike.config.focus_mode == FocusMode::Activate {
        panel.set_focus().map_err(describe)?;
    }
    panel.emit(PANEL_SHOWN_EVENT, ()).map_err(describe)
}

#[tauri::command]
fn panel_ready(panel: WebviewWindow, spike: State<Spike>) {
    let Some(elapsed) = spike.timer.first_frame_acked() else {
        log("first-frame ack without a pending shortcut");
        return;
    };
    let target = *spike.target();
    let foreground = win32::foreground_hwnd();
    let panel_is_foreground = panel_hwnd(&panel).is_ok_and(|hwnd| hwnd == foreground);
    log(&format!(
        "open ready_ms={:.1} target_hwnd={:?} blocked_by_uipi={:?} foreground_is_target={} panel_is_foreground={panel_is_foreground} {}",
        elapsed.as_secs_f64() * 1000.0,
        target.map(|target| target.hwnd),
        target.map(|target| target.blocked_by_uipi),
        target.is_some_and(|target| target.hwnd == foreground),
        spike.timer.report(),
    ));
}

#[tauri::command]
fn panel_hide(panel: WebviewWindow, spike: State<Spike>) -> Result<(), String> {
    close_panel(&panel, &spike)
}

#[tauri::command]
async fn paste(panel: WebviewWindow, spike: State<'_, Spike>, text: String) -> Result<(), String> {
    let result = paste_sequence(&panel, &spike, &text);
    if let Err(error) = &result {
        log(&format!("paste failed: {error}"));
    }
    result
}

fn paste_sequence(panel: &WebviewWindow, spike: &Spike, text: &str) -> Result<(), String> {
    let owner = panel_hwnd(panel)?;
    let target = claim_target(spike)?;
    let (snapshot, written_sequence) = write_sentinel(owner, text).inspect_err(|_| {
        *spike.target() = Some(target);
    })?;
    let pasted = inject_paste(panel, spike, target);
    let restored = restore_if_unchanged(owner, &snapshot, written_sequence);
    pasted.and(restored)
}

fn claim_target(spike: &Spike) -> Result<Target, String> {
    let mut slot = spike.target();
    let target = (*slot).ok_or("no target window was recorded")?;
    if target.blocked_by_uipi {
        return Err("target runs as administrator, paste is blocked by UIPI".to_owned());
    }
    if !win32::is_window(target.hwnd) {
        return Err("target window is gone".to_owned());
    }
    *slot = None;
    Ok(target)
}

fn write_sentinel(owner: isize, text: &str) -> Result<(ClipboardSnapshot, u32), String> {
    let snapshot = win32::snapshot_clipboard()?;
    log(&format!("snapshot {}", snapshot.describe()));
    match win32::write_text(owner, text) {
        Ok(sequence) => Ok((snapshot, sequence)),
        Err(error) => {
            let _ = restore_if_unchanged(owner, &snapshot, win32::clipboard_sequence());
            Err(error)
        }
    }
}

fn inject_paste(panel: &WebviewWindow, spike: &Spike, target: Target) -> Result<(), String> {
    hide_and_return_focus(panel, spike, target)?;
    if !win32::wait_modifiers_released(MODIFIER_RELEASE_TIMEOUT) {
        return Err("a modifier is still held, paste cancelled".to_owned());
    }
    sleep(spike.config.focus_settle);
    if win32::foreground_hwnd() != target.hwnd {
        return Err("target is not in front, paste cancelled".to_owned());
    }
    win32::send_paste_chord(target.hwnd)?;
    log("pasted");
    sleep(spike.config.restore_delay);
    Ok(())
}

fn restore_if_unchanged(
    owner: isize,
    snapshot: &ClipboardSnapshot,
    written_sequence: u32,
) -> Result<(), String> {
    match win32::restore_clipboard(owner, snapshot, written_sequence) {
        Ok(true) => log("clipboard restored"),
        Ok(false) => log("clipboard changed by another process, not restoring"),
        Err(error) => {
            log(&format!("clipboard restore failed: {error}"));
            return Err(error);
        }
    }
    Ok(())
}

fn close_panel(panel: &WebviewWindow, spike: &Spike) -> Result<(), String> {
    let target = spike.target().take();
    let panel_in_front = panel_hwnd(panel)? == win32::foreground_hwnd();
    match target {
        Some(target) if panel_in_front => hide_and_return_focus(panel, spike, target),
        _ => panel.hide().map_err(describe),
    }
}

fn hide_and_return_focus(
    panel: &WebviewWindow,
    spike: &Spike,
    target: Target,
) -> Result<(), String> {
    let hidden_at = Instant::now();
    panel.hide().map_err(describe)?;
    if !wait_until(|| !panel.is_visible().is_ok_and(|visible| visible)) {
        log("panel still visible after hide");
    }
    if spike.config.focus_mode == FocusMode::Activate && !win32::activate(target.hwnd) {
        log("SetForegroundWindow(target) was refused");
    }
    let returned = wait_until(|| win32::foreground_hwnd() == target.hwnd);
    log(&format!(
        "hide focus_returned={returned} focus_return_ms={:.1}",
        hidden_at.elapsed().as_secs_f64() * 1000.0,
    ));
    Ok(())
}

fn wait_until(condition: impl Fn() -> bool) -> bool {
    let deadline = Instant::now() + STATE_CHANGE_TIMEOUT;
    while !condition() {
        if Instant::now() >= deadline {
            return false;
        }
        sleep(STATE_POLL_INTERVAL);
    }
    true
}

fn panel_hwnd(panel: &WebviewWindow) -> Result<isize, String> {
    panel.hwnd().map(|hwnd| hwnd.0 as isize).map_err(describe)
}

fn log_path() -> PathBuf {
    std::env::temp_dir().join(LOG_FILE_NAME)
}

fn log(line: &str) {
    let epoch_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_millis())
        .unwrap_or_default();
    let entry = format!("{epoch_ms} {line}\n");
    eprint!("{entry}");
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path())
    {
        let _ = file.write_all(entry.as_bytes());
    }
}

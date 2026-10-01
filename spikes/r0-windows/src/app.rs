use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use tauri::{AppHandle, Emitter, Manager, State, WebviewWindow};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutEvent, ShortcutState};

use crate::config::{Config, FocusMode};
use crate::open_timer::OpenTimer;
use crate::win32::{self, Target};

const PANEL_LABEL: &str = "panel";
const PANEL_SHOWN_EVENT: &str = "panel-shown";
const LOG_FILE_NAME: &str = "lazyclipboard-r0-windows.log";
const CONFIG_ERROR_EXIT_CODE: i32 = 2;
const MODIFIER_RELEASE_TIMEOUT: Duration = Duration::from_millis(1000);
const FOCUS_RETURN_TIMEOUT: Duration = Duration::from_millis(500);
const FOCUS_RETURN_POLL_INTERVAL: Duration = Duration::from_millis(5);

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
            timer: OpenTimer::default(),
            target: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![panel_ready, panel_hide, paste])
        .setup(|app| {
            let spike = app.state::<Spike>();
            let panel = app
                .get_webview_window(PANEL_LABEL)
                .ok_or("panel window is missing")?;
            win32::set_panel_style(panel.hwnd()?.0 as isize, spike.config.focus_mode);
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
    spike.timer.shortcut_fired();
    let Some(panel) = app.get_webview_window(PANEL_LABEL) else {
        log("shortcut fired but the panel window is missing");
        return;
    };
    if let Err(error) = toggle_panel(&panel, &spike) {
        log(&format!("toggle failed: {error}"));
    }
}

fn toggle_panel(panel: &WebviewWindow, spike: &Spike) -> Result<(), String> {
    if panel.is_visible().map_err(|error| error.to_string())? {
        return panel.hide().map_err(|error| error.to_string());
    }
    open_panel(panel, spike)
}

fn open_panel(panel: &WebviewWindow, spike: &Spike) -> Result<(), String> {
    let target = win32::foreground_target();
    log(&format!(
        "shortcut target_hwnd={:?} blocked_by_uipi={:?}",
        target.map(|target| target.hwnd),
        target.map(|target| target.blocked_by_uipi),
    ));
    *spike.target() = target;
    let panel_hwnd = panel.hwnd().map_err(|error| error.to_string())?.0 as isize;
    panel.center().map_err(|error| error.to_string())?;
    match spike.config.focus_mode {
        FocusMode::Activate => {
            panel.show().map_err(|error| error.to_string())?;
            panel.set_focus().map_err(|error| error.to_string())?;
        }
        FocusMode::NoActivate => win32::show_without_activating(panel_hwnd)?,
    }
    panel
        .emit(PANEL_SHOWN_EVENT, ())
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn panel_ready(spike: State<Spike>) {
    let Some(elapsed) = spike.timer.first_frame_acked() else {
        log("first-frame ack without a pending shortcut");
        return;
    };
    let foreground_is_target = spike
        .target()
        .is_some_and(|target| target.hwnd == win32::foreground_hwnd());
    log(&format!(
        "open ready_ms={:.1} foreground_is_target={foreground_is_target} {}",
        elapsed.as_secs_f64() * 1000.0,
        spike.timer.summary(),
    ));
}

#[tauri::command]
fn panel_hide(panel: WebviewWindow, spike: State<Spike>) -> Result<(), String> {
    let target = spike.target().take();
    match target {
        Some(target) => hide_and_return_focus(&panel, &spike, target),
        None => panel.hide().map_err(|error| error.to_string()),
    }
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
    let target = (*spike.target()).ok_or("no target window was recorded")?;
    if target.blocked_by_uipi {
        return Err("target runs as administrator, paste is blocked by UIPI".to_owned());
    }
    if !win32::is_window(target.hwnd) {
        return Err("target window is gone".to_owned());
    }
    let snapshot = win32::snapshot_clipboard()?;
    log(&format!("snapshot {}", snapshot.describe()));
    let written_sequence = win32::write_text(text)?;
    spike.target().take();
    hide_and_return_focus(panel, spike, target)?;
    let modifiers_released = win32::wait_modifiers_released(MODIFIER_RELEASE_TIMEOUT);
    sleep(spike.config.focus_settle);
    win32::send_paste_chord(target.hwnd)?;
    log(&format!("pasted modifiers_released={modifiers_released}"));
    sleep(spike.config.restore_delay);
    if win32::clipboard_sequence() != written_sequence {
        log("clipboard changed by another process, not restoring");
        return Ok(());
    }
    win32::restore_clipboard(&snapshot)?;
    log("clipboard restored");
    Ok(())
}

fn hide_and_return_focus(
    panel: &WebviewWindow,
    spike: &Spike,
    target: Target,
) -> Result<(), String> {
    let hidden_at = Instant::now();
    panel.hide().map_err(|error| error.to_string())?;
    if spike.config.focus_mode == FocusMode::Activate && !win32::activate(target.hwnd) {
        log("SetForegroundWindow(target) was refused");
    }
    let returned = wait_for_foreground(target.hwnd);
    log(&format!(
        "hide focus_returned={returned} focus_return_ms={:.1}",
        hidden_at.elapsed().as_secs_f64() * 1000.0,
    ));
    Ok(())
}

fn wait_for_foreground(hwnd: isize) -> bool {
    let deadline = Instant::now() + FOCUS_RETURN_TIMEOUT;
    while win32::foreground_hwnd() != hwnd {
        if Instant::now() >= deadline {
            return false;
        }
        sleep(FOCUS_RETURN_POLL_INTERVAL);
    }
    true
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

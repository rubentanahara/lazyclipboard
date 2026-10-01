use std::fmt::Display;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

use arboard::Clipboard;
use ashpd::desktop::global_shortcuts::{BindShortcutsOptions, GlobalShortcuts, NewShortcut};
use ashpd::desktop::CreateSessionOptions;
use ashpd::{register_host_app, AppID};
use futures_util::StreamExt;
use tauri::{AppHandle, Emitter, Manager, State, WebviewWindow};

const APP_ID: &str = "dev.lazyclipboard.R0Wayland";
const SHORTCUT_ID: &str = "open-panel";
const SHORTCUT_DESCRIPTION: &str = "Open the R0 spike panel";
const PREFERRED_TRIGGER: &str = "CTRL+ALT+c";
const PANEL_LABEL: &str = "panel";
const SENTINEL_PREFIX: &str = "lazyclipboard-r0-sentinel-";

struct RunState {
    fired: AtomicU32,
    written: AtomicU32,
    clipboard: Mutex<Clipboard>,
}

fn sentinel(count: u32) -> String {
    format!("{SENTINEL_PREFIX}{count}")
}

fn log_line(message: impl Display) {
    eprintln!("[r0-wayland] {message}");
}

fn log_failure<T, E: Display>(step: &str, result: Result<T, E>) {
    if let Err(error) = result {
        log_line(format_args!("{step} failed: {error}"));
    }
}

fn open_panel(app: &AppHandle) {
    let state = app.state::<RunState>();
    let fired = state.fired.fetch_add(1, Ordering::SeqCst) + 1;
    log_line(format_args!("shortcut_fired n={fired}"));
    let Some(panel) = app.get_webview_window(PANEL_LABEL) else {
        return log_line("panel window missing");
    };
    log_failure("show", panel.show());
    log_failure("set_focus", panel.set_focus());
    log_failure("emit fired", panel.emit("fired", fired));
    log_line(format_args!("panel position={:?}", panel.outer_position()));
}

#[tauri::command]
fn paste_sentinel(state: State<RunState>, panel: WebviewWindow) -> Result<String, String> {
    let written = state.written.fetch_add(1, Ordering::SeqCst) + 1;
    let text = sentinel(written);
    state
        .clipboard
        .lock()
        .map_err(|error| error.to_string())?
        .set_text(text.clone())
        .map_err(|error| error.to_string())?;
    log_line(format_args!("sentinel_written n={written} text={text}"));
    log_failure("hide", panel.hide());
    Ok(text)
}

#[tauri::command]
fn hide_panel(panel: WebviewWindow) {
    log_failure("hide", panel.hide());
}

async fn listen_for_shortcut(app: AppHandle) -> ashpd::Result<()> {
    register_host_app(AppID::try_from(APP_ID)?).await?;
    let portal = GlobalShortcuts::new().await?;
    let mut activations = portal.receive_activated().await?;
    let session = portal
        .create_session(CreateSessionOptions::default())
        .await?;
    let shortcut =
        NewShortcut::new(SHORTCUT_ID, SHORTCUT_DESCRIPTION).preferred_trigger(PREFERRED_TRIGGER);
    let bound = portal
        .bind_shortcuts(&session, &[shortcut], None, BindShortcutsOptions::default())
        .await?
        .response()?;
    for shortcut in bound.shortcuts() {
        log_line(format_args!(
            "bound id={} trigger={}",
            shortcut.id(),
            shortcut.trigger_description()
        ));
    }
    while let Some(activation) = activations.next().await {
        if activation.shortcut_id() == SHORTCUT_ID {
            open_panel(&app);
        }
    }
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![paste_sentinel, hide_panel])
        .setup(|app| {
            app.manage(RunState {
                fired: AtomicU32::new(0),
                written: AtomicU32::new(0),
                clipboard: Mutex::new(Clipboard::new()?),
            });
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                log_failure("portal shortcut", listen_for_shortcut(handle).await);
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("the r0-wayland spike failed to run");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_sentinel_is_distinct_and_carries_its_count() {
        assert_eq!(sentinel(1), "lazyclipboard-r0-sentinel-1");
        assert_ne!(sentinel(1), sentinel(2));
    }
}

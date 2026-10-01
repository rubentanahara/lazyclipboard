use std::sync::Mutex;

use lazyclipboard_os::api::{Desktop, Flavours, TargetHandle};
use r0_linux::clipboard::SystemClipboard;
use r0_linux::paste::PasteSequence;
use r0_linux::timing::OpenTimer;
use r0_linux::x11::X11Desktop;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_global_shortcut::ShortcutState;

const OPEN_SHORTCUT: &str = "ctrl+alt+v";
const SENTINEL: &str = "lazyclipboard r0 sentinel";

struct SpikeState {
    timer: OpenTimer,
    target: Mutex<Option<TargetHandle>>,
    paste: Mutex<PasteSequence<X11Desktop, SystemClipboard>>,
}

fn open_panel(app: &AppHandle) {
    let state = app.state::<SpikeState>();
    state.timer.shortcut_fired();
    let paste = state.paste.lock().unwrap();
    match paste.desktop.frontmost_target() {
        Ok(target) => *state.target.lock().unwrap() = target,
        Err(error) => eprintln!("frontmost_target failed: {error}"),
    }
    if let Err(error) = paste.desktop.panel_show() {
        eprintln!("panel_show failed: {error}");
    }
}

#[tauri::command]
fn panel_ready(state: State<SpikeState>) {
    if let Some(elapsed) = state.timer.first_frame_acked() {
        eprintln!("open {}ms | {}", elapsed.as_millis(), state.timer.report());
    }
}

#[tauri::command]
fn panel_hide(state: State<SpikeState>) -> Result<(), String> {
    let paste = state.paste.lock().unwrap();
    paste
        .desktop
        .panel_hide()
        .map_err(|error| error.to_string())
}

#[tauri::command]
async fn paste_sentinel(state: State<'_, SpikeState>) -> Result<(), String> {
    let target = state
        .target
        .lock()
        .unwrap()
        .ok_or_else(|| "no target window was recorded".to_owned())?;
    let sentinel = Flavours {
        plain_text: Some(SENTINEL.to_owned()),
        ..Flavours::default()
    };
    let mut paste = state.paste.lock().unwrap();
    paste
        .run(target, &sentinel)
        .map_err(|error| error.to_string())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let desktop = X11Desktop::connect(app.handle().clone())?;
            app.manage(SpikeState {
                timer: OpenTimer::default(),
                target: Mutex::new(None),
                paste: Mutex::new(PasteSequence {
                    desktop,
                    clipboard: SystemClipboard::new()?,
                }),
            });
            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_shortcut(OPEN_SHORTCUT)?
                    .with_handler(|app, _shortcut, event| {
                        if event.state == ShortcutState::Pressed {
                            open_panel(app);
                        }
                    })
                    .build(),
            )?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            panel_ready,
            panel_hide,
            paste_sentinel
        ])
        .run(tauri::generate_context!())
        .expect("error while running the r0-linux spike");
}

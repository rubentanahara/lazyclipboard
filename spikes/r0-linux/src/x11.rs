use std::fmt::Display;
use std::thread;
use std::time::{Duration, Instant};

use lazyclipboard_core::model::CommandError;
use lazyclipboard_os::api::{AccessibilityStatus, Desktop, TargetHandle};
use tauri::{AppHandle, Emitter, Manager};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{
    Atom, AtomEnum, ClientMessageEvent, ConnectionExt, EventMask, Window, KEY_PRESS_EVENT,
    KEY_RELEASE_EVENT,
};
use x11rb::protocol::xtest::ConnectionExt as XTestConnectionExt;
use x11rb::rust_connection::RustConnection;
use x11rb::CURRENT_TIME;

use crate::chord::{paste_chord_for_window_class, PasteChord};
use crate::keymap::{
    keycode_for_keysym, modifiers_held, KEYSYM_CONTROL_LEFT, KEYSYM_LOWER_V, KEYSYM_SHIFT_LEFT,
};

pub const PANEL_LABEL: &str = "panel";
pub const PANEL_SHOWN_EVENT: &str = "panel-shown";
pub const FOCUS_SETTLE_DELAY: Duration = Duration::from_millis(50);
const MODIFIER_POLL_INTERVAL: Duration = Duration::from_millis(10);
const FOCUS_TIMEOUT: Duration = Duration::from_millis(500);
const FOCUS_POLL_INTERVAL: Duration = Duration::from_millis(10);
const PAGER_SOURCE_INDICATION: u32 = 2;
const WINDOW_CLASS_MAX_WORDS: u32 = 64;

pub struct X11Desktop {
    connection: RustConnection,
    root: Window,
    active_window_atom: Atom,
    app: AppHandle,
}

impl X11Desktop {
    pub fn connect(app: AppHandle) -> Result<Self, CommandError> {
        let (connection, screen_number) = x11rb::connect(None).map_err(log_internal)?;
        let root = connection.setup().roots[screen_number].root;
        let active_window_atom = connection
            .intern_atom(false, b"_NET_ACTIVE_WINDOW")
            .map_err(log_internal)?
            .reply()
            .map_err(log_internal)?
            .atom;
        Ok(Self {
            connection,
            root,
            active_window_atom,
            app,
        })
    }

    fn window_class(&self, window: Window) -> String {
        self.connection
            .get_property(
                false,
                window,
                AtomEnum::WM_CLASS,
                AtomEnum::STRING,
                0,
                WINDOW_CLASS_MAX_WORDS,
            )
            .ok()
            .and_then(|cookie| cookie.reply().ok())
            .map(|reply| String::from_utf8_lossy(&reply.value).replace('\0', " "))
            .unwrap_or_default()
    }

    fn pointer_mask(&self) -> Result<u16, CommandError> {
        let reply = self
            .connection
            .query_pointer(self.root)
            .map_err(log_internal)?
            .reply()
            .map_err(log_internal)?;
        Ok(u16::from(reply.mask))
    }

    fn activate(&self, window: Window) -> Result<(), CommandError> {
        let request = ClientMessageEvent::new(
            32,
            window,
            self.active_window_atom,
            [PAGER_SOURCE_INDICATION, CURRENT_TIME, 0, 0, 0],
        );
        self.connection
            .send_event(
                false,
                self.root,
                EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY,
                request,
            )
            .map_err(log_internal)?;
        self.connection.flush().map_err(log_internal)
    }

    fn wait_until_active(&self, target: TargetHandle) -> Result<(), CommandError> {
        let deadline = Instant::now() + FOCUS_TIMEOUT;
        while Instant::now() < deadline {
            if self.frontmost_target()? == Some(target) {
                return Ok(());
            }
            thread::sleep(FOCUS_POLL_INTERVAL);
        }
        eprintln!("target window {target:?} did not become active within {FOCUS_TIMEOUT:?}");
        Err(CommandError::Internal)
    }

    fn keycodes_for(&self, keysyms: &[u32]) -> Result<Vec<u8>, CommandError> {
        let setup = self.connection.setup();
        let first_keycode = setup.min_keycode;
        let keycode_count = setup.max_keycode - first_keycode + 1;
        let mapping = self
            .connection
            .get_keyboard_mapping(first_keycode, keycode_count)
            .map_err(log_internal)?
            .reply()
            .map_err(log_internal)?;
        keysyms
            .iter()
            .map(|&keysym| {
                keycode_for_keysym(
                    first_keycode,
                    mapping.keysyms_per_keycode,
                    &mapping.keysyms,
                    keysym,
                )
                .ok_or_else(|| log_internal(format!("no key produces keysym {keysym:#x}")))
            })
            .collect()
    }

    fn tap_together(&self, keycodes: &[u8]) -> Result<(), CommandError> {
        for &keycode in keycodes {
            self.fake_key(KEY_PRESS_EVENT, keycode)?;
        }
        for &keycode in keycodes.iter().rev() {
            self.fake_key(KEY_RELEASE_EVENT, keycode)?;
        }
        Ok(())
    }

    fn fake_key(&self, event_type: u8, keycode: u8) -> Result<(), CommandError> {
        self.connection
            .xtest_fake_input(event_type, keycode, CURRENT_TIME, self.root, 0, 0, 0)
            .map_err(log_internal)?
            .check()
            .map_err(log_internal)
    }

    pub fn panel_is_visible(&self) -> bool {
        self.panel()
            .and_then(|panel| panel.is_visible().map_err(log_internal))
            .unwrap_or(false)
    }

    fn panel(&self) -> Result<tauri::WebviewWindow, CommandError> {
        self.app
            .get_webview_window(PANEL_LABEL)
            .ok_or(CommandError::NotFound)
    }
}

impl Desktop for X11Desktop {
    fn frontmost_target(&self) -> Result<Option<TargetHandle>, CommandError> {
        let reply = self
            .connection
            .get_property(
                false,
                self.root,
                self.active_window_atom,
                AtomEnum::WINDOW,
                0,
                1,
            )
            .map_err(log_internal)?
            .reply()
            .map_err(log_internal)?;
        Ok(reply
            .value32()
            .and_then(|mut windows| windows.next())
            .filter(|&window| window != 0)
            .map(|window| TargetHandle(u64::from(window))))
    }

    fn wait_modifiers_released(&self, timeout: Duration) -> Result<(), CommandError> {
        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            if !modifiers_held(self.pointer_mask()?) {
                return Ok(());
            }
            thread::sleep(MODIFIER_POLL_INTERVAL);
        }
        eprintln!("modifiers still held after {timeout:?}");
        Err(CommandError::Internal)
    }

    fn send_copy_chord(&self) -> Result<(), CommandError> {
        Err(CommandError::Unsupported)
    }

    fn send_paste_chord(&self, target: TargetHandle) -> Result<(), CommandError> {
        let window = Window::try_from(target.0).map_err(log_internal)?;
        let chord = paste_chord_for_window_class(&self.window_class(window));
        self.activate(window)?;
        self.wait_until_active(target)?;
        thread::sleep(FOCUS_SETTLE_DELAY);
        let keysyms: &[u32] = match chord {
            PasteChord::ControlV => &[KEYSYM_CONTROL_LEFT, KEYSYM_LOWER_V],
            PasteChord::ControlShiftV => &[KEYSYM_CONTROL_LEFT, KEYSYM_SHIFT_LEFT, KEYSYM_LOWER_V],
        };
        self.tap_together(&self.keycodes_for(keysyms)?)
    }

    fn panel_show(&self) -> Result<(), CommandError> {
        let panel = self.panel()?;
        panel.center().map_err(log_internal)?;
        panel.show().map_err(log_internal)?;
        panel.set_focus().map_err(log_internal)?;
        panel.emit(PANEL_SHOWN_EVENT, ()).map_err(log_internal)
    }

    fn panel_hide(&self) -> Result<(), CommandError> {
        self.panel()?.hide().map_err(log_internal)
    }

    fn accessibility_status(&self) -> AccessibilityStatus {
        AccessibilityStatus::NotRequired
    }
}

fn log_internal(error: impl Display) -> CommandError {
    eprintln!("x11: {error}");
    CommandError::Internal
}

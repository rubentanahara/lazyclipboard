use std::ffi::c_void;
use std::iter::once;
use std::mem::size_of;
use std::ops::RangeInclusive;
use std::thread::sleep;
use std::time::{Duration, Instant};

use windows::Win32::Foundation::{CloseHandle, GlobalFree, HANDLE, HGLOBAL, HWND};
use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
use windows::Win32::System::DataExchange::{
    CloseClipboard, EmptyClipboard, EnumClipboardFormats, GetClipboardData,
    GetClipboardSequenceNumber, OpenClipboard, SetClipboardData,
};
use windows::Win32::System::Memory::{
    GlobalAlloc, GlobalLock, GlobalSize, GlobalUnlock, GMEM_MOVEABLE,
};
use windows::Win32::System::Ole::{
    CF_BITMAP, CF_DSPBITMAP, CF_DSPENHMETAFILE, CF_DSPMETAFILEPICT, CF_ENHMETAFILE,
    CF_METAFILEPICT, CF_OWNERDISPLAY, CF_PALETTE, CF_UNICODETEXT,
};
use windows::Win32::System::Threading::{
    GetCurrentProcess, OpenProcess, OpenProcessToken, PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, GetKeyboardLayout, MapVirtualKeyExW, SendInput, HKL, INPUT, INPUT_0,
    INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, MAPVK_VK_TO_VSC, VIRTUAL_KEY,
    VK_CONTROL, VK_LWIN, VK_MENU, VK_RWIN, VK_SHIFT, VK_V,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowLongPtrW, GetWindowThreadProcessId, IsWindow,
    SetForegroundWindow, SetWindowLongPtrW, SetWindowPos, ShowWindow, GWL_EXSTYLE, HWND_TOPMOST,
    SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SW_SHOWNOACTIVATE, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW,
};

use crate::config::FocusMode;

const CLIPBOARD_OPEN_ATTEMPTS: u32 = 20;
const CLIPBOARD_OPEN_RETRY_INTERVAL: Duration = Duration::from_millis(10);
const MODIFIER_POLL_INTERVAL: Duration = Duration::from_millis(10);
const MODIFIER_KEYS: [VIRTUAL_KEY; 5] = [VK_SHIFT, VK_CONTROL, VK_MENU, VK_LWIN, VK_RWIN];
const GDI_OBJECT_FORMATS: RangeInclusive<u32> = 0x0300..=0x03FF;

#[derive(Debug, Clone, Copy)]
pub struct Target {
    pub hwnd: isize,
    pub blocked_by_uipi: bool,
}

pub struct ClipboardSnapshot {
    formats: Vec<(u32, Vec<u8>)>,
    skipped: Vec<u32>,
}

impl ClipboardSnapshot {
    pub fn describe(&self) -> String {
        let captured: Vec<(u32, usize)> = self
            .formats
            .iter()
            .map(|(format, bytes)| (*format, bytes.len()))
            .collect();
        format!("formats(id,bytes)={captured:?} skipped={:?}", self.skipped)
    }
}

pub fn foreground_target() -> Option<Target> {
    let hwnd = unsafe { GetForegroundWindow() };
    if hwnd.0.is_null() {
        return None;
    }
    Some(Target {
        hwnd: hwnd.0 as isize,
        blocked_by_uipi: is_blocked_by_uipi(hwnd),
    })
}

pub fn foreground_hwnd() -> isize {
    unsafe { GetForegroundWindow() }.0 as isize
}

pub fn is_window(hwnd: isize) -> bool {
    unsafe { IsWindow(Some(hwnd_of(hwnd))) }.as_bool()
}

pub fn activate(hwnd: isize) -> bool {
    unsafe { SetForegroundWindow(hwnd_of(hwnd)) }.as_bool()
}

pub fn set_panel_style(hwnd: isize, focus_mode: FocusMode) {
    let hwnd = hwnd_of(hwnd);
    let mut style = unsafe { GetWindowLongPtrW(hwnd, GWL_EXSTYLE) } | WS_EX_TOOLWINDOW.0 as isize;
    if focus_mode == FocusMode::NoActivate {
        style |= WS_EX_NOACTIVATE.0 as isize;
    }
    unsafe { SetWindowLongPtrW(hwnd, GWL_EXSTYLE, style) };
}

pub fn show_without_activating(hwnd: isize) -> Result<(), String> {
    let hwnd = hwnd_of(hwnd);
    let _ = unsafe { ShowWindow(hwnd, SW_SHOWNOACTIVATE) };
    unsafe {
        SetWindowPos(
            hwnd,
            Some(HWND_TOPMOST),
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
        )
    }
    .map_err(|error| error.to_string())
}

pub fn wait_modifiers_released(timeout: Duration) -> bool {
    let deadline = Instant::now() + timeout;
    while any_modifier_down() {
        if Instant::now() >= deadline {
            return false;
        }
        sleep(MODIFIER_POLL_INTERVAL);
    }
    true
}

pub fn send_paste_chord(target_hwnd: isize) -> Result<(), String> {
    let thread = unsafe { GetWindowThreadProcessId(hwnd_of(target_hwnd), None) };
    let layout = unsafe { GetKeyboardLayout(thread) };
    let inputs = [
        keyboard_input(VK_CONTROL, layout, KEYBD_EVENT_FLAGS(0)),
        keyboard_input(VK_V, layout, KEYBD_EVENT_FLAGS(0)),
        keyboard_input(VK_V, layout, KEYEVENTF_KEYUP),
        keyboard_input(VK_CONTROL, layout, KEYEVENTF_KEYUP),
    ];
    let injected = unsafe { SendInput(&inputs, size_of::<INPUT>() as i32) } as usize;
    if injected != inputs.len() {
        return Err(format!(
            "SendInput injected {injected} of {} events",
            inputs.len()
        ));
    }
    Ok(())
}

pub fn clipboard_sequence() -> u32 {
    unsafe { GetClipboardSequenceNumber() }
}

pub fn snapshot_clipboard() -> Result<ClipboardSnapshot, String> {
    let _open = OpenClipboardGuard::acquire()?;
    let mut formats = Vec::new();
    let mut skipped = Vec::new();
    let mut format = unsafe { EnumClipboardFormats(0) };
    while format != 0 {
        if holds_global_memory(format) {
            match read_format(format) {
                Some(bytes) => formats.push((format, bytes)),
                None => skipped.push(format),
            }
        }
        format = unsafe { EnumClipboardFormats(format) };
    }
    Ok(ClipboardSnapshot { formats, skipped })
}

pub fn write_text(text: &str) -> Result<u32, String> {
    {
        let _open = OpenClipboardGuard::acquire()?;
        unsafe { EmptyClipboard() }.map_err(|error| error.to_string())?;
        let utf16_with_terminator: Vec<u8> = text
            .encode_utf16()
            .chain(once(0))
            .flat_map(u16::to_le_bytes)
            .collect();
        put_format(CF_UNICODETEXT.0 as u32, &utf16_with_terminator)?;
    }
    Ok(clipboard_sequence())
}

pub fn restore_clipboard(snapshot: &ClipboardSnapshot) -> Result<(), String> {
    let _open = OpenClipboardGuard::acquire()?;
    unsafe { EmptyClipboard() }.map_err(|error| error.to_string())?;
    let failures: Vec<String> = snapshot
        .formats
        .iter()
        .filter_map(|(format, bytes)| {
            put_format(*format, bytes)
                .err()
                .map(|error| format!("{format}: {error}"))
        })
        .collect();
    if failures.is_empty() {
        return Ok(());
    }
    Err(format!("restore failed for {}", failures.join("; ")))
}

fn hwnd_of(raw: isize) -> HWND {
    HWND(raw as *mut c_void)
}

fn is_blocked_by_uipi(hwnd: HWND) -> bool {
    let mut process_id = 0u32;
    unsafe { GetWindowThreadProcessId(hwnd, Some(&mut process_id)) };
    let Ok(process) =
        (unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id) })
    else {
        return true;
    };
    let target_elevated = process_is_elevated(process);
    let _ = unsafe { CloseHandle(process) };
    target_elevated && !process_is_elevated(unsafe { GetCurrentProcess() })
}

fn process_is_elevated(process: HANDLE) -> bool {
    let mut token = HANDLE::default();
    if unsafe { OpenProcessToken(process, TOKEN_QUERY, &mut token) }.is_err() {
        return true;
    }
    let mut elevation = TOKEN_ELEVATION::default();
    let mut returned = 0u32;
    let queried = unsafe {
        GetTokenInformation(
            token,
            TokenElevation,
            Some(&mut elevation as *mut TOKEN_ELEVATION as *mut c_void),
            size_of::<TOKEN_ELEVATION>() as u32,
            &mut returned,
        )
    };
    let _ = unsafe { CloseHandle(token) };
    queried.is_err() || elevation.TokenIsElevated != 0
}

fn any_modifier_down() -> bool {
    MODIFIER_KEYS
        .iter()
        .any(|key| unsafe { GetAsyncKeyState(key.0 as i32) } < 0)
}

fn keyboard_input(key: VIRTUAL_KEY, layout: HKL, flags: KEYBD_EVENT_FLAGS) -> INPUT {
    let scan_code = unsafe { MapVirtualKeyExW(key.0 as u32, MAPVK_VK_TO_VSC, Some(layout)) };
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: key,
                wScan: scan_code as u16,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

fn holds_global_memory(format: u32) -> bool {
    let handle_formats = [
        CF_BITMAP.0,
        CF_METAFILEPICT.0,
        CF_PALETTE.0,
        CF_ENHMETAFILE.0,
        CF_OWNERDISPLAY.0,
        CF_DSPBITMAP.0,
        CF_DSPMETAFILEPICT.0,
        CF_DSPENHMETAFILE.0,
    ]
    .map(u32::from);
    !handle_formats.contains(&format) && !GDI_OBJECT_FORMATS.contains(&format)
}

fn read_format(format: u32) -> Option<Vec<u8>> {
    let handle = unsafe { GetClipboardData(format) }.ok()?;
    let memory = HGLOBAL(handle.0);
    let size = unsafe { GlobalSize(memory) };
    let source = unsafe { GlobalLock(memory) };
    if source.is_null() {
        return None;
    }
    let bytes = unsafe { std::slice::from_raw_parts(source as *const u8, size) }.to_vec();
    let _ = unsafe { GlobalUnlock(memory) };
    Some(bytes)
}

fn put_format(format: u32, bytes: &[u8]) -> Result<(), String> {
    let memory =
        unsafe { GlobalAlloc(GMEM_MOVEABLE, bytes.len()) }.map_err(|error| error.to_string())?;
    let destination = unsafe { GlobalLock(memory) };
    if destination.is_null() {
        let _ = unsafe { GlobalFree(Some(memory)) };
        return Err("GlobalLock returned null".to_owned());
    }
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), destination as *mut u8, bytes.len());
        let _ = GlobalUnlock(memory);
    }
    if let Err(error) = unsafe { SetClipboardData(format, Some(HANDLE(memory.0))) } {
        let _ = unsafe { GlobalFree(Some(memory)) };
        return Err(error.to_string());
    }
    Ok(())
}

struct OpenClipboardGuard;

impl OpenClipboardGuard {
    fn acquire() -> Result<Self, String> {
        for _ in 0..CLIPBOARD_OPEN_ATTEMPTS {
            if unsafe { OpenClipboard(None) }.is_ok() {
                return Ok(Self);
            }
            sleep(CLIPBOARD_OPEN_RETRY_INTERVAL);
        }
        Err("clipboard is held by another process".to_owned())
    }
}

impl Drop for OpenClipboardGuard {
    fn drop(&mut self) {
        let _ = unsafe { CloseClipboard() };
    }
}

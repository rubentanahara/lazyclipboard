mod fake;

use std::time::Duration;

use lazyclipboard_core::model::CommandError;

pub use fake::FakeClipboard;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Flavours {
    pub plain_text: Option<String>,
    pub html: Option<String>,
    pub png: Option<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipboardSnapshot {
    pub flavours: Flavours,
    pub change_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TargetHandle(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessibilityStatus {
    Granted,
    Missing,
    NotRequired,
}

pub trait Clipboard {
    fn snapshot(&mut self) -> Result<ClipboardSnapshot, CommandError>;
    fn restore(&mut self, snapshot: ClipboardSnapshot) -> Result<(), CommandError>;
    fn change_count(&self) -> u64;
    fn read_flavours(&self) -> Result<Flavours, CommandError>;
    fn write_item(&mut self, flavours: &Flavours) -> Result<(), CommandError>;
}

pub trait Desktop {
    fn frontmost_target(&self) -> Result<Option<TargetHandle>, CommandError>;
    fn wait_modifiers_released(&self, timeout: Duration) -> Result<(), CommandError>;
    fn send_copy_chord(&self) -> Result<(), CommandError>;
    fn send_paste_chord(&self, target: TargetHandle) -> Result<(), CommandError>;
    fn panel_show(&self) -> Result<(), CommandError>;
    fn panel_hide(&self) -> Result<(), CommandError>;
    fn accessibility_status(&self) -> AccessibilityStatus;
}

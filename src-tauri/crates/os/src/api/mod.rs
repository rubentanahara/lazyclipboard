mod fake;

use std::fmt;
use std::time::Duration;

use lazyclipboard_core::model::CommandError;

pub use fake::FakeClipboard;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Flavours {
    pub plain_text: Option<String>,
    pub html: Option<String>,
    pub png: Option<Vec<u8>>,
}

impl fmt::Debug for Flavours {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Flavours")
            .field(
                "plain_text_bytes",
                &self.plain_text.as_ref().map(String::len),
            )
            .field("html_bytes", &self.html.as_ref().map(String::len))
            .field("png_bytes", &self.png.as_ref().map(Vec::len))
            .finish()
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_output_never_contains_clipboard_content() {
        let flavours = Flavours {
            plain_text: Some("hunter2-sentinel".to_owned()),
            html: Some("<b>hunter2-sentinel</b>".to_owned()),
            png: Some(vec![1, 2, 3]),
        };
        let snapshot = ClipboardSnapshot {
            flavours: flavours.clone(),
            change_count: 7,
        };

        for rendered in [format!("{flavours:?}"), format!("{snapshot:?}")] {
            assert!(!rendered.contains("hunter2"), "leaked: {rendered}");
        }
    }
}

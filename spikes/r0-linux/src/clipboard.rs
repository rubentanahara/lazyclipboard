use std::fmt::Display;

use clipboard_rs::{Clipboard as _, ClipboardContent, ClipboardContext};
use lazyclipboard_core::model::CommandError;
use lazyclipboard_os::api::{Clipboard, ClipboardSnapshot, Flavours};

const PNG_FORMAT: &str = "image/png";

pub struct SystemClipboard {
    context: ClipboardContext,
    writes: u64,
}

impl SystemClipboard {
    pub fn new() -> Result<Self, CommandError> {
        Ok(Self {
            context: ClipboardContext::new().map_err(log_internal)?,
            writes: 0,
        })
    }
}

impl Clipboard for SystemClipboard {
    fn snapshot(&mut self) -> Result<ClipboardSnapshot, CommandError> {
        Ok(ClipboardSnapshot {
            flavours: self.read_flavours()?,
            change_count: self.change_count(),
        })
    }

    fn restore(&mut self, snapshot: ClipboardSnapshot) -> Result<(), CommandError> {
        self.write_item(&snapshot.flavours)
    }

    fn change_count(&self) -> u64 {
        self.writes
    }

    fn read_flavours(&self) -> Result<Flavours, CommandError> {
        Ok(Flavours {
            plain_text: self.context.get_text().ok().filter(|text| !text.is_empty()),
            html: self.context.get_html().ok().filter(|html| !html.is_empty()),
            png: self
                .context
                .get_buffer(PNG_FORMAT)
                .ok()
                .filter(|bytes| !bytes.is_empty()),
        })
    }

    fn write_item(&mut self, flavours: &Flavours) -> Result<(), CommandError> {
        let contents = clipboard_contents(flavours);
        if contents.is_empty() {
            self.context.clear().map_err(log_internal)?;
        } else {
            self.context.set(contents).map_err(log_internal)?;
        }
        self.writes += 1;
        Ok(())
    }
}

fn clipboard_contents(flavours: &Flavours) -> Vec<ClipboardContent> {
    let text = flavours.plain_text.clone().map(ClipboardContent::Text);
    let html = flavours.html.clone().map(ClipboardContent::Html);
    let png = flavours
        .png
        .clone()
        .map(|bytes| ClipboardContent::Other(PNG_FORMAT.to_owned(), bytes));
    [text, html, png].into_iter().flatten().collect()
}

fn log_internal(error: impl Display) -> CommandError {
    eprintln!("clipboard: {error}");
    CommandError::Internal
}

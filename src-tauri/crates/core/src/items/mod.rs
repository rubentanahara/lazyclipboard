use std::fmt;

mod classify;
mod image;
mod sanitise;
mod save;

#[cfg(test)]
mod tests;

pub use classify::classify;
pub use save::{save, SaveRequest};

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Flavours {
    pub plain_text: Option<String>,
    pub html: Option<String>,
    pub png: Option<Vec<u8>>,
}

#[derive(Clone, PartialEq, Eq)]
pub enum PendingItem {
    Text {
        text: String,
    },
    RichText {
        plain_text: String,
        html: String,
    },
    Link {
        url: String,
    },
    Image {
        png: Vec<u8>,
        width: u32,
        height: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Classified {
    pub item: Option<PendingItem>,
    pub skipped_images: u32,
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

impl fmt::Debug for PendingItem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text { text } => formatter
                .debug_struct("Text")
                .field("text_bytes", &text.len())
                .finish(),
            Self::RichText { plain_text, html } => formatter
                .debug_struct("RichText")
                .field("plain_text_bytes", &plain_text.len())
                .field("html_bytes", &html.len())
                .finish(),
            Self::Link { url } => formatter
                .debug_struct("Link")
                .field("url_bytes", &url.len())
                .finish(),
            Self::Image { png, width, height } => formatter
                .debug_struct("Image")
                .field("png_bytes", &png.len())
                .field("width", width)
                .field("height", height)
                .finish(),
        }
    }
}

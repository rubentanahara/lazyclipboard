mod classify;
mod image;
mod sanitise;
mod save;

#[cfg(test)]
mod tests;

pub use classify::classify;
pub use save::{save, SaveRequest};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RgbaImage {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Flavours {
    pub plain_text: Option<String>,
    pub html: Option<String>,
    pub image: Option<RgbaImage>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

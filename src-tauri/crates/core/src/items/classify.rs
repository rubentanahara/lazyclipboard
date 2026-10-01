use super::image::normalise_png;
use super::sanitise::sanitise;
use super::{Classified, Flavours, PendingItem};
use crate::model::CommandError;

const MAX_PLAIN_TEXT_BYTES: usize = 1024 * 1024;
const MAX_HTML_BYTES: usize = 2 * 1024 * 1024;
const MAX_PNG_BYTES: usize = 10 * 1024 * 1024;
const LINK_SCHEMES: [&str; 2] = ["http://", "https://"];

pub fn classify(flavours: Flavours) -> Result<Classified, CommandError> {
    let mut skipped_images = 0;
    if let Some(source) = &flavours.png {
        match image_item(source) {
            Some(item) => {
                return Ok(Classified {
                    item: Some(item),
                    skipped_images,
                })
            }
            None => skipped_images += 1,
        }
    }
    match text_item(flavours.plain_text, flavours.html) {
        Ok(item) => Ok(Classified {
            item: Some(item),
            skipped_images,
        }),
        Err(CommandError::Unsupported) if skipped_images > 0 => Ok(Classified {
            item: None,
            skipped_images,
        }),
        Err(error) => Err(error),
    }
}

fn image_item(source: &[u8]) -> Option<PendingItem> {
    let image = normalise_png(source).filter(|image| image.png.len() <= MAX_PNG_BYTES)?;
    Some(PendingItem::Image {
        png: image.png,
        width: image.width,
        height: image.height,
    })
}

fn text_item(
    plain_text: Option<String>,
    html: Option<String>,
) -> Result<PendingItem, CommandError> {
    let text = plain_text
        .filter(|text| !text.trim().is_empty())
        .ok_or(CommandError::Unsupported)?;
    if text.len() > MAX_PLAIN_TEXT_BYTES {
        return Err(CommandError::TooLarge);
    }
    let sanitised = html
        .filter(|html| html.len() <= MAX_HTML_BYTES)
        .map(|html| sanitise(&html))
        .filter(|sanitised| sanitised.html.len() <= MAX_HTML_BYTES);
    if let Some(sanitised) = sanitised.filter(|sanitised| sanitised.has_formatting) {
        return Ok(PendingItem::RichText {
            plain_text: text,
            html: sanitised.html,
        });
    }
    if is_link(text.trim()) {
        return Ok(PendingItem::Link {
            url: text.trim().to_owned(),
        });
    }
    Ok(PendingItem::Text { text })
}

fn is_link(text: &str) -> bool {
    LINK_SCHEMES.iter().any(|scheme| {
        text.strip_prefix(scheme)
            .is_some_and(|rest| !rest.is_empty() && !rest.contains(char::is_whitespace))
    })
}

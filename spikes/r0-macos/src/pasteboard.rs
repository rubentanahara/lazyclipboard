use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_app_kit::{
    NSPasteboard, NSPasteboardContentsOptions, NSPasteboardItem, NSPasteboardTypeString,
    NSPasteboardWriting,
};
use objc2_foundation::{NSArray, NSData, NSString};

const TRANSIENT_TYPE: &str = "org.nspasteboard.TransientType";

pub struct Snapshot {
    items: Vec<Vec<(Retained<NSString>, Vec<u8>)>>,
}

pub fn change_count() -> isize {
    NSPasteboard::generalPasteboard().changeCount()
}

pub fn snapshot() -> Snapshot {
    let pasteboard = NSPasteboard::generalPasteboard();
    let items = pasteboard
        .pasteboardItems()
        .map(|items| items.iter().map(|item| flavours_of(&item)).collect())
        .unwrap_or_default();
    Snapshot { items }
}

pub fn write_transient_text(text: &str) -> isize {
    let item = NSPasteboardItem::new();
    unsafe {
        item.setString_forType(&NSString::from_str(text), NSPasteboardTypeString);
        item.setData_forType(&NSData::new(), &NSString::from_str(TRANSIENT_TYPE));
    }
    write_items(&[item])
}

pub struct RestoreOnDrop {
    prior: Option<Snapshot>,
    change_count_after_our_write: isize,
}

impl RestoreOnDrop {
    pub fn new(prior: Snapshot, change_count_after_our_write: isize) -> Self {
        Self {
            prior: Some(prior),
            change_count_after_our_write,
        }
    }

    pub fn restore_now(&mut self) -> bool {
        let Some(prior) = self.prior.take() else {
            return false;
        };
        let untouched = should_restore(self.change_count_after_our_write, change_count());
        if untouched {
            restore(&prior);
        }
        untouched
    }
}

impl Drop for RestoreOnDrop {
    fn drop(&mut self) {
        self.restore_now();
    }
}

fn restore(snapshot: &Snapshot) -> isize {
    let items: Vec<_> = snapshot
        .items
        .iter()
        .map(|flavours| {
            let item = NSPasteboardItem::new();
            for (data_type, bytes) in flavours {
                item.setData_forType(&NSData::with_bytes(bytes), data_type);
            }
            item
        })
        .collect();
    write_items(&items)
}

pub fn should_restore(change_count_after_our_write: isize, change_count_now: isize) -> bool {
    change_count_after_our_write == change_count_now
}

fn flavours_of(item: &NSPasteboardItem) -> Vec<(Retained<NSString>, Vec<u8>)> {
    item.types()
        .iter()
        .filter_map(|data_type| {
            let data = item.dataForType(&data_type)?;
            Some((data_type, data.to_vec()))
        })
        .collect()
}

fn write_items(items: &[Retained<NSPasteboardItem>]) -> isize {
    let pasteboard = NSPasteboard::generalPasteboard();
    let writers: Vec<_> = items
        .iter()
        .map(|item| ProtocolObject::<dyn NSPasteboardWriting>::from_retained(item.clone()))
        .collect();
    pasteboard.prepareForNewContentsWithOptions(NSPasteboardContentsOptions::CurrentHostOnly);
    pasteboard.writeObjects(&NSArray::from_retained_slice(&writers));
    pasteboard.changeCount()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restore_is_skipped_when_someone_else_wrote_after_us() {
        assert!(should_restore(7, 7));
        assert!(!should_restore(7, 8));
    }
}

use lazyclipboard_core::model::CommandError;

use super::{Clipboard, ClipboardSnapshot, Flavours};

#[derive(Debug, Default)]
pub struct FakeClipboard {
    flavours: Flavours,
    change_count: u64,
}

impl Clipboard for FakeClipboard {
    fn snapshot(&mut self) -> Result<ClipboardSnapshot, CommandError> {
        Ok(ClipboardSnapshot {
            flavours: self.flavours.clone(),
            change_count: self.change_count,
        })
    }

    fn restore(&mut self, snapshot: ClipboardSnapshot) -> Result<(), CommandError> {
        self.write_item(&snapshot.flavours)
    }

    fn change_count(&self) -> u64 {
        self.change_count
    }

    fn read_flavours(&self) -> Result<Flavours, CommandError> {
        Ok(self.flavours.clone())
    }

    fn write_item(&mut self, flavours: &Flavours) -> Result<(), CommandError> {
        self.flavours = flavours.clone();
        self.change_count += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(value: &str) -> Flavours {
        Flavours {
            plain_text: Some(value.to_owned()),
            ..Flavours::default()
        }
    }

    fn rich(plain: &str, html: &str) -> Flavours {
        Flavours {
            plain_text: Some(plain.to_owned()),
            html: Some(html.to_owned()),
            png: Some(vec![0x89, b'P', b'N', b'G']),
        }
    }

    #[test]
    fn restore_brings_back_the_content_held_before_the_snapshot() {
        let mut clipboard = FakeClipboard::default();
        let prior = rich("sentinel", "<b>sentinel</b>");
        clipboard.write_item(&prior).unwrap();

        let snapshot = clipboard.snapshot().unwrap();
        clipboard.write_item(&text("captured selection")).unwrap();
        clipboard.restore(snapshot).unwrap();

        assert_eq!(clipboard.read_flavours().unwrap(), prior);
    }

    #[test]
    fn restore_clears_flavours_the_snapshot_did_not_hold() {
        let mut clipboard = FakeClipboard::default();
        clipboard.write_item(&text("sentinel")).unwrap();

        let snapshot = clipboard.snapshot().unwrap();
        clipboard
            .write_item(&rich("other", "<i>other</i>"))
            .unwrap();
        clipboard.restore(snapshot).unwrap();

        assert_eq!(clipboard.read_flavours().unwrap(), text("sentinel"));
    }

    #[test]
    fn every_write_advances_the_change_count() {
        let mut clipboard = FakeClipboard::default();
        let before = clipboard.change_count();

        clipboard.write_item(&text("one")).unwrap();
        clipboard.write_item(&text("two")).unwrap();

        assert_eq!(clipboard.change_count(), before + 2);
    }

    #[test]
    fn restore_counts_as_a_change_like_a_real_clipboard() {
        let mut clipboard = FakeClipboard::default();
        let snapshot = clipboard.snapshot().unwrap();
        let before = clipboard.change_count();

        clipboard.restore(snapshot).unwrap();

        assert_eq!(clipboard.change_count(), before + 1);
    }

    #[test]
    fn snapshot_leaves_the_clipboard_untouched() {
        let mut clipboard = FakeClipboard::default();
        clipboard.write_item(&text("sentinel")).unwrap();
        let before = clipboard.change_count();

        clipboard.snapshot().unwrap();

        assert_eq!(clipboard.change_count(), before);
        assert_eq!(clipboard.read_flavours().unwrap(), text("sentinel"));
    }
}

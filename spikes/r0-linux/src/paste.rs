use std::thread;
use std::time::Duration;

use lazyclipboard_core::model::CommandError;
use lazyclipboard_os::api::{Clipboard, ClipboardSnapshot, Desktop, Flavours, TargetHandle};

pub const MODIFIER_RELEASE_TIMEOUT: Duration = Duration::from_millis(1000);
pub const RESTORE_DELAY: Duration = Duration::from_millis(250);

pub struct PasteSequence<D, C> {
    pub desktop: D,
    pub clipboard: C,
}

impl<D: Desktop, C: Clipboard> PasteSequence<D, C> {
    pub fn run(&mut self, target: TargetHandle, item: &Flavours) -> Result<(), CommandError> {
        let prior = self.clipboard.snapshot()?;
        let delivered = self.deliver(target, item);
        thread::sleep(RESTORE_DELAY);
        let restored = self.restore_unless_replaced(prior, item);
        delivered.and(restored)
    }

    fn restore_unless_replaced(
        &mut self,
        prior: ClipboardSnapshot,
        item: &Flavours,
    ) -> Result<(), CommandError> {
        match self.clipboard.read_flavours() {
            Ok(current) if current == *item => self.clipboard.restore(prior),
            _ => Ok(()),
        }
    }

    fn deliver(&mut self, target: TargetHandle, item: &Flavours) -> Result<(), CommandError> {
        self.clipboard.write_item(item)?;
        self.desktop.panel_hide()?;
        self.desktop
            .wait_modifiers_released(MODIFIER_RELEASE_TIMEOUT)?;
        self.desktop.send_paste_chord(target)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::time::Duration;

    use lazyclipboard_core::model::CommandError;
    use lazyclipboard_os::api::{
        AccessibilityStatus, Clipboard, ClipboardSnapshot, Desktop, FakeClipboard, Flavours,
        TargetHandle,
    };

    use super::*;

    type Log = Rc<RefCell<Vec<&'static str>>>;

    const TARGET: TargetHandle = TargetHandle(0x3a0_0007);
    const PNG_SIGNATURE: [u8; 4] = [0x89, b'P', b'N', b'G'];

    type SharedClipboard = Rc<RefCell<FakeClipboard>>;

    struct SpyDesktop {
        log: Log,
        chord_fails: bool,
        modifiers_stuck: bool,
        copy_during_paste: Option<(SharedClipboard, Flavours)>,
    }

    impl Desktop for SpyDesktop {
        fn frontmost_target(&self) -> Result<Option<TargetHandle>, CommandError> {
            Ok(Some(TARGET))
        }

        fn wait_modifiers_released(&self, _timeout: Duration) -> Result<(), CommandError> {
            self.log.borrow_mut().push("wait_modifiers_released");
            if self.modifiers_stuck {
                return Err(CommandError::Internal);
            }
            Ok(())
        }

        fn send_copy_chord(&self) -> Result<(), CommandError> {
            Err(CommandError::Unsupported)
        }

        fn send_paste_chord(&self, target: TargetHandle) -> Result<(), CommandError> {
            assert_eq!(target, TARGET);
            self.log.borrow_mut().push("send_paste_chord");
            if let Some((clipboard, copied)) = &self.copy_during_paste {
                clipboard.borrow_mut().write_item(copied).unwrap();
            }
            if self.chord_fails {
                return Err(CommandError::Internal);
            }
            Ok(())
        }

        fn panel_show(&self) -> Result<(), CommandError> {
            Ok(())
        }

        fn panel_hide(&self) -> Result<(), CommandError> {
            self.log.borrow_mut().push("panel_hide");
            Ok(())
        }

        fn accessibility_status(&self) -> AccessibilityStatus {
            AccessibilityStatus::NotRequired
        }
    }

    struct SpyClipboard {
        log: Log,
        inner: SharedClipboard,
    }

    impl Clipboard for SpyClipboard {
        fn snapshot(&mut self) -> Result<ClipboardSnapshot, CommandError> {
            self.log.borrow_mut().push("snapshot");
            self.inner.borrow_mut().snapshot()
        }

        fn restore(&mut self, snapshot: ClipboardSnapshot) -> Result<(), CommandError> {
            self.log.borrow_mut().push("restore");
            self.inner.borrow_mut().restore(snapshot)
        }

        fn change_count(&self) -> u64 {
            self.inner.borrow().change_count()
        }

        fn read_flavours(&self) -> Result<Flavours, CommandError> {
            self.inner.borrow().read_flavours()
        }

        fn write_item(&mut self, flavours: &Flavours) -> Result<(), CommandError> {
            self.log.borrow_mut().push("write_item");
            self.inner.borrow_mut().write_item(flavours)
        }
    }

    fn text(value: &str) -> Flavours {
        Flavours {
            plain_text: Some(value.to_owned()),
            ..Flavours::default()
        }
    }

    fn image() -> Flavours {
        Flavours {
            png: Some(PNG_SIGNATURE.to_vec()),
            ..Flavours::default()
        }
    }

    fn sequence_holding(
        prior: &Flavours,
        chord_fails: bool,
    ) -> (PasteSequence<SpyDesktop, SpyClipboard>, Log) {
        sequence_with_copy_during_paste(prior, chord_fails, None)
    }

    fn sequence_with_copy_during_paste(
        prior: &Flavours,
        chord_fails: bool,
        copied: Option<Flavours>,
    ) -> (PasteSequence<SpyDesktop, SpyClipboard>, Log) {
        let log = Log::default();
        let inner = SharedClipboard::default();
        inner.borrow_mut().write_item(prior).unwrap();
        let sequence = PasteSequence {
            desktop: SpyDesktop {
                log: Rc::clone(&log),
                chord_fails,
                modifiers_stuck: false,
                copy_during_paste: copied.map(|flavours| (Rc::clone(&inner), flavours)),
            },
            clipboard: SpyClipboard {
                log: Rc::clone(&log),
                inner,
            },
        };
        (sequence, log)
    }

    #[test]
    fn the_chord_is_sent_after_hide_and_modifier_release_with_the_item_on_the_clipboard() {
        let (mut sequence, log) = sequence_holding(&text("prior"), false);

        sequence.run(TARGET, &text("sentinel")).unwrap();

        assert_eq!(
            *log.borrow(),
            [
                "snapshot",
                "write_item",
                "panel_hide",
                "wait_modifiers_released",
                "send_paste_chord",
                "restore"
            ]
        );
    }

    #[test]
    fn a_text_clipboard_is_back_to_its_prior_content_after_the_paste() {
        let (mut sequence, _log) = sequence_holding(&text("prior"), false);

        sequence.run(TARGET, &text("sentinel")).unwrap();

        assert_eq!(sequence.clipboard.read_flavours().unwrap(), text("prior"));
    }

    #[test]
    fn an_image_clipboard_is_back_to_its_prior_content_after_the_paste() {
        let (mut sequence, _log) = sequence_holding(&image(), false);

        sequence.run(TARGET, &text("sentinel")).unwrap();

        assert_eq!(sequence.clipboard.read_flavours().unwrap(), image());
    }

    #[test]
    fn a_failed_chord_reports_the_error_and_still_restores_the_prior_content() {
        let (mut sequence, _log) = sequence_holding(&text("prior"), true);

        let outcome = sequence.run(TARGET, &text("sentinel"));

        assert!(outcome.is_err());
        assert_eq!(sequence.clipboard.read_flavours().unwrap(), text("prior"));
    }

    #[test]
    fn a_copy_made_while_pasting_is_not_overwritten_by_the_restore() {
        let (mut sequence, log) =
            sequence_with_copy_during_paste(&text("prior"), false, Some(text("fresh copy")));

        sequence.run(TARGET, &text("sentinel")).unwrap();

        assert_eq!(
            sequence.clipboard.read_flavours().unwrap(),
            text("fresh copy")
        );
        assert!(!log.borrow().contains(&"restore"));
    }

    #[test]
    fn modifiers_that_never_release_send_no_chord_and_restore_the_prior_content() {
        let (mut sequence, log) = sequence_holding(&text("prior"), false);
        sequence.desktop.modifiers_stuck = true;

        let outcome = sequence.run(TARGET, &text("sentinel"));

        assert!(outcome.is_err());
        assert!(!log.borrow().contains(&"send_paste_chord"));
        assert_eq!(sequence.clipboard.read_flavours().unwrap(), text("prior"));
    }
}

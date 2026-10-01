use std::thread::sleep;
use std::time::{Duration, Instant};

use objc2_core_graphics::{
    CGEvent, CGEventFlags, CGEventSource, CGEventSourceStateID, CGEventTapLocation,
};

const MODIFIER_POLL_INTERVAL: Duration = Duration::from_millis(10);
const HELD_MODIFIERS: CGEventFlags = CGEventFlags::MaskCommand
    .union(CGEventFlags::MaskShift)
    .union(CGEventFlags::MaskAlternate)
    .union(CGEventFlags::MaskControl);

pub fn wait_modifiers_released(timeout: Duration) -> bool {
    let deadline = Instant::now() + timeout;
    while modifiers_held() {
        if Instant::now() >= deadline {
            return false;
        }
        sleep(MODIFIER_POLL_INTERVAL);
    }
    true
}

pub fn post_command_chord(keycode: u16) {
    for key_down in [true, false] {
        let event =
            CGEvent::new_keyboard_event(None, keycode, key_down).expect("CGEvent keyboard event");
        CGEvent::set_flags(Some(&event), CGEventFlags::MaskCommand);
        CGEvent::post(CGEventTapLocation::HIDEventTap, Some(&event));
    }
}

fn modifiers_held() -> bool {
    CGEventSource::flags_state(CGEventSourceStateID::CombinedSessionState)
        .intersects(HELD_MODIFIERS)
}

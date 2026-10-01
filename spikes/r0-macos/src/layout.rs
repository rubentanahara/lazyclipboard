use std::ffi::c_void;

use core_foundation::base::{CFType, TCFType};
use core_foundation::data::{CFData, CFDataRef};
use core_foundation::string::CFStringRef;

const KEY_ACTION_DISPLAY: u16 = 3;
const KEY_TRANSLATE_NO_DEAD_KEYS: u32 = 1;
const COMMAND_MODIFIER_STATE: u32 = 1;
const VIRTUAL_KEY_COUNT: u16 = 128;
const PASTE_CHARACTER: char = 'v';

type InputSourceRef = *const c_void;

#[link(name = "Carbon", kind = "framework")]
extern "C" {
    static kTISPropertyUnicodeKeyLayoutData: CFStringRef;
    fn TISCopyCurrentASCIICapableKeyboardLayoutInputSource() -> InputSourceRef;
    fn TISGetInputSourceProperty(source: InputSourceRef, key: CFStringRef) -> *const c_void;
    fn UCKeyTranslate(
        layout: *const c_void,
        virtual_key_code: u16,
        key_action: u16,
        modifier_key_state: u32,
        keyboard_type: u32,
        key_translate_options: u32,
        dead_key_state: *mut u32,
        max_string_length: usize,
        actual_string_length: *mut usize,
        unicode_string: *mut u16,
    ) -> i32;
    fn LMGetKbdType() -> u8;
}

pub fn paste_keycode() -> Result<u16, String> {
    let source = unsafe { TISCopyCurrentASCIICapableKeyboardLayoutInputSource() };
    if source.is_null() {
        return Err("no ASCII-capable keyboard layout is active".to_owned());
    }
    let source = unsafe { CFType::wrap_under_create_rule(source) };
    keycode_in_source(
        source.as_CFTypeRef(),
        PASTE_CHARACTER,
        COMMAND_MODIFIER_STATE,
    )
    .ok_or_else(|| format!("no key produces '{PASTE_CHARACTER}' in the active layout"))
}

fn keycode_in_source(source: InputSourceRef, character: char, modifier_state: u32) -> Option<u16> {
    let layout_data =
        unsafe { TISGetInputSourceProperty(source, kTISPropertyUnicodeKeyLayoutData) };
    if layout_data.is_null() {
        return None;
    }
    let layout_data = unsafe { CFData::wrap_under_get_rule(layout_data as CFDataRef) };
    if layout_data.is_empty() {
        return None;
    }
    let layout = layout_data.bytes().as_ptr().cast::<c_void>();
    (0..VIRTUAL_KEY_COUNT)
        .find(|&code| character_for_key(layout, code, modifier_state) == Some(character))
}

fn character_for_key(
    layout: *const c_void,
    virtual_key_code: u16,
    modifier_state: u32,
) -> Option<char> {
    let mut dead_key_state = 0u32;
    let mut produced = [0u16; 4];
    let mut produced_len = 0usize;
    let status = unsafe {
        UCKeyTranslate(
            layout,
            virtual_key_code,
            KEY_ACTION_DISPLAY,
            modifier_state,
            u32::from(LMGetKbdType()),
            KEY_TRANSLATE_NO_DEAD_KEYS,
            &mut dead_key_state,
            produced.len(),
            &mut produced_len,
            produced.as_mut_ptr(),
        )
    };
    if status != 0 || produced_len != 1 {
        return None;
    }
    char::from_u32(u32::from(produced[0]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_foundation::array::{CFArrayGetCount, CFArrayGetValueAtIndex, CFArrayRef};
    use core_foundation::string::CFString;
    use std::sync::Mutex;

    static TEXT_INPUT_SOURCES_ARE_NOT_THREAD_SAFE: Mutex<()> = Mutex::new(());

    const NO_MODIFIER_STATE: u32 = 0;
    const KEYCODE_ANSI_V: u16 = 0x09;
    const KEYCODE_ANSI_PERIOD: u16 = 0x2F;

    #[link(name = "Carbon", kind = "framework")]
    extern "C" {
        static kTISPropertyInputSourceID: CFStringRef;
        fn TISCreateInputSourceList(
            properties: *const c_void,
            include_all_installed: bool,
        ) -> CFArrayRef;
    }

    fn keycode_in_installed_layout(
        layout_id: &str,
        character: char,
        modifier_state: u32,
    ) -> Option<u16> {
        let _serialised = TEXT_INPUT_SOURCES_ARE_NOT_THREAD_SAFE
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let sources = unsafe { TISCreateInputSourceList(std::ptr::null(), true) };
        let count = unsafe { CFArrayGetCount(sources) };
        (0..count)
            .map(|index| unsafe { CFArrayGetValueAtIndex(sources, index) })
            .find(|&source| {
                let id = unsafe { TISGetInputSourceProperty(source, kTISPropertyInputSourceID) };
                let id = unsafe { CFString::wrap_under_get_rule(id as CFStringRef) };
                id == CFString::new(layout_id)
            })
            .and_then(|source| keycode_in_source(source, character, modifier_state))
    }

    #[test]
    fn us_layout_puts_v_on_the_ansi_v_key() {
        assert_eq!(
            keycode_in_installed_layout("com.apple.keylayout.US", 'v', COMMAND_MODIFIER_STATE),
            Some(KEYCODE_ANSI_V)
        );
    }

    #[test]
    fn dvorak_layout_puts_v_on_the_ansi_period_key() {
        assert_eq!(
            keycode_in_installed_layout("com.apple.keylayout.Dvorak", 'v', COMMAND_MODIFIER_STATE),
            Some(KEYCODE_ANSI_PERIOD)
        );
    }

    #[test]
    fn dvorak_qwerty_command_layout_resolves_command_v_to_the_ansi_v_key() {
        assert_eq!(
            keycode_in_installed_layout(
                "com.apple.keylayout.DVORAK-QWERTYCMD",
                'v',
                COMMAND_MODIFIER_STATE
            ),
            Some(KEYCODE_ANSI_V)
        );
    }

    #[test]
    fn russian_layout_has_no_latin_v_without_command_so_the_ascii_capable_layout_must_be_used() {
        assert_eq!(
            keycode_in_installed_layout("com.apple.keylayout.Russian", 'v', NO_MODIFIER_STATE),
            None
        );
    }
}

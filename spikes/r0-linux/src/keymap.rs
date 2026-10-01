pub const KEYSYM_LOWER_V: u32 = 0x76;
pub const KEYSYM_SHIFT_LEFT: u32 = 0xffe1;
pub const KEYSYM_CONTROL_LEFT: u32 = 0xffe3;

const MASK_SHIFT: u16 = 0x0001;
const MASK_CONTROL: u16 = 0x0004;
const MASK_ALT: u16 = 0x0008;
const MASK_SUPER: u16 = 0x0040;
const HELD_MODIFIER_MASK: u16 = MASK_SHIFT | MASK_CONTROL | MASK_ALT | MASK_SUPER;

pub fn modifiers_held(pointer_mask: u16) -> bool {
    pointer_mask & HELD_MODIFIER_MASK != 0
}

pub fn keycode_for_keysym(
    first_keycode: u8,
    keysyms_per_keycode: u8,
    keysyms: &[u32],
    wanted: u32,
) -> Option<u8> {
    let position = keysyms.iter().position(|&keysym| keysym == wanted)?;
    let row = position.checked_div(usize::from(keysyms_per_keycode))?;
    u8::try_from(row).ok()?.checked_add(first_keycode)
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEYSYM_K: u32 = 0x6b;
    const KEYSYM_UPPER_K: u32 = 0x4b;
    const KEYSYM_UPPER_V: u32 = 0x56;
    const KEYSYM_CYRILLIC_EM: u32 = 0x6cd;
    const KEYSYM_CYRILLIC_UPPER_EM: u32 = 0x6ed;
    const FIRST_KEYCODE: u8 = 8;
    const SYMS_PER_KEYCODE: u8 = 4;
    const NO_SYMBOL: u32 = 0;

    fn mapping(rows: &[[u32; 4]]) -> Vec<u32> {
        rows.iter().flatten().copied().collect()
    }

    #[test]
    fn dvorak_puts_v_on_the_key_qwerty_calls_period() {
        let rows = [
            [KEYSYM_K, KEYSYM_UPPER_K, NO_SYMBOL, NO_SYMBOL],
            [KEYSYM_LOWER_V, KEYSYM_UPPER_V, NO_SYMBOL, NO_SYMBOL],
        ];

        let keycode = keycode_for_keysym(
            FIRST_KEYCODE,
            SYMS_PER_KEYCODE,
            &mapping(&rows),
            KEYSYM_LOWER_V,
        );

        assert_eq!(keycode, Some(FIRST_KEYCODE + 1));
    }

    #[test]
    fn a_latin_v_on_the_second_group_is_found_under_a_russian_first_group() {
        let rows = [
            [NO_SYMBOL; 4],
            [
                KEYSYM_CYRILLIC_EM,
                KEYSYM_CYRILLIC_UPPER_EM,
                KEYSYM_LOWER_V,
                KEYSYM_UPPER_V,
            ],
        ];

        let keycode = keycode_for_keysym(
            FIRST_KEYCODE,
            SYMS_PER_KEYCODE,
            &mapping(&rows),
            KEYSYM_LOWER_V,
        );

        assert_eq!(keycode, Some(FIRST_KEYCODE + 1));
    }

    #[test]
    fn a_keysym_on_no_key_has_no_keycode() {
        let rows = [[KEYSYM_K, KEYSYM_UPPER_K, NO_SYMBOL, NO_SYMBOL]];

        let keycode = keycode_for_keysym(
            FIRST_KEYCODE,
            SYMS_PER_KEYCODE,
            &mapping(&rows),
            KEYSYM_LOWER_V,
        );

        assert_eq!(keycode, None);
    }

    const MASK_CAPS_LOCK: u16 = 0x0002;
    const MASK_NUM_LOCK: u16 = 0x0010;
    const MASK_BUTTON_ONE: u16 = 0x0100;

    #[test]
    fn shift_control_alt_and_super_count_as_held() {
        for mask in [MASK_SHIFT, MASK_CONTROL, MASK_ALT, MASK_SUPER] {
            assert!(modifiers_held(mask), "mask {mask:#06x}");
        }
    }

    #[test]
    fn lock_keys_and_mouse_buttons_do_not_count_as_held() {
        for mask in [0, MASK_CAPS_LOCK, MASK_NUM_LOCK, MASK_BUTTON_ONE] {
            assert!(!modifiers_held(mask), "mask {mask:#06x}");
        }
    }

    #[test]
    fn a_mapping_with_no_keysyms_per_keycode_has_no_keycode() {
        let keycode = keycode_for_keysym(FIRST_KEYCODE, 0, &[KEYSYM_LOWER_V], KEYSYM_LOWER_V);

        assert_eq!(keycode, None);
    }
}

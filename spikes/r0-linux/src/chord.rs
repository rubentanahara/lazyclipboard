const TERMINAL_CLASSES: [&str; 14] = [
    "alacritty",
    "foot",
    "gnome-terminal",
    "gnome-terminal-server",
    "guake",
    "kitty",
    "konsole",
    "ptyxis",
    "st",
    "st-256color",
    "tilix",
    "urxvt",
    "xterm",
    "yakuake",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasteChord {
    ControlV,
    ControlShiftV,
}

pub fn paste_chord_for_window_class(window_class: &str) -> PasteChord {
    let is_terminal = window_class
        .split_whitespace()
        .any(|word| TERMINAL_CLASSES.contains(&word.to_lowercase().as_str()));
    if is_terminal {
        PasteChord::ControlShiftV
    } else {
        PasteChord::ControlV
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminals_paste_with_control_shift_v() {
        for class in [
            "Gnome-terminal",
            "gnome-terminal-server",
            "Konsole",
            "kitty",
            "Alacritty",
            "XTerm",
        ] {
            assert_eq!(
                paste_chord_for_window_class(class),
                PasteChord::ControlShiftV,
                "{class}"
            );
        }
    }

    #[test]
    fn a_window_class_pair_reads_the_class_after_the_instance() {
        assert_eq!(
            paste_chord_for_window_class("gnome-terminal-server Gnome-terminal"),
            PasteChord::ControlShiftV
        );
    }

    #[test]
    fn terminals_missed_by_substring_matching_paste_with_control_shift_v() {
        for class in [
            "Tilix",
            "foot",
            "URxvt",
            "Ptyxis",
            "Guake",
            "Yakuake",
            "st-256color",
        ] {
            assert_eq!(
                paste_chord_for_window_class(class),
                PasteChord::ControlShiftV,
                "{class}"
            );
        }
    }

    #[test]
    fn editors_and_browsers_paste_with_control_v() {
        for class in [
            "Firefox",
            "firefox_firefox",
            "Code",
            "org.gnome.TextEditor",
            "determinant",
            "",
        ] {
            assert_eq!(
                paste_chord_for_window_class(class),
                PasteChord::ControlV,
                "{class}"
            );
        }
    }
}

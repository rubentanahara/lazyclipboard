const TERMINAL_CLASS_MARKERS: [&str; 4] = ["term", "konsole", "kitty", "alacritty"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasteChord {
    ControlV,
    ControlShiftV,
}

pub fn paste_chord_for_window_class(window_class: &str) -> PasteChord {
    let lowercase = window_class.to_lowercase();
    if TERMINAL_CLASS_MARKERS
        .iter()
        .any(|marker| lowercase.contains(marker))
    {
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
    fn editors_and_browsers_paste_with_control_v() {
        for class in [
            "Firefox",
            "firefox_firefox",
            "Code",
            "org.gnome.TextEditor",
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

use std::time::Duration;

const DEFAULT_SHORTCUT: &str = "Super+Alt+V";
const DEFAULT_FOCUS_SETTLE_MS: u64 = 50;
const DEFAULT_RESTORE_DELAY_MS: u64 = 250;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusMode {
    Activate,
    NoActivate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub shortcut: String,
    pub focus_mode: FocusMode,
    pub focus_settle: Duration,
    pub restore_delay: Duration,
}

impl Config {
    pub fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> Result<Self, String> {
        let focus_mode = match lookup("R0_FOCUS").as_deref() {
            None | Some("activate") => FocusMode::Activate,
            Some("noactivate") => FocusMode::NoActivate,
            Some(other) => {
                return Err(format!(
                    "R0_FOCUS must be activate or noactivate, got {other}"
                ))
            }
        };
        Ok(Self {
            shortcut: lookup("R0_SHORTCUT").unwrap_or_else(|| DEFAULT_SHORTCUT.to_owned()),
            focus_mode,
            focus_settle: millis(&lookup, "R0_FOCUS_SETTLE_MS", DEFAULT_FOCUS_SETTLE_MS)?,
            restore_delay: millis(&lookup, "R0_RESTORE_DELAY_MS", DEFAULT_RESTORE_DELAY_MS)?,
        })
    }
}

fn millis(
    lookup: &impl Fn(&str) -> Option<String>,
    name: &str,
    default: u64,
) -> Result<Duration, String> {
    let Some(raw) = lookup(name) else {
        return Ok(Duration::from_millis(default));
    };
    raw.parse()
        .map(Duration::from_millis)
        .map_err(|_| format!("{name} must be whole milliseconds, got {raw}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lookup_of<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        move |name| {
            pairs
                .iter()
                .find(|(key, _)| *key == name)
                .map(|(_, value)| (*value).to_owned())
        }
    }

    #[test]
    fn defaults_follow_the_design_spec() {
        let config = Config::from_lookup(lookup_of(&[])).unwrap();

        assert_eq!(config.shortcut, "Super+Alt+V");
        assert_eq!(config.focus_mode, FocusMode::Activate);
        assert_eq!(config.focus_settle, Duration::from_millis(50));
        assert_eq!(config.restore_delay, Duration::from_millis(250));
    }

    #[test]
    fn environment_overrides_every_knob() {
        let config = Config::from_lookup(lookup_of(&[
            ("R0_SHORTCUT", "Ctrl+Alt+F9"),
            ("R0_FOCUS", "noactivate"),
            ("R0_FOCUS_SETTLE_MS", "80"),
            ("R0_RESTORE_DELAY_MS", "400"),
        ]))
        .unwrap();

        assert_eq!(config.shortcut, "Ctrl+Alt+F9");
        assert_eq!(config.focus_mode, FocusMode::NoActivate);
        assert_eq!(config.focus_settle, Duration::from_millis(80));
        assert_eq!(config.restore_delay, Duration::from_millis(400));
    }

    #[test]
    fn invalid_values_name_the_variable() {
        let bad_focus = Config::from_lookup(lookup_of(&[("R0_FOCUS", "sideways")]));
        let bad_delay = Config::from_lookup(lookup_of(&[("R0_RESTORE_DELAY_MS", "soon")]));

        assert!(bad_focus.unwrap_err().contains("R0_FOCUS"));
        assert!(bad_delay.unwrap_err().contains("R0_RESTORE_DELAY_MS"));
    }
}

mod domain;
mod error;

pub use domain::{
    AiProvider, Group, GroupId, ItemContent, ItemId, ItemPreview, Os, Settings, Shortcuts, Theme,
    UsageMetric, DEFAULT_RETENTION_LIMIT,
};
pub use error::{AiError, CommandError};

use specta::Types;

pub fn types() -> Types {
    Types::default()
        .register::<GroupId>()
        .register::<ItemId>()
        .register::<Group>()
        .register::<ItemContent>()
        .register::<ItemPreview>()
        .register::<Settings>()
        .register::<UsageMetric>()
        .register::<CommandError>()
        .register::<AiError>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use specta_typescript::Typescript;

    const EXPORTED_TYPE_NAMES: &[&str] = &[
        "GroupId",
        "ItemId",
        "Group",
        "ItemContent",
        "ItemPreview",
        "Settings",
        "Shortcuts",
        "Theme",
        "AiProvider",
        "UsageMetric",
        "CommandError",
        "AiError",
    ];

    #[test]
    fn every_model_type_appears_in_generated_typescript() {
        let exported = Typescript::default()
            .export(&types(), specta_serde::Format)
            .expect("specta export");

        for name in EXPORTED_TYPE_NAMES {
            assert!(
                exported.contains(&format!("export type {name} ")),
                "{name} missing from generated TypeScript:\n{exported}"
            );
        }
    }

    #[test]
    fn default_chords_differ_per_os() {
        assert_eq!(Shortcuts::default_for(Os::Macos).copy_to_group, "Cmd+Alt+C");
        assert_eq!(
            Shortcuts::default_for(Os::Windows).paste_from_group,
            "Win+Alt+V"
        );
        assert_eq!(
            Shortcuts::default_for(Os::Linux).copy_to_group,
            "Ctrl+Alt+C"
        );
    }

    #[test]
    fn fresh_settings_keep_the_documented_defaults() {
        let settings = Settings::default_for(Os::Linux);

        assert_eq!(settings.retention_limit, 200);
        assert_eq!(settings.auto_delete_days, None);
        assert_eq!(settings.theme, Theme::System);
    }

    #[test]
    fn ai_error_converts_into_command_error() {
        assert_eq!(
            CommandError::from(AiError::NoKey),
            CommandError::Ai(AiError::NoKey)
        );
    }
}

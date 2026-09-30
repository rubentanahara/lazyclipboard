use lazyclipboard_core::model::{AiProvider, GroupId, Os, Settings, Theme};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CaptureTarget {
    Existing { group_id: GroupId },
    New { name: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct CapturePreview {
    pub plain_text: String,
    pub has_rich_text: bool,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum PasteFlavour {
    Default,
    Plain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum PasteOrder {
    OldestFirst,
    NewestFirst,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum PasteSeparator {
    NewLine,
    BlankLine,
    Space,
    Comma,
    BulletedList,
    NumberedList,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct AiResult {
    pub result_id: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct AiKeyStatus {
    pub anthropic: bool,
    pub openai: bool,
    pub gemini: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(default)]
pub struct SettingsPatch {
    pub theme: Option<Theme>,
    pub vim_mode: Option<bool>,
    pub launch_at_login: Option<bool>,
    pub usage_stats: Option<bool>,
    pub retention_limit: Option<u32>,
    pub auto_delete_days: Option<u32>,
    pub ai_provider: Option<AiProvider>,
    pub ai_model: Option<String>,
}

impl SettingsPatch {
    pub fn apply_to(self, settings: Settings) -> Settings {
        Settings {
            theme: self.theme.unwrap_or(settings.theme),
            vim_mode: self.vim_mode.unwrap_or(settings.vim_mode),
            launch_at_login: self.launch_at_login.unwrap_or(settings.launch_at_login),
            usage_stats: self.usage_stats.unwrap_or(settings.usage_stats),
            retention_limit: self.retention_limit.unwrap_or(settings.retention_limit),
            auto_delete_days: self.auto_delete_days.or(settings.auto_delete_days),
            ai_provider: self.ai_provider.or(settings.ai_provider),
            ai_model: self.ai_model.or(settings.ai_model),
            shortcuts: settings.shortcuts,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum ShortcutAction {
    CopyToGroup,
    PasteFromGroup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum PlatformOs {
    Macos,
    Windows,
    Linux,
}

impl From<Os> for PlatformOs {
    fn from(os: Os) -> Self {
        match os {
            Os::Macos => Self::Macos,
            Os::Windows => Self::Windows,
            Os::Linux => Self::Linux,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum DisplaySession {
    X11,
    Wayland,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct PlatformInfo {
    pub os: PlatformOs,
    pub session: Option<DisplaySession>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum PermissionStatus {
    Granted,
    Missing,
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum WindowKind {
    Main,
    Settings,
    Onboarding,
}

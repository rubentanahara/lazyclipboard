use serde::{Deserialize, Serialize};
use specta::Type;

pub const DEFAULT_RETENTION_LIMIT: u32 = 200;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(transparent)]
pub struct GroupId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(transparent)]
pub struct ItemId(pub u32);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
pub struct Group {
    pub id: GroupId,
    pub name: String,
    pub position: u32,
    pub never_send_to_ai: bool,
    #[specta(type = specta_typescript::Number)]
    pub created_at: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ItemContent {
    Text {
        text: String,
    },
    RichText {
        plain_text: String,
        html: String,
    },
    Link {
        url: String,
    },
    Image {
        file: String,
        width: u32,
        height: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
pub struct ItemPreview {
    pub id: ItemId,
    pub plain_text: String,
    pub has_rich_text: bool,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum UsageMetric {
    Captures,
    CaptureFailures,
    Pastes,
    PasteDegraded,
    PanelOpens,
    PanelCancels,
    Searches,
    SearchesWithoutResult,
    Undos,
    AiRequestsSucceeded,
    AiRequestsFailed,
    PanelOpenMs,
    CaptureMs,
    PasteMs,
    PanelOpenToPasteMs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    Light,
    Dark,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum AiProvider {
    Anthropic,
    Openai,
    Gemini,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Os {
    Macos,
    Windows,
    Linux,
}

impl Os {
    pub const fn current() -> Self {
        if cfg!(target_os = "macos") {
            Self::Macos
        } else if cfg!(target_os = "windows") {
            Self::Windows
        } else {
            Self::Linux
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
pub struct Shortcuts {
    pub copy_to_group: String,
    pub paste_from_group: String,
}

impl Shortcuts {
    pub fn default_for(os: Os) -> Self {
        let (copy_to_group, paste_from_group) = match os {
            Os::Macos => ("Cmd+Alt+C", "Cmd+Alt+V"),
            Os::Windows => ("Win+Shift+C", "Win+Alt+V"),
            Os::Linux => ("Ctrl+Alt+C", "Ctrl+Alt+V"),
        };
        Self {
            copy_to_group: copy_to_group.to_owned(),
            paste_from_group: paste_from_group.to_owned(),
        }
    }
}

impl Default for Shortcuts {
    fn default() -> Self {
        Self::default_for(Os::current())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(default)]
pub struct Settings {
    pub theme: Theme,
    pub shortcuts: Shortcuts,
    pub vim_mode: bool,
    pub launch_at_login: bool,
    pub usage_stats: bool,
    pub retention_limit: u32,
    pub auto_delete_days: Option<u32>,
    pub ai_provider: Option<AiProvider>,
    pub ai_model: Option<String>,
}

impl Settings {
    pub fn default_for(os: Os) -> Self {
        Self {
            theme: Theme::System,
            shortcuts: Shortcuts::default_for(os),
            vim_mode: false,
            launch_at_login: false,
            usage_stats: true,
            retention_limit: DEFAULT_RETENTION_LIMIT,
            auto_delete_days: None,
            ai_provider: None,
            ai_model: None,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::default_for(Os::current())
    }
}

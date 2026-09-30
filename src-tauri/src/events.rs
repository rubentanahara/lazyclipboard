use lazyclipboard_core::model::GroupId;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;

use crate::wire::{CapturePreview, PermissionStatus};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type, Event)]
#[serde(tag = "mode", rename_all = "snake_case")]
#[tauri_specta(event_name = "panel:show")]
pub enum PanelShow {
    Copy { capture: CapturePreview },
    Paste,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type, Event)]
#[tauri_specta(event_name = "panel:hidden")]
pub struct PanelHidden;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type, Event)]
#[tauri_specta(event_name = "data:groups-changed")]
pub struct DataGroupsChanged;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type, Event)]
#[tauri_specta(event_name = "data:items-changed")]
pub struct DataItemsChanged {
    pub group_id: GroupId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type, Event)]
#[tauri_specta(event_name = "settings:changed")]
pub struct SettingsChanged;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type, Event)]
#[tauri_specta(event_name = "permission:changed")]
pub struct PermissionChanged {
    pub status: PermissionStatus,
}

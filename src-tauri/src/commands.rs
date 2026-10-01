#![allow(unused_variables)]

use lazyclipboard_core::model::{
    AiError, AiProvider, CommandError, Group, GroupId, ItemId, ItemPreview, Os, Settings,
};
use tauri::State;

use crate::stub::Stub;
use crate::wire::{
    AiKeyStatus, AiPromptPreset, AiResult, CaptureTarget, ItemView, PasteFlavour, PasteOrder,
    PasteSeparator, PermissionStatus, PlatformInfo, SettingsPatch, ShortcutAction, WindowKind,
};

const STUB_AI_RESULT_ID: &str = "stub-result";
const STUB_AI_RESULT_TEXT: &str = "Stub AI result";
const STUB_DIAGNOSTICS_PATH: &str = "lazyclipboard-diagnostics.zip";

type CommandResult<T> = Result<T, CommandError>;

#[tauri::command]
#[specta::specta]
pub fn groups_list(stub: State<'_, Stub>) -> CommandResult<Vec<Group>> {
    stub.groups()
}

#[tauri::command]
#[specta::specta]
pub fn group_create(stub: State<'_, Stub>, name: String) -> CommandResult<Group> {
    let groups = stub.groups()?;
    let next_position = groups.len() as u32;
    let template = groups.into_iter().next().ok_or(CommandError::Internal)?;
    Ok(Group {
        id: GroupId(next_position + 1),
        name,
        position: next_position,
        never_send_to_ai: false,
        ..template
    })
}

#[tauri::command]
#[specta::specta]
pub fn group_rename(stub: State<'_, Stub>, id: GroupId, name: String) -> CommandResult<()> {
    stub.group(id).map(drop)
}

#[tauri::command]
#[specta::specta]
pub fn group_reorder(ids: Vec<GroupId>) -> CommandResult<()> {
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn group_delete(stub: State<'_, Stub>, id: GroupId) -> CommandResult<()> {
    stub.group(id).map(drop)
}

#[tauri::command]
#[specta::specta]
pub fn group_set_never_send_to_ai(
    stub: State<'_, Stub>,
    id: GroupId,
    never_send_to_ai: bool,
) -> CommandResult<()> {
    stub.group(id).map(drop)
}

#[tauri::command]
#[specta::specta]
pub fn items_list(stub: State<'_, Stub>, group_id: GroupId) -> CommandResult<Vec<ItemPreview>> {
    stub.items_in_group(group_id)
}

#[tauri::command]
#[specta::specta]
pub fn items_search(stub: State<'_, Stub>, query: String) -> CommandResult<Vec<ItemPreview>> {
    stub.items_matching(&query)
}

#[tauri::command]
#[specta::specta]
pub fn item_get(stub: State<'_, Stub>, id: ItemId) -> CommandResult<ItemView> {
    stub.item(id)
}

#[tauri::command]
#[specta::specta]
pub fn item_delete(stub: State<'_, Stub>, id: ItemId) -> CommandResult<()> {
    stub.item(id).map(drop)
}

#[tauri::command]
#[specta::specta]
pub fn item_undo_delete(id: ItemId) -> CommandResult<()> {
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn capture_save(stub: State<'_, Stub>, target: CaptureTarget) -> CommandResult<Group> {
    match target {
        CaptureTarget::Existing { group_id } => stub.group(group_id),
        CaptureTarget::New { name } => group_create(stub, name),
    }
}

#[tauri::command]
#[specta::specta]
pub fn capture_discard() -> CommandResult<()> {
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn paste_item(stub: State<'_, Stub>, id: ItemId, flavour: PasteFlavour) -> CommandResult<()> {
    stub.item(id).map(drop)
}

#[tauri::command]
#[specta::specta]
pub fn paste_all(
    stub: State<'_, Stub>,
    group_id: GroupId,
    order: PasteOrder,
    separator: PasteSeparator,
) -> CommandResult<()> {
    stub.group(group_id).map(drop)
}

#[tauri::command]
#[specta::specta]
pub fn paste_ai_result(result_id: String) -> CommandResult<()> {
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn panel_close() -> CommandResult<()> {
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn ai_reformat(
    stub: State<'_, Stub>,
    item_id: ItemId,
    preset: AiPromptPreset,
) -> CommandResult<AiResult> {
    stub.item(item_id).map(|_| stub_ai_result())
}

#[tauri::command]
#[specta::specta]
pub fn ai_summarize(
    stub: State<'_, Stub>,
    group_id: GroupId,
    order: PasteOrder,
    separator: PasteSeparator,
    preset: AiPromptPreset,
) -> CommandResult<AiResult> {
    stub.group(group_id).map(|_| stub_ai_result())
}

#[tauri::command]
#[specta::specta]
pub fn ai_key_set(provider: AiProvider, key: String) -> CommandResult<()> {
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn ai_key_delete(provider: AiProvider) -> CommandResult<()> {
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn ai_key_status() -> CommandResult<AiKeyStatus> {
    Ok(AiKeyStatus {
        anthropic: false,
        openai: false,
        gemini: false,
    })
}

#[tauri::command]
#[specta::specta]
pub fn ai_test_connection(provider: AiProvider) -> CommandResult<()> {
    Err(AiError::NoKey.into())
}

#[tauri::command]
#[specta::specta]
pub fn settings_get() -> CommandResult<Settings> {
    Ok(Settings::default())
}

#[tauri::command]
#[specta::specta]
pub fn settings_update(patch: SettingsPatch) -> CommandResult<Settings> {
    Ok(patch.apply_to(Settings::default()))
}

#[tauri::command]
#[specta::specta]
pub fn shortcut_set(action: ShortcutAction, chord: String) -> CommandResult<()> {
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn platform_info() -> CommandResult<PlatformInfo> {
    Ok(PlatformInfo {
        os: Os::current().into(),
        session: None,
    })
}

#[tauri::command]
#[specta::specta]
pub fn permission_status() -> CommandResult<PermissionStatus> {
    Ok(PermissionStatus::Granted)
}

#[tauri::command]
#[specta::specta]
pub fn permission_open_settings() -> CommandResult<()> {
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn window_open(kind: WindowKind) -> CommandResult<()> {
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn diagnostics_export() -> CommandResult<String> {
    Ok(STUB_DIAGNOSTICS_PATH.to_owned())
}

#[tauri::command]
#[specta::specta]
pub fn usage_clear() -> CommandResult<()> {
    Ok(())
}

fn stub_ai_result() -> AiResult {
    AiResult {
        result_id: STUB_AI_RESULT_ID.to_owned(),
        text: STUB_AI_RESULT_TEXT.to_owned(),
    }
}

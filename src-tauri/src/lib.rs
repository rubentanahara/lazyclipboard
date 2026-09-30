mod commands;
mod events;
mod stub;
mod wire;

use std::path::Path;

use specta_typescript::Typescript;
use tauri::Runtime;
use tauri_specta::{collect_commands, collect_events, Builder};

pub use stub::Stub;

pub const BINDINGS_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../apps/ui/src/shared/bindings/index.ts"
);

pub fn bindings_builder<R: Runtime>() -> Builder<R> {
    Builder::<R>::new()
        .commands(collect_commands![
            commands::groups_list,
            commands::group_create,
            commands::group_rename,
            commands::group_reorder,
            commands::group_delete,
            commands::group_set_never_send_to_ai,
            commands::items_list,
            commands::items_search,
            commands::item_get,
            commands::item_delete,
            commands::item_undo_delete,
            commands::capture_save,
            commands::capture_discard,
            commands::paste_item,
            commands::paste_all,
            commands::paste_ai_result,
            commands::panel_close,
            commands::ai_reformat,
            commands::ai_summarize,
            commands::ai_key_set,
            commands::ai_key_delete,
            commands::ai_key_status,
            commands::ai_test_connection,
            commands::settings_get,
            commands::settings_update,
            commands::shortcut_set,
            commands::platform_info,
            commands::permission_status,
            commands::permission_open_settings,
            commands::window_open,
            commands::diagnostics_export,
            commands::usage_clear,
        ])
        .events(collect_events![
            events::PanelShow,
            events::PanelHidden,
            events::DataGroupsChanged,
            events::DataItemsChanged,
            events::SettingsChanged,
            events::PermissionChanged,
        ])
        .types(&lazyclipboard_core::model::types())
}

pub fn export_bindings(path: &Path) -> Result<(), specta_typescript::Error> {
    bindings_builder::<tauri::Wry>().export(Typescript::default(), path)
}

pub fn run() {
    let builder = bindings_builder::<tauri::Wry>();
    tauri::Builder::default()
        .invoke_handler(builder.invoke_handler())
        .manage(Stub::seeded().expect("seed the stub database"))
        .setup(move |app| {
            builder.mount_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests;

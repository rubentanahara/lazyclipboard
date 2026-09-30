const APP_COMMANDS: &[&str] = &[
    "groups_list",
    "group_create",
    "group_rename",
    "group_reorder",
    "group_delete",
    "group_set_never_send_to_ai",
    "items_list",
    "items_search",
    "item_get",
    "item_delete",
    "item_undo_delete",
    "capture_save",
    "capture_discard",
    "paste_item",
    "paste_all",
    "paste_ai_result",
    "panel_close",
    "ai_reformat",
    "ai_summarize",
    "ai_key_set",
    "ai_key_delete",
    "ai_key_status",
    "ai_test_connection",
    "settings_get",
    "settings_update",
    "shortcut_set",
    "platform_info",
    "permission_status",
    "permission_open_settings",
    "window_open",
    "diagnostics_export",
    "usage_clear",
];

fn main() {
    let manifest = tauri_build::AppManifest::new().commands(APP_COMMANDS);
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(manifest))
        .expect("build the tauri app");
}

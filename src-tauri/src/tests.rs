use std::path::PathBuf;

use lazyclipboard_core::model::{
    AiError, CommandError, Group, ItemContent, ItemId, ItemPreview, Settings,
};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use tauri::test::{get_ipc_response, mock_builder, mock_context, noop_assets, MockRuntime};
use tauri::webview::InvokeRequest;
use tauri::{WebviewUrl, WebviewWindow, WebviewWindowBuilder};

use crate::wire::{AiKeyStatus, AiResult, PermissionStatus, PlatformInfo};
use crate::{bindings_builder, export_bindings, Stub, BINDINGS_PATH};

#[cfg(any(windows, target_os = "android"))]
const IPC_ORIGIN: &str = "http://tauri.localhost";
#[cfg(not(any(windows, target_os = "android")))]
const IPC_ORIGIN: &str = "tauri://localhost";
const SEEDED_GROUP_COUNT: usize = 20;
const SEEDED_ITEMS_PER_GROUP: usize = 200;

fn webview() -> WebviewWindow<MockRuntime> {
    let app = mock_builder()
        .invoke_handler(bindings_builder().invoke_handler())
        .manage(Stub::seeded().expect("seeded stub"))
        .build(mock_context(noop_assets()))
        .expect("mock app");
    WebviewWindowBuilder::new(&app, "main", WebviewUrl::default())
        .build()
        .expect("mock webview")
}

fn call<T: DeserializeOwned>(
    webview: &WebviewWindow<MockRuntime>,
    command: &str,
    arguments: Value,
) -> Result<T, CommandError> {
    let request = InvokeRequest {
        cmd: command.into(),
        callback: tauri::ipc::CallbackFn(0),
        error: tauri::ipc::CallbackFn(1),
        url: IPC_ORIGIN.parse().expect("url"),
        body: tauri::ipc::InvokeBody::Json(arguments),
        headers: Default::default(),
        invoke_key: tauri::test::INVOKE_KEY.to_string(),
    };
    get_ipc_response(webview, request)
        .map(|body| body.deserialize::<T>().expect("typed response"))
        .map_err(|error| serde_json::from_value(error).expect("typed error"))
}

#[test]
fn groups_list_returns_the_seeded_groups() {
    let groups: Vec<Group> = call(&webview(), "groups_list", json!({})).expect("groups");

    assert_eq!(groups.len(), SEEDED_GROUP_COUNT);
    assert_eq!(groups[0].name, "Group 00");
}

#[test]
fn items_list_returns_the_newest_items_first() {
    let items: Vec<ItemPreview> =
        call(&webview(), "items_list", json!({ "groupId": 1 })).expect("items");

    assert_eq!(items.len(), SEEDED_ITEMS_PER_GROUP);
    assert!(items[0].image_url.is_some());
    assert_eq!(items[1].plain_text, "https://example.com/group-0/item-198");
}

#[test]
fn item_get_returns_typed_content_and_not_found_is_a_typed_error() {
    let webview = webview();

    let content: ItemContent = call(&webview, "item_get", json!({ "id": 9 })).expect("link");
    let missing: Result<ItemContent, CommandError> =
        call(&webview, "item_get", json!({ "id": 1_000_000 }));

    assert_eq!(
        content,
        ItemContent::Link {
            url: "https://example.com/group-0/item-8".to_owned()
        }
    );
    assert_eq!(missing, Err(CommandError::NotFound));
}

#[test]
fn items_search_finds_seeded_text() {
    let items: Vec<ItemPreview> = call(
        &webview(),
        "items_search",
        json!({ "query": "Group 3 item 7." }),
    )
    .expect("items");

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, ItemId(3 * 200 + 8));
}

#[test]
fn ai_commands_return_fixture_results_and_typed_errors() {
    let webview = webview();

    let result: AiResult = call(
        &webview,
        "ai_reformat",
        json!({ "itemId": 1, "prompt": "shorter" }),
    )
    .expect("result");
    let status: AiKeyStatus = call(&webview, "ai_key_status", json!({})).expect("status");
    let test: Result<(), CommandError> = call(
        &webview,
        "ai_test_connection",
        json!({ "provider": "anthropic" }),
    );

    assert_eq!(result.result_id, "stub-result");
    assert!(!status.anthropic && !status.openai && !status.gemini);
    assert_eq!(test, Err(CommandError::Ai(AiError::NoKey)));
}

#[test]
fn settings_and_platform_commands_return_fixtures() {
    let webview = webview();

    let settings: Settings = call(&webview, "settings_get", json!({})).expect("settings");
    let updated: Settings = call(
        &webview,
        "settings_update",
        json!({ "patch": { "vim_mode": true } }),
    )
    .expect("updated");
    let platform: PlatformInfo = call(&webview, "platform_info", json!({})).expect("platform");
    let permission: PermissionStatus =
        call(&webview, "permission_status", json!({})).expect("permission");

    assert_eq!(settings, Settings::default());
    assert!(updated.vim_mode);
    assert_eq!(platform.os, lazyclipboard_core::model::Os::current().into());
    assert_eq!(permission, PermissionStatus::Granted);
}

#[test]
fn committed_bindings_match_the_rust_signatures() {
    let regenerated = std::env::temp_dir().join("lazyclipboard-bindings-check.ts");

    export_bindings(&regenerated).expect("export");

    assert_eq!(
        std::fs::read_to_string(&regenerated).expect("regenerated"),
        std::fs::read_to_string(PathBuf::from(BINDINGS_PATH)).expect("committed bindings"),
        "stale bindings: run `make bindings`"
    );
}

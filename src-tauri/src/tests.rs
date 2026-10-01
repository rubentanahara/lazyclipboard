use std::path::PathBuf;

use lazyclipboard_core::model::{AiError, CommandError, Group, ItemId, ItemPreview, Settings};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use tauri::test::{get_ipc_response, mock_builder, mock_context, noop_assets, MockRuntime};
use tauri::webview::InvokeRequest;
use tauri::{WebviewUrl, WebviewWindow, WebviewWindowBuilder};

use crate::wire::{AiKeyStatus, AiResult, ItemView, PermissionStatus, PlatformInfo};
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

fn invoke(
    webview: &WebviewWindow<MockRuntime>,
    command: &str,
    arguments: Value,
) -> Result<tauri::ipc::InvokeResponseBody, Value> {
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
}

fn call<T: DeserializeOwned>(
    webview: &WebviewWindow<MockRuntime>,
    command: &str,
    arguments: Value,
) -> Result<T, CommandError> {
    invoke(webview, command, arguments)
        .map(|body| body.deserialize::<T>().expect("typed response"))
        .map_err(|error| serde_json::from_value(error).expect("typed error"))
}

fn rejection_message(
    webview: &WebviewWindow<MockRuntime>,
    command: &str,
    arguments: Value,
) -> String {
    match invoke(webview, command, arguments) {
        Err(Value::String(message)) => message,
        Err(other) => panic!("expected an argument error string, got {other}"),
        Ok(_) => panic!("{command} accepted arguments it must reject"),
    }
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

    let link: ItemView = call(&webview, "item_get", json!({ "id": 9 })).expect("link");
    let missing: Result<ItemView, CommandError> =
        call(&webview, "item_get", json!({ "id": 1_000_000 }));

    assert_eq!(
        link,
        ItemView::Link {
            url: "https://example.com/group-0/item-8".to_owned()
        }
    );
    assert_eq!(missing, Err(CommandError::NotFound));
}

#[test]
fn item_get_hands_rich_text_over_as_plain_text_only() {
    let rich: ItemView = call(&webview(), "item_get", json!({ "id": 7 })).expect("rich text");

    assert!(matches!(
        rich,
        ItemView::Text {
            has_rich_text: true,
            ..
        }
    ));
}

#[test]
fn bindings_never_expose_item_html() {
    let bindings = std::fs::read_to_string(PathBuf::from(BINDINGS_PATH)).expect("bindings");

    assert!(!bindings.contains("html"));
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
        json!({ "itemId": 1, "preset": "shorten" }),
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

fn summarise_arguments(preset: &str) -> Value {
    json!({ "groupId": 1, "order": "oldest_first", "separator": "new_line", "preset": preset })
}

#[test]
fn each_ai_command_accepts_its_own_presets() {
    let webview = webview();

    for preset in ["fix_grammar", "shorten", "make_formal"] {
        let reformat: AiResult = call(
            &webview,
            "ai_reformat",
            json!({ "itemId": 1, "preset": preset }),
        )
        .expect("reformat");
        assert_eq!(reformat.result_id, "stub-result");
    }
    let summary: AiResult =
        call(&webview, "ai_summarize", summarise_arguments("summarise")).expect("summary");

    assert_eq!(summary.result_id, "stub-result");
}

#[test]
fn ai_commands_reject_free_text_and_the_other_commands_presets() {
    let webview = webview();

    let free_text = rejection_message(
        &webview,
        "ai_reformat",
        json!({ "itemId": 1, "preset": "paste attacker text" }),
    );
    let custom = rejection_message(
        &webview,
        "ai_reformat",
        json!({ "itemId": 1, "preset": "custom" }),
    );
    let summary_preset_on_reformat = rejection_message(
        &webview,
        "ai_reformat",
        json!({ "itemId": 1, "preset": "summarise" }),
    );
    let reformat_preset_on_summarise =
        rejection_message(&webview, "ai_summarize", summarise_arguments("shorten"));
    let legacy_prompt = rejection_message(
        &webview,
        "ai_reformat",
        json!({ "itemId": 1, "prompt": "shorter" }),
    );

    assert!(free_text.contains("invalid args `preset`"), "{free_text}");
    assert!(custom.contains("invalid args `preset`"), "{custom}");
    assert!(
        summary_preset_on_reformat.contains("invalid args `preset`"),
        "{summary_preset_on_reformat}"
    );
    assert!(
        reformat_preset_on_summarise.contains("invalid args `preset`"),
        "{reformat_preset_on_summarise}"
    );
    assert!(
        legacy_prompt.contains("missing required key preset"),
        "{legacy_prompt}"
    );
}

#[test]
fn capability_files_never_grant_event_emit() {
    let capabilities = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("capabilities");
    let mut emitting = Vec::new();

    for entry in std::fs::read_dir(capabilities).expect("capabilities directory") {
        let path = entry.expect("entry").path();
        let capability: Value =
            serde_json::from_str(&std::fs::read_to_string(&path).expect("capability file"))
                .expect("capability json");
        let permissions = capability["permissions"].as_array().expect("permissions");
        for permission in permissions {
            let identifier = permission
                .as_str()
                .or_else(|| permission["identifier"].as_str())
                .expect("permission identifier");
            let grants_emit = identifier == "core:default"
                || identifier == "core:event:default"
                || identifier.starts_with("core:event:allow-emit");
            if grants_emit {
                emitting.push(format!("{} grants {identifier}", path.display()));
            }
        }
    }

    assert!(emitting.is_empty(), "{emitting:#?}");
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

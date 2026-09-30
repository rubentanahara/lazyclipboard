use std::collections::BTreeSet;

use lazyclipboard_app::{bindings_builder, Stub, BINDINGS_PATH};
use serde_json::{json, Value};
use tauri::test::MockRuntime;
use tauri::test::{get_ipc_response, mock_builder, INVOKE_KEY};
use tauri::utils::config::Csp;
use tauri::webview::InvokeRequest;
use tauri::{App, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

#[cfg(any(windows, target_os = "android"))]
const IPC_ORIGIN: &str = "http://tauri.localhost";
#[cfg(not(any(windows, target_os = "android")))]
const IPC_ORIGIN: &str = "tauri://localhost";

const PANEL: &str = "panel";
const MAIN: &str = "main";
const SETTINGS: &str = "settings";
const ONBOARDING: &str = "onboarding";
const ALL_WINDOWS: [&str; 4] = [PANEL, MAIN, SETTINGS, ONBOARDING];

const COMMAND_WINDOWS: [(&str, &[&str]); 32] = [
    ("groups_list", &[PANEL, MAIN, SETTINGS, ONBOARDING]),
    ("group_create", &[PANEL, MAIN, SETTINGS]),
    ("group_rename", &[MAIN, SETTINGS]),
    ("group_reorder", &[MAIN, SETTINGS]),
    ("group_delete", &[MAIN, SETTINGS]),
    ("group_set_never_send_to_ai", &[MAIN, SETTINGS]),
    ("items_list", &[PANEL, MAIN]),
    ("items_search", &[PANEL, MAIN]),
    ("item_get", &[PANEL, MAIN]),
    ("item_delete", &[PANEL, MAIN]),
    ("item_undo_delete", &[PANEL, MAIN]),
    ("capture_save", &[PANEL]),
    ("capture_discard", &[PANEL]),
    ("paste_item", &[PANEL]),
    ("paste_all", &[PANEL]),
    ("paste_ai_result", &[PANEL]),
    ("panel_close", &[PANEL]),
    ("ai_reformat", &[PANEL]),
    ("ai_summarize", &[PANEL]),
    ("ai_key_set", &[SETTINGS, ONBOARDING]),
    ("ai_key_delete", &[SETTINGS, ONBOARDING]),
    ("ai_key_status", &[SETTINGS, ONBOARDING]),
    ("ai_test_connection", &[SETTINGS, ONBOARDING]),
    ("settings_get", &[PANEL, MAIN, SETTINGS, ONBOARDING]),
    ("settings_update", &[SETTINGS, ONBOARDING]),
    ("shortcut_set", &[SETTINGS, ONBOARDING]),
    ("platform_info", &[PANEL, MAIN, SETTINGS, ONBOARDING]),
    ("permission_status", &[SETTINGS, ONBOARDING]),
    ("permission_open_settings", &[SETTINGS, ONBOARDING]),
    ("window_open", &[PANEL, MAIN, SETTINGS]),
    ("diagnostics_export", &[SETTINGS]),
    ("usage_clear", &[SETTINGS]),
];

fn context() -> tauri::Context<MockRuntime> {
    tauri::generate_context!()
}

fn app() -> App<MockRuntime> {
    mock_builder()
        .invoke_handler(bindings_builder().invoke_handler())
        .manage(Stub::seeded().expect("seeded stub"))
        .build(context())
        .expect("mock app")
}

fn is_permitted(webview: &WebviewWindow<MockRuntime>, command: &str) -> bool {
    let request = InvokeRequest {
        cmd: command.into(),
        callback: tauri::ipc::CallbackFn(0),
        error: tauri::ipc::CallbackFn(1),
        url: IPC_ORIGIN.parse().expect("url"),
        body: tauri::ipc::InvokeBody::Json(json!({})),
        headers: Default::default(),
        invoke_key: INVOKE_KEY.to_string(),
    };
    match get_ipc_response(webview, request) {
        Ok(_) => true,
        Err(Value::String(message)) => !message.contains("not allowed"),
        Err(_) => true,
    }
}

#[test]
fn every_window_may_call_only_its_own_commands() {
    let app = app();
    let mut violations = Vec::new();

    for window_label in ALL_WINDOWS {
        let webview = WebviewWindowBuilder::new(&app, window_label, WebviewUrl::default())
            .build()
            .expect("mock webview");
        for (command, allowed_windows) in COMMAND_WINDOWS {
            let expected = allowed_windows.contains(&window_label);
            if is_permitted(&webview, command) != expected {
                violations.push(format!(
                    "{window_label} {command} expected permitted={expected}"
                ));
            }
        }
    }

    assert!(violations.is_empty(), "{violations:#?}");
}

fn content_security_policy() -> String {
    match context().config().app.security.csp.clone() {
        Some(Csp::Policy(policy)) => policy,
        other => panic!("expected a CSP policy string, got {other:?}"),
    }
}

fn directive_sources(policy: &str, directive: &str) -> Vec<String> {
    policy
        .split(';')
        .map(str::trim)
        .find_map(|entry| entry.strip_prefix(directive)?.strip_prefix(' '))
        .map(|sources| {
            sources
                .split_whitespace()
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

#[test]
fn csp_blocks_remote_scripts() {
    let policy = content_security_policy();

    assert_eq!(directive_sources(&policy, "default-src"), ["'self'"]);
    assert_eq!(directive_sources(&policy, "script-src"), ["'self'"]);
    assert_eq!(directive_sources(&policy, "object-src"), ["'none'"]);
}

#[test]
fn every_registered_command_has_a_window_row() {
    let bindings = std::fs::read_to_string(BINDINGS_PATH).expect("generated bindings");
    let registered: BTreeSet<&str> = bindings
        .split("TAURI_INVOKE(\"")
        .skip(1)
        .filter_map(|after_quote| after_quote.split('"').next())
        .collect();
    let listed: BTreeSet<&str> = COMMAND_WINDOWS
        .iter()
        .map(|(command, _)| *command)
        .collect();

    assert_eq!(registered, listed);
}

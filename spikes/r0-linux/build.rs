const APP_COMMANDS: &[&str] = &["panel_ready", "panel_hide", "paste_sentinel"];

fn main() {
    let manifest = tauri_build::AppManifest::new().commands(APP_COMMANDS);
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(manifest))
        .expect("build the r0-linux spike");
}

use std::path::Path;

fn main() {
    lazyclipboard_app::export_bindings(Path::new(lazyclipboard_app::BINDINGS_PATH))
        .expect("export bindings");
}

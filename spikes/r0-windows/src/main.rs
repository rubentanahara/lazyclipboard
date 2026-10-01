#![cfg_attr(not(windows), allow(dead_code))]
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

#[cfg(windows)]
mod app;
mod config;
#[cfg(windows)]
mod win32;

#[cfg(windows)]
fn main() {
    app::run();
}

#[cfg(not(windows))]
fn main() {
    eprintln!("r0-windows runs on Windows only");
}

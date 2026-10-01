use std::process::Command;

const OPEN_COMMAND: &str = "/usr/bin/open";
const ACCESSIBILITY_SETTINGS_URL: &str =
    "x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility";

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn AXIsProcessTrusted() -> u8;
}

pub fn is_trusted() -> bool {
    unsafe { AXIsProcessTrusted() != 0 }
}

pub fn open_settings() -> std::io::Result<()> {
    Command::new(OPEN_COMMAND)
        .arg(ACCESSIBILITY_SETTINGS_URL)
        .status()
        .map(drop)
}

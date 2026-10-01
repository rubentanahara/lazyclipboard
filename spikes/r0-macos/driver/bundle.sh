#!/bin/sh
set -e
cd "$(dirname "$0")/.."
app=target/r0-macos.app
rm -rf "$app"
mkdir -p "$app/Contents/MacOS"
cp target/release/r0-macos "$app/Contents/MacOS/r0-macos"
cat > "$app/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key><string>r0-macos</string>
  <key>CFBundleIdentifier</key><string>io.github.rubentanahara.lazyclipboard.r0macos</string>
  <key>CFBundleName</key><string>r0-macos</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>LSUIElement</key><true/>
</dict>
</plist>
PLIST
codesign --force --sign - "$app"

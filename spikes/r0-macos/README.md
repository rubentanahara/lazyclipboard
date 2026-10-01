# r0-macos

R0 spike for #77: one global shortcut (⌘⌥V), one non-activating panel and one paste of a fixed sentinel on macOS. Standalone crate, outside the root Cargo and pnpm workspaces. On Windows and Linux it compiles to a stub.

## Run

```sh
cargo run --release
```

Grant Accessibility to the terminal that runs it. A launch from Finder or `open` needs its own grant.

## Scripted smoke run

`driver/r0-driver.swift` posts the shortcut and keys with `CGEventPost`, reads the target app back and compares the clipboard. It refuses to post a key while the frontmost app is not the target, and discards an iteration when it sees keys it did not send. Keep your hands off the Mac while it runs.

```sh
cargo build --release
swiftc -O driver/r0-driver.swift -o target/r0-driver
target/r0-driver target/release/r0-macos target/run.log full TextEdit 25
```

Scenarios: `full` (R0-1 to R0-5), `shift` (R0-8 with ⇧ held), `image` (R0-5 image), `layout` (paste under the current input source; use TextEdit or Safari, Terminal types with fixed ANSI keycodes), `fullscreen` (R0-7, TextEdit only), `permission` (R0-9). Targets: `TextEdit`, `Terminal`, `Safari`. Safari is served a textarea page by `driver/textarea_server.py`. The driver creates its own TextEdit document, Terminal window and Safari window and closes only those. Exit code is 1 when a check fails, 2 on abort.

`permission` needs an app without an Accessibility grant: run `driver/bundle.sh` and pass `target/r0-macos.app` as the binary.

`R0_RESTORE_DELAY_MS` overrides the clipboard restore delay for tuning.

Results: `results.md`.

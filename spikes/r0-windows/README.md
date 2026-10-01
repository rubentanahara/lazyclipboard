# R0 spike: Windows

One global shortcut, one panel, one paste of a fixed sentinel. Throwaway code: deleted after the R0 gate, proven parts move into `src-tauri/crates/os`.

Standalone crate (own `[workspace]`), not part of the root Cargo or pnpm workspace. On macOS and Linux only the pure modules build (`cargo test` runs their tests); the app itself builds and runs on Windows.

## Build and run (Windows)

Needs the MSVC build tools and WebView2 (preinstalled on Windows 10 and 11).

```
cd spikes\r0-windows
cargo run --release
```

The app has no visible window until the shortcut fires. Default chord: Win+Alt+V. Press it to open the panel from the app you are in; ↑/↓ move, ↵ pastes the selected row, Esc closes. The first row is the sentinel `LAZYCLIPBOARD-R0-SENTINEL`. Stop the app from Task Manager.

## Knobs (environment variables)

| Variable | Default | Effect |
| --- | --- | --- |
| `R0_SHORTCUT` | `Super+Alt+V` | Chord to register (for example when another app owns Win+Alt+V) |
| `R0_FOCUS` | `activate` | `activate`: panel takes focus, target is re-activated on hide. `noactivate`: `WS_EX_NOACTIVATE`, the panel never takes focus (R1) |
| `R0_FOCUS_SETTLE_MS` | `50` | Wait between returning focus and injecting Ctrl+V |
| `R0_RESTORE_DELAY_MS` | `250` | Wait between injecting Ctrl+V and restoring the clipboard |

Example: `set R0_FOCUS=noactivate && cargo run --release`.

## Log

Every event goes to `%TEMP%\lazyclipboard-r0-windows.log` (one line per event, epoch milliseconds first):

- `shortcut target_hwnd=… blocked_by_uipi=…`: the app that was in front, and whether it runs elevated above the spike.
- `open ready_ms=… foreground_is_target=… opens=N p95_ms=… PASS|FAIL|INCOMPLETE`: shortcut to first frame (R0-6). `foreground_is_target=true` while the panel is open is Strict focus (R0-3).
- `hide focus_returned=… focus_return_ms=…`: time from hide until the target is foreground again (R0-3 Acceptable is ≤ 100 ms).
- `snapshot formats(id,bytes)=… skipped=…`: clipboard formats captured before the paste; `skipped` lists formats that could not be copied.
- `pasted modifiers_released=…`: `false` means a modifier was still held after 1000 ms.
- `clipboard restored` or `clipboard changed by another process, not restoring`.

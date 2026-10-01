# R0 macOS results

Machine: macOS 26.3.1, Apple silicon, release build, tauri-nspanel `6e595d7`, Tauri 2.12.1. Layout in effect: U.S. Targets run by the scripted driver: TextEdit, Terminal (plain `bash --norc`), Safari (textarea page). Hands off during every run; iterations with foreign keys were discarded and retried (none were needed in the runs below).

## Pass or fail

| ID | Result |
| --- | --- |
| R0-1 no chord character leaks | TextEdit 25/25, Terminal 20/20, Safari 20/20. VS Code not run (not installed) |
| R0-2 panel receives typed characters, ↑/↓, Esc and ↵ | 25/25 TextEdit, 20/20 Terminal, 20/20 Safari |
| R0-3 Strict focus | Frontmost app stayed the target while the panel was open and after hide: TextEdit 25/25, Terminal 20/20, Safari 20/20. VS Code not run |
| R0-4 ↵ inserts the sentinel at the caret | TextEdit 25/25, Terminal 20/20, Safari 20/20. VS Code not run |
| R0-5 clipboard restored | Text 65/65 across the three apps, image 5/5 (TextEdit, PNG and TIFF flavours byte-equal) |
| R0-6 shortcut to panel ready | 50 opens in TextEdit: p95 50.5 ms, max 88.3 ms (budget 150 ms). ↵ to text visible 60 fps recording: not run |
| R0-7 fullscreen Space | TextEdit in a fullscreen Space (AXFullScreen verified): 5/5 by window list, and the panel is visible in two screenshots |
| R0-8 ⇧ held on ↵ | 5/5 in each of TextEdit, Terminal, Safari; paste waited for the ⇧ release (about 350 ms). Dvorak and Russian layouts not run |
| R0-9 Accessibility off | Unsigned `.app` started untrusted: permission state shown 5/5, paste refused with a reason 5/5. Granting, paste after granting and relaunch need a person |
| R0-10 3-OS CI | Not verifiable here: `.github` is outside Owns. The crate builds on macOS and compiles as a stub elsewhere |

## Recorded

- Panel ready time is bimodal, about 25 to 50 ms with an occasional 90 to 113 ms open; every run's p95 stayed under 150 ms (Terminal 96.0 ms, Safari 50.1 ms and 93.4 ms, TextEdit 50.5 ms).
- Release binary 10.3 MB. Idle app process RSS 99 MB, 0.0% CPU after 8 s (WebKit helper processes not separated out).
- Constants used: MODIFIER_RELEASE_TIMEOUT 1000 ms, FOCUS_SETTLE_DELAY 50 ms, RESTORE_DELAY 250 ms (overridable with `R0_RESTORE_DELAY_MS`). ↵ to chord posted: 55 to 70 ms.
- First Safari run, taken while other worktrees were building, pasted the prior clipboard text in 7 of 20 iterations, so the restore ran before Safari read the paste. It did not repeat with a 250 ms delay when idle or under 7 busy-loop workers (20/20 each), and 600 ms was 20/20 as well. Cause unexplained; raise RESTORE_DELAY if it recurs.

## Findings for the os crate

- Text Input Source calls (`TISCopyCurrentASCIICapableKeyboardLayoutInputSource`, `TISGetInputSourceProperty`) abort off the main thread (`dispatch_assert_queue_fail`). Look the keycode up on the main thread.
- Hiding the NSPanel from a worker thread aborts inside AppKit window management. Dispatch show and hide to the main thread (TC-4).
- The layout-aware ⌘V keycode must come from the ASCII-capable layout, not the active one: the Russian layout has no Latin `v`. Unit tests pin US to 0x09 and Dvorak to 0x2F against installed layouts without enabling them.
- The Carbon hotkey consumes ⌘⌥V, so nothing reaches the target.
- A nonactivating NSPanel made key keeps the target frontmost and receives keys.
- AXIsProcessTrusted for a process started from a terminal reports the terminal's grant; test the permission state from a separate app bundle.

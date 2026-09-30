# lazyclipboard technical approach

Engineering companion to the PRD. Written 2026-09-30 by the tech-lead pass. `[verify]` marks a library or OS fact not yet confirmed; the owning issue checks it before starting. `[NEEDS INPUT]` marks an open question, tracked in the PRD Open Issues.

## 1. Technical Considerations

**Status: greenfield.** The repository has the design spec, the agent guide, the security policy and the AI provider research. It has no code, schema or CI yet. Everything below is the proposed Sprint 0 design, built on the decisions in the Decision Log. `[verify]` marks a library or OS fact the repository could not confirm. The owning issue checks it before it starts.

### 1.1 Architecture overview

**Processes and windows**
- There is one Rust process (the Tauri app) plus the helper processes of the system webview: WKWebView on macOS, WebView2 on Windows, WebKitGTK 4.1 on Linux.
- The Cargo workspace has one app crate, `src-tauri`, which holds the Tauri setup, commands, events, capabilities and `tauri.conf.json`. It links three library crates:
  - `crates/core`: DB, models, classification, retention, usage counters
  - `crates/os`: shortcuts, capture, paste, panel, tray, keychain, per OS
  - `crates/ai`: the `AiProvider` trait and three providers
- There are four windows. Each has its own Vite entry under `apps/ui/src/windows/<name>`:
  - `panel` is created **hidden at startup** and reused for Copy to group, Paste from group, Paste all and the AI views. It is never destroyed. Opening a panel shows a window that already exists; it does not boot a webview.
  - `main`, `settings` and `onboarding` are created when opened and destroyed when closed, so they cost no memory while idle.
- The tray uses the Tauri core tray API with Open, Settings and Quit. The single-instance plugin makes a second launch focus the Main Window instead of starting a second process.
- On macOS the app runs as an accessory app (no Dock icon). See Q17.

**Summoning a panel**
1. The OS delivers the global shortcut to Rust. The webview never sees it.
2. If a capture or paste sequence is already running, Rust ignores the press (TC-3).
3. Rust records the frontmost app or window. It is the capture source or the paste target.
4. For **Copy to group**, Rust runs the capture sequence (1.3) and shows the panel with the captured content. For **Paste from group**, Rust shows the panel directly.
5. Rust sets the width: 520, or 560 for AI and Paste all. It centres the panel on the display under the mouse cursor (an assumption, see Q18). It emits `panel:show`, then shows the panel without activating (1.5).
6. The panel UI resets its view state, renders, and focuses the list or search field.

**Dismissing a panel**
- Esc or Ctrl+[: the UI calls `panel_close`.
- Clicking elsewhere: Rust hides the panel when it loses key status. On macOS this is the NSPanel resign-key callback; on Windows and Linux it is Tauri's `Focused(false)` window event.
- A successful save or paste: Rust hides the panel before it injects keys.
- On hide, Rust emits `panel:hidden` and drops the pending capture and any AI result. The UI clears search, selection and subview.

**State ownership**

| State | Owner | Lifetime |
| --- | --- | --- |
| Groups, items, settings, usage counters | `core`, SQLite | Persistent |
| Image bytes | `core`, files on disk | Persistent until the item is deleted |
| API keys | OS keychain, via `os` | Persistent; read per request, never sent to the webview |
| Pending capture | App crate, in memory | Until save, discard or panel hide |
| Clipboard snapshot | `os`, in memory | One capture or paste sequence |
| Paste target / capture source handle | `os`, in memory | One panel session |
| AI result | App crate, in memory, keyed by `result_id` | Until paste or panel hide (Q6) |
| View state (selection, query, subview, Undo toast timer) | That window's Zustand store | Reset on `panel:hidden` or window close |

**IPC (Tauri commands generated with tauri-specta; names are fixed in contract C2)**

| Area | Commands | Windows allowed (Tauri capability) |
| --- | --- | --- |
| Groups | `groups_list`, `group_create(name)`, `group_rename(id,name)`, `group_reorder(ids)`, `group_delete(id)`, `group_set_never_send_to_ai(id,bool)` | list: all; create: panel, main, settings; others: main, settings |
| Items | `items_list(group_id)`, `items_search(query)`, `item_get(id)`, `item_delete(id)`, `item_undo_delete(id)` | panel, main |
| Capture | `capture_save(target: Existing{group_id} \| New{name})`, `capture_discard()` | panel |
| Paste | `paste_item(id, flavour: Default \| Plain)`, `paste_all(group_id, order, separator)`, `paste_ai_result(result_id)`, `panel_close()` | panel |
| AI | `ai_reformat(item_id, prompt)` and `ai_summarize(group_id, order, separator, prompt)` both return `{result_id, text}`; `ai_key_set(provider,key)`, `ai_key_delete(provider)`, `ai_key_status()` (present or absent per provider), `ai_test_connection(provider)` | reformat and summarize: panel; keys and test: settings, onboarding |
| Settings | `settings_get()`, `settings_update(patch)`, `shortcut_set(action, chord)` (unregister, validate, register; returns `Conflict`) | get: all; others: settings, onboarding |
| Platform | `platform_info()` (OS, session: x11 or wayland), `permission_status()`, `permission_open_settings()`, `window_open(kind)`, `diagnostics_export()`, `usage_clear()` | Per window, as needed |

**Events (Rust to windows):**
- `panel:show`, with payload `{mode:"copy", capture: CapturePreview} | {mode:"paste"}`
- `panel:hidden`
- `data:groups-changed`
- `data:items-changed {group_id}`
- `settings:changed`
- `permission:changed {status}` (macOS)

A failed capture does not open the panel. Rust sends the OS notification (Notification Toast failure) directly.

**Binding technical rules**
- **TC-1** Rust MUST own all persistent and cross-window state. A window store MUST hold only view state and MUST refetch after a `data:*` or `settings:changed` event. Windows MUST NOT message each other.
- **TC-2** Shortcuts, capture, paste and clipboard access MUST run in Rust. The webview MUST NOT have clipboard permissions.
- **TC-3** Only one capture or paste sequence MAY run at a time. A hotkey pressed during a sequence is ignored.
- **TC-4** Capture and paste sequences MUST NOT run on the Tauri event-loop thread, because they wait up to about 1.5 s. Window show and hide MUST be dispatched to the main thread.
- **TC-5** The previous clipboard MUST be restored by a scope guard that runs on success, on error and on panic. The release profile MUST keep `panic = "unwind"`. Tauri's size guide suggests `abort`, which would skip the guard.
- **TC-6** The restore MUST be skipped if the clipboard change counter moved after our write, because someone else wrote.
- **TC-7** The clipboard snapshot and the pending capture MUST live in memory only, never on disk or in logs.
- **TC-8** The webview MUST NOT receive item HTML. Item payloads carry plain text plus `has_rich_text`.
- **TC-9** No command MAY accept arbitrary text to paste or inject. Paste takes an item id or an AI `result_id` held in Rust.
- **TC-10** `core` and `ai` MUST NOT depend on `tauri`. The app crate adapts them into commands.
- **TC-11** Generated bindings MUST NOT be hand-edited. CI regenerates them and fails on a diff.
- **TC-12** Every command error MUST be a typed discriminated union (C3), so the UI can render the designed error and missing-prerequisite states.
- **TC-13** Timing constants (1.3) MUST be named per-OS constants. They are calibration knobs tuned in R0.
- **TC-14** Retention, soft-delete purge and usage purge MUST run at startup and on writes. There are no background timers.
- **TC-15** AI requests MUST contain only the prompt and item plain text. Items from a group marked Never send to AI MUST be refused in Rust (`AiLocked`), not only hidden in the UI.
- **TC-16** AI error classification MUST be a pure function `(status, headers, body) -> AiError`, following the rules and recorded bodies in the AI provider research.

### 1.2 Data model (SQLite via rusqlite, schema v1)

**`groups`**

| Column | Type | Rule |
| --- | --- | --- |
| `id` | INTEGER PRIMARY KEY | |
| `name` | TEXT NOT NULL | Trimmed, non-empty, `UNIQUE COLLATE NOCASE` |
| `position` | INTEGER NOT NULL | Sidebar order; assumed to also drive ⌘1–⌘5 in Copy to group (Q18) |
| `never_send_to_ai` | INTEGER NOT NULL DEFAULT 0 | `CHECK (never_send_to_ai IN (0,1))` |
| `created_at` | INTEGER NOT NULL | Unix ms |

**`items`**

| Column | Type | Rule |
| --- | --- | --- |
| `id` | INTEGER PRIMARY KEY | |
| `group_id` | INTEGER NOT NULL | `REFERENCES groups(id) ON DELETE CASCADE` |
| `kind` | TEXT NOT NULL | `CHECK (kind IN ('text','rich_text','link','image'))` |
| `plain_text` | TEXT | Required unless `kind='image'` (CHECK) |
| `html` | TEXT | Sanitised; present only when `kind='rich_text'` (CHECK) |
| `image_file` | TEXT | File name under `images/`; present only when `kind='image'` (CHECK) |
| `image_width`, `image_height` | INTEGER | Image only |
| `byte_size` | INTEGER NOT NULL | Stored size, used for limits and diagnostics |
| `content_hash` | BLOB NOT NULL | SHA-256 of the kind tag plus the plain text or PNG bytes |
| `source_app` | TEXT | NULL when unknown (Wayland) |
| `captured_at` | INTEGER NOT NULL | Unix ms; recapture updates it, which moves the item to the top |
| `deleted_at` | INTEGER | Set by delete; Undo clears it; purged afterwards |

**`settings`**: a single row, `id INTEGER PRIMARY KEY CHECK (id=1)` and `json TEXT NOT NULL`. It deserialises into a typed Rust `Settings` struct with serde defaults, so adding a setting means adding a field, not a migration. It holds theme, shortcuts, Vim mode, launch at login, usage stats toggle, retention limit, auto-delete, AI provider, model overrides and prompt template.

**`usage_daily`**
- Columns: `day` TEXT (UTC `YYYY-MM-DD`), `metric` TEXT (from a Rust enum, never user data), `bucket_ms` INTEGER (0 for plain counts, otherwise the histogram upper bound: 50, 100, 150, 250, 500, 1000, 2000, max), `count` INTEGER.
- Primary key: `(day, metric, bucket_ms)`.
- Counting per day into histogram buckets gives percentiles without storing per-action timestamps.

**Indexes**
- `items(group_id, captured_at DESC) WHERE deleted_at IS NULL` serves panel lists and retention.
- `UNIQUE items(group_id, content_hash) WHERE deleted_at IS NULL` enforces dedupe. Upsert with `ON CONFLICT ... DO UPDATE SET captured_at, source_app, html`.
- Search starts as `plain_text LIKE ?` with wildcards escaped. Its known ceiling: case folding is ASCII-only and it is a full scan. The upgrade path is FTS5 with the trigram tokenizer, only if NFR-10 fails.

**Data rules**
- **DM-1** Recapturing identical content into the same group MUST update the existing row, not insert a new one.
- **DM-2** After each insert, items beyond the retention limit (default 200 per group, oldest `captured_at` first) MUST be deleted in the same transaction. Their image files are deleted after commit.
- **DM-3** An image file MUST be written as `images/<uuid>.png.tmp`, then renamed, before its row is inserted. At startup, files with no row MUST be deleted.
- **DM-4** Soft-deleted rows MUST be purged at startup and on the next write once they are older than the Undo window (about 6 s).
- **DM-5** `usage_daily` rows older than 90 days MUST be purged at startup. When the toggle is off, nothing is written.
- **DM-6** Auto-delete is off by default. Its rule is open (Q7).

**Connection:** one `rusqlite::Connection` behind a `Mutex` in Tauri state. The ceiling is a single global lock, which is fine for one user. Every open sets the pragmas `foreign_keys=ON`, `journal_mode=WAL`, `synchronous=NORMAL` and `busy_timeout=2000`.

**Migrations**
- Ordered SQL files in `crates/core/migrations/NNNN_name.sql`, embedded with `include_str!`. The version is tracked with `PRAGMA user_version`.
- At startup: copy the DB to `lazyclipboard.db.bak`, then apply each newer file and bump `user_version` inside one transaction. Migrations are forward-only.
- If `user_version` is newer than the app knows (a downgrade), refuse to open and show an error. Do not write.
- Tests: migrate an empty DB to latest; migrate a v1 fixture DB to latest.

**Files on disk.** All paths come from Tauri's path API, which keys them by bundle identifier (Q8).
- `<app data>/lazyclipboard.db` (plus `-wal` and `-shm`)
- `<app data>/lazyclipboard.db.bak`
- `<app data>/images/*.png`
- `<app log dir>/` for logs

### 1.3 Shortcut, capture and paste sequences

**Copy to group (capture)**

| # | Step | macOS | Windows | Linux X11 | Linux Wayland |
| --- | --- | --- | --- | --- | --- |
| 1 | Hotkey fires | Carbon hotkey (global-shortcut plugin) | `RegisterHotKey` / `WM_HOTKEY` (plugin) | `XGrabKey` (plugin) | Portal `GlobalShortcuts` `Activated` signal (ashpd) |
| 2 | Permission guard | `AXIsProcessTrusted()`; if false, show revoked or needed state and stop | Check whether the target is elevated; if so, show the "runs as administrator" error and stop | none | none |
| 3 | Record source | `NSWorkspace.frontmostApplication` | `GetForegroundWindow`, process id, exe name | `_NET_ACTIVE_WINDOW`, `WM_CLASS` | unknown, stored as NULL |
| 4 | Wait for the user to release the shortcut's modifiers (max 1000 ms) | `CGEventSourceFlagsState` | `GetAsyncKeyState` | `XQueryKeymap` | n/a |
| 5 | Snapshot the clipboard and its change counter | `NSPasteboard` `changeCount` plus all item types and data | `GetClipboardSequenceNumber` plus readable formats | XFixes owner-change subscription plus readable targets | skipped |
| 6 | Inject the copy chord (layout-aware key for "C") | ⌘C via `CGEventPost` | Ctrl+C via `SendInput` | Ctrl+C via XTest | not possible (Q1) |
| 7 | Poll every 10 ms, up to 500 ms, for the counter to change | `changeCount` | sequence number | XFixes event | n/a |
| 8 | No change: notify "Nothing selected" and stop (clipboard untouched) | same on all | | | |
| 9 | Read flavours: PNG or TIFF image, else HTML plus plain, else plain | NSPasteboard | clipboard API | selection targets | n/a |
| 10 | Restore the snapshot (scope guard, TC-5) | same on all | | | n/a |
| 11 | `core::classify` (pure): image (encode PNG, ≤ 10 MB) > rich text (ammonia-sanitised HTML plus plain) > link (trimmed text matches `^https?://\S+$`) > text; anything else is `Unsupported` | same on all | | | |
| 12 | Hold the pending capture in memory; show the panel in copy mode | NSPanel | panel (1.5) | panel (1.5) | normal window |
| 13 | User picks a group: `capture_save`, upsert (DM-1), retention trim (DM-2), hide, OS success notification | same on all | | | |

**Paste from group**

| # | Step | macOS | Windows | Linux X11 | Linux Wayland |
| --- | --- | --- | --- | --- | --- |
| 1 | Hotkey fires; record target | pid | HWND, plus elevation check | window id | none |
| 2 | Show the panel in paste mode | Non-activating NSPanel | See R1 | Panel takes focus | Panel takes focus |
| 3 | User picks an item and presses ↵ (default), ⇧↵ (plain) or ⌘↵ (Paste all) | | | | |
| 4 | `paste_item` / `paste_all` / `paste_ai_result` | | | | |
| 5 | Permission guard (as in capture step 2) | AX check | elevation check | none | none |
| 6 | Snapshot the clipboard | yes | yes | yes | skipped |
| 7 | Write the item with transient and exclude-from-history markers | `public.utf8-plain-text`, `public.html`, `public.png` | `CF_UNICODETEXT`, "HTML Format", image | `UTF8_STRING`, `text/html`, `image/png` | Write while the panel still has focus |
| 8 | Hide the panel and return focus | Order out; the target never lost active status | `SetForegroundWindow(target)` if the panel activated | `_NET_ACTIVE_WINDOW` request | Hide, show "Press Ctrl+V", stop |
| 9 | Wait for modifier release (the user may still hold ⇧ or ⌘ from ⇧↵ or ⌘↵), then a 50 ms focus settle | yes | yes | yes | n/a |
| 10 | Inject the paste chord | ⌘V | Ctrl+V | Ctrl+V (terminals need Ctrl+Shift+V, see R4) | n/a |
| 11 | Wait the restore delay, then restore if the counter still equals our write (TC-6) | yes | yes | yes | n/a; the previous clipboard is replaced |
| 12 | Record usage metrics | | | | |

**Timing constants.** Per OS; the starting values are guesses, tuned in R0.
- `MODIFIER_RELEASE_TIMEOUT` = 1000 ms
- `CLIPBOARD_POLL_INTERVAL` = 10 ms
- `CLIPBOARD_CHANGE_TIMEOUT` = 500 ms
- `FOCUS_SETTLE_DELAY` = 50 ms
- `RESTORE_DELAY` = 250 ms

Why the modifier wait matters: if the user still holds ⌘⌥ or Win+Shift when we inject, the app sees ⌘⌥C or Win+Shift+Ctrl+C. If ⇧ is held on paste, many macOS apps run ⌘⇧V (Paste and Match Style).

### 1.4 Key crates and libraries

**Rust**

| Crate | Use | Status |
| --- | --- | --- |
| `tauri` 2 | Shell, windows, tray (`tray-icon`), events, asset protocol for image thumbnails | Official |
| `tauri-plugin-global-shortcut` | Hotkeys on macOS, Windows, X11 | Official. Built on `global-hotkey`, which to my knowledge is X11-only on Linux [verify] |
| `ashpd` | xdg-desktop-portal client (`GlobalShortcuts`) for Wayland | Community, widely used [verify the GlobalShortcuts API in the current version] |
| `tauri-nspanel` | Turn the panel into a non-activating NSPanel | Community, single maintainer. I believe it ships as a git dependency on a Tauri 2 branch, not a crates.io release [verify]. Pin to a commit |
| `tauri-plugin-single-instance`, `-autostart`, `-updater`, `-process`, `-notification`, `-dialog`, `-log` | Single instance, launch at login, updates, relaunch, copy notification, save dialog for Export diagnostics, logs | Official |
| `tauri-specta`, `specta`, `specta-typescript` | Generated TS types for commands and events | Community. tauri-specta for Tauri 2 has shipped as long-running release candidates [verify stability]. Fallback: `ts-rs` plus a thin typed `invoke` wrapper |
| `arboard` | Clipboard text, image and HTML write | Community (1Password). Reading HTML and enumerating all formats for a full snapshot may be missing [verify]. Alternative: `clipboard-rs` [verify maturity]. Last resort: direct OS calls |
| `enigo` | Inject ⌘C/⌘V and Ctrl+C/Ctrl+V | Community. Its API changed a lot in 0.2, and the Linux backend may need `libxdo` depending on features [verify]. Direct `CGEventPost` / `SendInput` / XTest is a small fallback (four key events per chord) |
| `objc2`, `objc2-app-kit`, `objc2-foundation` | NSWorkspace, NSPasteboard, modifier state | Community, maintained; used by Tauri itself |
| `windows` | Foreground window, `SendInput`, `SetWindowLongPtrW`, clipboard sequence, `RegisterClipboardFormatW`, token elevation | Official Microsoft |
| `x11rb` | EWMH atoms, XTest, XFixes, `XQueryKeymap` | Community, maintained |
| (none) | `AXIsProcessTrusted` / `AXIsProcessTrustedWithOptions` | Direct FFI to ApplicationServices; a few lines, no crate |
| `rusqlite` (`bundled`) | SQLite compiled in | Community, standard |
| `keyring` v3 | OS keychain | Community. I believe v3 falls back to an in-memory mock store unless platform features (`apple-native`, `windows-native`, a Secret Service feature) are enabled [verify feature names]. The smoke checklist includes "key survives an app restart" |
| `reqwest` (no default features; rustls, json), `serde`, `serde_json` | Provider HTTP | Standard. The rustls feature names differ across versions [verify at install] |
| `ammonia` | Sanitise captured HTML | Community, maintained |
| `png`, `sha2`, `regex`, `uuid` | PNG encode, content hash, link detection, image file names | Standard |
| `wiremock` | Request-shape tests per provider | Community. Error classes are tested on the pure classifier with the recorded bodies, no HTTP server needed |

**UI**
- App: `@tauri-apps/api`, whose `mocks` module provides `mockIPC` for tests.
- Build and UI: `react`, `vite`, `typescript`, `tailwindcss` v4 with `@tailwindcss/vite`, Radix primitives, `zustand`, `lucide-react` (the design spec's only icon library).
- Font: Inter, bundled locally (for example `@fontsource/inter`). No font CDN.
- Tests: `vitest`, `@testing-library/react`, `@testing-library/user-event`, `jsdom`, `@playwright/test`, `@axe-core/playwright`.
- Storybook 10 in `apps/storybook`: `storybook`, `@storybook/react-vite`, `@storybook/addon-a11y`, `@storybook/addon-vitest`, `@vitest/browser-playwright`. Stories match `../../../packages/components/src/**/*.stories.@(ts|tsx)` and `../../ui/src/windows/**/*.stories.@(ts|tsx)`. `.storybook/preview.ts` imports the token CSS and the Tailwind entry, installs the C9 `mockIPC` fixture, and offers a Light and Dark toolbar toggle. Turborepo tasks: `storybook` (persistent, no cache), `build:storybook` (outputs `storybook-static/**`), `test:storybook` (`vitest --project=storybook`). Versions and install commands are checked against current docs when the harness task runs.

**Tooling:** `pnpm`, `turbo`, `clippy`, `rustfmt`, `cargo-deny` (advisories and licences), and ESLint with `react/no-danger`.

### 1.5 Per-OS differences

| Concern | macOS | Windows | Linux X11 | Linux Wayland |
| --- | --- | --- | --- | --- |
| Default chords (design spec) | ⌘⌥C / ⌘⌥V | Win+Shift+C / Win+Alt+V | Ctrl+Alt+C / Ctrl+Alt+V | Chosen through the portal; the desktop may show its own bind dialog [verify] |
| Shortcut conflict detection | Partial: registration may not report another app's claim [verify] | Reliable: `RegisterHotKey` fails | Reliable-ish: `BadAccess` from `XGrabKey` | Owned by the desktop |
| Permission | Accessibility (needed for `CGEventPost`); onboarding plus revoked state | None | None | None |
| Panel | NSPanel non-activating, floating level, joins all Spaces plus fullscreen auxiliary; transparent corners need `macOSPrivateApi` | `WS_EX_NOACTIVATE \| WS_EX_TOOLWINDOW`, topmost; may need the focus-restore fallback (R1) | Utility type hint, keep-above, skip taskbar; takes focus and restores the previous window; focus-stealing prevention may need the hotkey event timestamp [verify] | Normal window; the compositor decides position and stacking |
| Centring on active display | Yes | Yes | Yes | No, the compositor places it |
| Capture injection | ⌘C | Ctrl+C (blocked into elevated apps) | Ctrl+C (terminals: SIGINT, R4) | None (Q1) |
| Clipboard change signal | `changeCount` | `GetClipboardSequenceNumber` | XFixes events (no counter) | n/a |
| Paste | ⌘V plus restore | Ctrl+V plus restore | Ctrl+V plus restore; our process serves the restored data | Item left on clipboard; user presses Ctrl+V |
| Hide from other clipboard history | `org.nspasteboard.TransientType` (a convention; only some managers honour it) | `ExcludeClipboardContentFromMonitorProcessing`, `CanIncludeInClipboardHistory`=0, `CanUploadToCloudClipboard`=0 | No widely honoured convention known [verify] | Same as X11 |
| Source app | Frontmost application name | Exe name of the foreground window | `WM_CLASS` | Unknown |
| Keychain | Keychain | Credential Manager | Secret Service (gnome-keyring, KWallet); may be absent, which disables AI with a missing-prerequisite state | Same as X11 |
| Tray | Menu bar | Notification area | AppIndicator; invisible on stock GNOME without the extension | Same as X11 |
| Launch at login | autostart plugin | autostart plugin | XDG autostart | XDG autostart |
| Webview | WKWebView | WebView2 (Evergreen) | WebKitGTK 4.1 | WebKitGTK 4.1 |
| Package | Signed and notarized `.dmg` | Unsigned installer (SmartScreen warning) | AppImage and `.deb` | AppImage and `.deb` |

### 1.6 Shared contracts for Sprint 0 (so parallel worktrees can start)

| ID | Contract | Lives in | Unblocks |
| --- | --- | --- | --- |
| C1 | Domain types: `GroupId`, `ItemId`, `Group`, `ItemContent` union (Text, RichText, Link, Image), `ItemPreview` (what the webview receives: plain text, `has_rich_text`, image asset URL), `Settings` with per-OS default chords, `UsageMetric` enum | `crates/core` | All windows, `os`, `ai` |
| C2 | Command and event signatures from 1.1, registered with stubs that return fixture data | `src-tauri/src` (owner: Q10) | UI windows build against stubs |
| C3 | Error unions: `CommandError` (NotFound, Validation{field}, Conflict, PermissionMissing, TargetElevated, NothingSelected, Unsupported, TooLarge, KeychainUnavailable, AiLocked, Ai(AiError), Internal); `AiError` exactly as in the AI provider research | `core`, `ai` | Designed states everywhere |
| C4 | Schema v1 SQL plus a seed fixture (20 groups × 200 items, including rich, link and image items) | `crates/core` | Core issues, performance tests, UI fixtures |
| C5 | `os` API: `frontmost_target()`, `wait_modifiers_released(timeout)`, `send_copy_chord()`, `send_paste_chord(target)`, a `Clipboard` trait (`snapshot`, `restore`, `change_count`, `read_flavours`, `write_item`), `panel_show`/`panel_hide`, `accessibility_status()`, plus a fake `Clipboard` for tests. The trait has two implementations: real and fake | `crates/os` | Capture and paste issues per OS, restore tests |
| C6 | `AiProvider` trait, `AiError`, `classify_error(status, headers, body)` signature, recorded fixture bodies | `crates/ai` | Three provider issues in parallel |
| C7 | CSS variable names generated from the design spec front matter (colour per mode, type, spacing, radius, elevation, motion) | `packages/tokens` | Components, windows |
| C8 | Window labels, Vite multi-page entries, per-window capability files, CSP | `src-tauri`, `apps/ui` | Windows |
| C9 | IPC mock harness (`mockIPC` backed by the C4 fixture) for Vitest, Storybook and Playwright | `apps/ui` shared (owner: Q10) | Component and screen tests |
| C10 | CI skeleton: 3-OS build, lint and test; bindings-diff check; SHA-pinned actions; artifact size report | `.github` | Every PR |
| C11 | Per-OS native smoke checklist template | `docs` | Every native issue |

### 1.7 R0 spike: exact pass/fail criteria

**Scope:** one global shortcut, one panel, one paste of a fixed sentinel string, on all three OSes.

**Target apps per OS:**
- macOS: TextEdit, Safari textarea, VS Code, Terminal
- Windows: Notepad, Edge textarea, VS Code, Windows Terminal, plus an elevated Notepad (behaviour recorded only)
- Linux X11 (Xorg session): Text Editor, Firefox textarea, VS Code, GNOME Terminal (behaviour recorded only)

**Criteria.** macOS, Windows and X11 must each pass all of these:

| ID | Criterion | Pass |
| --- | --- | --- |
| R0-1 | The shortcut opens the panel from each target app, and no character from the chord leaks into the target (for example ⌥C typing "ç") | 20/20 per app |
| R0-2 | The panel receives typed characters, ↑/↓, ↵ and Esc | 20/20 |
| R0-3 | Focus. **Strict:** the target stays the active app while the panel is open. **Acceptable:** the target regains focus with the caret unchanged within 100 ms of hide, with no taskbar flash or Dock bounce. macOS MUST pass Strict. Windows and X11 may pass on Acceptable, but only with owner sign-off (Q3) | As stated |
| R0-4 | ↵ inserts the sentinel at the original caret in each target app (the Linux terminal is recorded, not blocking) | 20/20 per app |
| R0-5 | After paste the clipboard equals its prior content: a text sentinel 20/20, an image 5/5 | As stated |
| R0-6 | Shortcut to panel ready: p95 ≤ 150 ms over 50 opens (Rust timestamp to the webview's first-frame ack). ↵ to text visible: ≤ 300 ms (60 fps screen-recording spot check, 5×) | As stated |
| R0-7 | The panel appears over a fullscreen app (macOS fullscreen Space; Windows F11 browser) | 5/5 |
| R0-8 | R0-4 repeated with ⇧ still held on ↵, and with a non-QWERTY layout (Dvorak) and a non-Latin layout (Russian) | 5/5 each |
| R0-9 | macOS only: with Accessibility off, the shortcut shows the permission state instead of failing silently. After granting, paste works. Whether a relaunch is needed is recorded | Pass/record |
| R0-10 | The spike builds in the 3-OS CI matrix | Green |

**Recorded, not pass/fail:** idle RSS, idle CPU, installer size, cold start, tuned values of the 1.3 constants.

**Wayland** (GNOME and KDE Plasma, latest stable):
- W1: the portal shortcut fires 20/20.
- W2: the panel takes keys.
- W3: ↵ puts the sentinel on the clipboard, and a manual Ctrl+V pastes it 20/20.
- W4: record where the compositor places the panel.
- A Wayland failure does NOT trigger the Electron fallback, because Electron has the same limits.

**Fallback rule.** If R0-1 to R0-5 fail on macOS or Windows within the timebox (Q13), classify the cause:
- A Tauri or plugin cause (webview, plugin, window API): switch to Electron.
- An OS cause (focus rules, UIPI, injection): Electron would fail the same way. Amend the interaction in an ADR instead.

## 2. Non-functional requirements

| ID | Requirement (MUST) | Target | How measured |
| --- | --- | --- | --- |
| NFR-1 | Panel open, warm (panel pre-created) | Hotkey to panel painted and accepting keys: p95 ≤ 150 ms | `panel_open_ms` histogram in `usage_daily`, read from Export diagnostics after 50 opens on a reference machine per OS |
| NFR-2 | Startup | Process start to shortcuts registered and tray visible: ≤ 2 s | Log timestamps, launch-at-login run on each OS |
| NFR-3 | Capture latency | Modifiers released to Copy to Group panel showing content: p95 ≤ 300 ms. Hard timeout 500 ms leads to "Nothing selected" | `capture_ms` histogram |
| NFR-4 | Paste latency | ↵ to paste chord sent: p95 ≤ 150 ms internal. ↵ to text visible: ≤ 300 ms | `paste_ms` histogram; screen-recording spot check in the smoke checklist |
| NFR-5 | Clipboard restore, including crash safety | Restored (or deliberately skipped per TC-6) in 100% of sequences that end in success, error or Rust panic. A hard kill loses at most the one in-flight snapshot, which is never written to disk | `core`/`os` unit tests with the fake clipboard, injecting a failure and a panic at every step; a CI check that the release profile is not `panic = "abort"`; smoke sentinel check 20/20 |
| NFR-6 | Idle CPU | Average ≤ 0.1% over 10 min with the panel hidden and other windows closed. No timers, no polling outside a capture window | Activity Monitor, Task Manager, `top` |
| NFR-7 | Idle memory | Sum of app and webview processes ≤ 200 MB. Revise once the R0 baseline is known | Activity Monitor, Task Manager process group, `smem` |
| NFR-8 | Memory stability | ≤ 20 MB RSS growth after 500 panel open/close cycles | Scripted hotkey loop in the smoke run |
| NFR-9 | Installer size | `.dmg` ≤ 20 MB (≤ 40 MB if universal), Windows installer ≤ 15 MB (WebView2 via bootstrapper), `.deb` ≤ 15 MB, AppImage ≤ 120 MB (bundles WebKitGTK; unverified, set from R0) | CI prints artifact sizes and fails over budget |
| NFR-10 | DB performance and size | Item size: image PNG ≤ 10 MB; text limit Q5. With the 20 × 200 seed: group list ≤ 10 ms p95, search ≤ 50 ms p95, DB file ≤ 20 MB (text items averaging 2 KB) | `#[ignore]` perf test on the C4 seed, run per release |
| NFR-11 | Accessibility | WCAG 2.2 AA (contrast already measured in the design spec); zero serious or critical axe violations per screen; every screen operable by keyboard only; text ≥ 12 px except overlines; motion off under `prefers-reduced-motion` | Storybook a11y test per component and screen state, `@axe-core/playwright` plus keyboard-only Playwright test per window flow; manual screen-reader pass per OS (scope Q16) |
| NFR-12 | Offline | Every non-AI feature works with networking off. AI returns `Network` within the 10 s connect timeout (60 s total request timeout, named constants). A failed update check is silent and logged. No other connections are made | Smoke checklist with networking off; connection monitor (Little Snitch, `nettop`, Resource Monitor) during one smoke run |
| NFR-13 | Data crash safety | Killing the process at any point leaves `PRAGMA integrity_check` = ok and no orphan image after the next start | Core test simulating a crash between the file write and the row insert; startup sweep test |
| NFR-14 | Single instance | A second launch never creates a second process, tray icon or hotkey registration | Smoke checklist |

## 3. Security and privacy

**Threats and mitigations**

| ID | Threat | Mitigation |
| --- | --- | --- |
| T1 | Captured HTML runs as code inside the app | The HTML never reaches the webview (TC-8). Previews are React text nodes. ESLint `react/no-danger`. HTML is sanitised with ammonia at capture, which also protects the apps it is pasted into. Strict CSP: `'self'` only, no `unsafe-eval`, no remote origins. Images are re-encoded to PNG in Rust; the asset protocol is scoped to `images/` only |
| T2 | A compromised webview (malicious UI dependency) drives IPC | Per-window Tauri capabilities (1.1 table). No command injects arbitrary text (TC-9). Key commands are write-only. No shell or filesystem plugin is exposed to JS |
| T3 | API key exposure | Keys live only in the keychain, one entry per provider (`lazyclipboard.ai.<provider>`), read per request. They are never returned to the webview (status only), are wrapped in a type whose `Debug` prints `***`, and are never logged. The Gemini key goes in a header, not the URL. Limit to document: on Windows and Linux (unlocked Secret Service), other processes of the same user can read the credential. Same-user malware is out of scope |
| T4 | Logs or diagnostics leak content | Logging allowlist: no item text or HTML, group names, source apps, prompts, AI output, HTTP bodies or keys. Errors are logged by variant name. The panic hook logs `file:line` only, not the payload. The Export diagnostics JSON has a fixed schema, checked by a test against an allowlist |
| T5 | Items stored in plaintext (no DB encryption in MVP) | App data directory set to owner-only (0700) on macOS and Linux. The README states where data lives and that it is unencrypted. Retention limits exposure. Password-manager items: Q12 |
| T6 | Our transient clipboard writes recorded by other clipboard managers or Windows cloud clipboard | Every item write sets the exclusion formats and markers from 1.5 |
| T7 | Tampered update | The Tauri updater verifies the signature against the public key compiled into the app, over HTTPS from GitHub Releases. macOS is notarized. Releases publish SHA-256 checksums. The private key has an offline backup (R14) |
| T8 | Supply chain | Lockfiles committed. `cargo deny` (RustSec advisories, licences) and `pnpm audit` in CI. Actions pinned by SHA, with Dependabot for actions. Git dependencies (tauri-nspanel) pinned by commit and reviewed on bump. pnpm lifecycle scripts limited to an allowlist [verify pnpm default]. Few dependencies |
| T9 | Fork PRs or agents reach release secrets | CI uses `pull_request`, never `pull_request_target`. Default workflow `permissions: contents: read`. Release runs only on `v*` tags in the protected `release` environment with owner approval. A tag ruleset limits `v*` to the owner. CODEOWNERS on `.github`. Secret scanning and push protection are on. Agents push branches only |
| T10 | Over-broad OS permissions | macOS asks only for Accessibility (not Input Monitoring or Screen Recording) and uses no extra hardened-runtime entitlements unless R0 proves one is needed. Windows runs `asInvoker` and never requests admin. Linux uses no `uinput` or `input` group |
| T11 | AI egress beyond consent | Only on explicit action. Only the prompt plus item plain text (TC-15). Never send to AI enforced in Rust. The "Sends N items" note names the provider. Gemini uses the stateless `generateContent` |
| T12 | OS notifications expose content | Copy notifications show group name and item type, not content (proposed) |

**What is stored where**

| Data | Location | Protection |
| --- | --- | --- |
| Groups, items, settings, usage counters | `<app data>/lazyclipboard.db` | Owner-only directory, unencrypted |
| Images | `<app data>/images/*.png` | Same |
| Pre-migration backup | `<app data>/lazyclipboard.db.bak` | Same |
| API keys | OS keychain, one entry per provider | OS keychain |
| Logs | `<app log dir>`, rotating and size-capped | No content by rule (T4) |
| Clipboard snapshot, pending capture, AI result | Memory only | Dropped at the end of the sequence or on panel hide |

**What leaves the device**
- **AI request**, on explicit action only: prompt plus plain text, sent to the one configured provider, with the key in a header.
- **Test connection**: `GET /models` to that provider.
- **Update check**: HTTPS to GitHub Releases. It reveals IP address and app version to GitHub (Q11).
- **Nothing else:** no telemetry, crash reports, fonts or CDNs. Usage counters leave only if the user exports them and shares the file.

## 4. Dependencies and integration points

**OS APIs**
- **macOS:**
  - Carbon `RegisterEventHotKey` (through the plugin), NSPanel (tauri-nspanel)
  - `NSPasteboard`, `CGEventPost`, `CGEventSourceFlagsState`
  - `AXIsProcessTrustedWithOptions`, `NSWorkspace`, Keychain Services
  - LaunchAgent (autostart plugin), UserNotifications (copy notification; the user must allow it)
  - The System Settings deep link `x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility` [verify on current macOS]
- **Windows:**
  - `RegisterHotKey`, `SetWindowLongPtrW`/`ShowWindow(SW_SHOWNOACTIVATE)`, `GetForegroundWindow`/`SetForegroundWindow`
  - `SendInput`, `GetAsyncKeyState`, the clipboard API plus `GetClipboardSequenceNumber` and the exclusion formats
  - `OpenProcessToken`/`GetTokenInformation(TokenElevation)`, Credential Manager
  - HKCU Run key (autostart plugin), WebView2 Evergreen runtime
- **Linux X11:**
  - `XGrabKey`, XTest, XFixes, EWMH (`_NET_ACTIVE_WINDOW`, `_NET_WM_STATE_ABOVE`, `SKIP_TASKBAR`)
  - Secret Service over D-Bus, AppIndicator/StatusNotifierItem, `org.freedesktop.Notifications`, XDG autostart
  - Runtime packages: `libwebkit2gtk-4.1`, `libayatana-appindicator3` [verify exact package names per distro]
- **Linux Wayland:** `org.freedesktop.portal.GlobalShortcuts` and the Wayland clipboard while focused. No injection.

**Tauri plugins**
- Official: global-shortcut, single-instance, autostart, updater, process, notification, dialog, log.
- Community: tauri-nspanel, tauri-specta.

**Vendor endpoints** (exact facts in the AI provider research; not re-derived here)
- Anthropic: `POST /v1/messages`, `GET /v1/models`
- OpenAI: `POST /v1/responses`, `GET /v1/models`
- Gemini: `POST /v1beta/models/{model}:generateContent`, `GET /v1beta/models`

**GitHub**
- Releases host the artifacts and the updater manifest (`latest.json`).
- Actions, all SHA-pinned: `actions/checkout`, `actions/setup-node`, `pnpm/action-setup`, `dtolnay/rust-toolchain`, `Swatinem/rust-cache`, `tauri-apps/tauri-action`, `actions/upload-artifact`.
- `release` environment secrets (names per Tauri 2 docs [verify]):
  - `TAURI_SIGNING_PRIVATE_KEY` and its password
  - `APPLE_CERTIFICATE`, `APPLE_CERTIFICATE_PASSWORD`, `APPLE_SIGNING_IDENTITY`
  - Either `APPLE_API_ISSUER`/`APPLE_API_KEY` or `APPLE_ID`/`APPLE_PASSWORD`/`APPLE_TEAM_ID`
- Linux release builds run on the oldest available Ubuntu runner, for glibc compatibility.

**Apple:** paid Developer Program, Developer ID Application certificate, notary service.

**Microsoft:** SmartScreen reputation while the beta is unsigned.

## 5. Risks

| ID | Risk | Likelihood | Impact | Mitigation | Owner type |
| --- | --- | --- | --- | --- | --- |
| R1 | **Windows:** a `WS_EX_NOACTIVATE` window cannot get keyboard focus, because keys go to the foreground window. The panel then can't be searched or navigated | High | High | Test first in R0. Fallback: activate the panel, remember the HWND, and `SetForegroundWindow(target)` on hide (should be allowed while we hold the foreground [verify]). A low-level keyboard hook is rejected (AV heuristics, complexity). Needs sign-off (Q3) | R0 spike, `os` |
| R2 | **macOS:** NSPanel through a community plugin. Key without activation in WKWebView, showing over fullscreen Spaces, and breakage on Tauri upgrades | Medium | High | Pin the commit; R0-3 and R0-7; fork or vendor it into `os` if it goes unmaintained | `os` |
| R3 | **Capture races:** the clipboard isn't written yet (Electron, Office, remote desktop); held modifiers change the chord; non-US layouts move "C" | High | High | Wait for modifier release, poll the change counter, layout-aware key lookup, per-OS constants tuned in R0, R0-8 | `os` |
| R4 | **Terminals:** injected Ctrl+C in a Linux terminal (or a Windows console with nothing selected) sends SIGINT and can kill the user's running command. Ctrl+V doesn't paste in Linux terminals | High on Linux | High | Q2. R0 records current behaviour | Product owner, `os` |
| R5 | **Wayland:** GlobalShortcuts portal support varies (KDE yes; recent GNOME reportedly; wlroots/sway reportedly no [verify]). No capture injection. The compositor places the panel. The tray is hidden on stock GNOME | High | Medium | Onboarding Wayland notice. Proposed fallback: `lazyclipboard --paste` / `--copy` forwarded through single-instance and bound as a custom shortcut in desktop settings. Q1 | `os`, product owner |
| R6 | **Windows elevated apps (UIPI):** `SendInput` into an elevated window fails and reports no error, so capture says "Nothing selected" and paste does nothing | Medium | Medium | Check target elevation before injecting (an access-denied result also means elevated) and show a specific message. Document it | `os` |
| R7 | **macOS permission:** revoked by the user, or the grant is lost when the code signature changes (every ad-hoc dev build, a Team ID change). `CGEventPost` then silently does nothing | High | High | Check before every injection and on window focus, then show the revoked state. Sign dev builds with one stable local certificate. Never change the bundle id or Team ID. R0-9 | `os`, maintainer |
| R8 | **Apple notarization:** paid program; outages or hardened-runtime rejections block a release; certificate expiry | Medium | Medium | Notarize a test tag in Sprint 0, use API-key auth, record the certificate expiry date | Release/CI |
| R9 | **Tauri plugin maturity:** community NSPanel, tauri-specta release candidates, no Wayland in global-shortcut, WebKitGTK quirks (transparency, GPU drivers) | Medium | Medium | Every plugin sits behind an `os` function, so a swap stays local. Pin versions. Validate in R0 | `os`, maintainer |
| R10 | **Restore fidelity and timing:** formats we can't read (files, app-private) are lost; restoring too early makes the target paste the old content; data we restore on X11 disappears when we quit | Medium | Medium | `RESTORE_DELAY` tuned per OS; TC-6; macOS copies every pasteboard type; Q4 | `os` |
| R11 | **Unsigned Windows beta:** SmartScreen warning; antivirus heuristics flag global hotkey plus `SendInput` as keylogger-like | Medium | Medium | Publish checksums, report false positives, sign later (Q14) | Maintainer |
| R12 | Screen readers may not announce a non-activating panel | Medium | Medium | Manual screen-reader pass in the smoke checklist; Q16 | UI, `os` |
| R13 | **Parallel worktrees:** conflicts in `Cargo.lock`, `pnpm-lock.yaml`, command registration, capabilities and bindings; contract drift | High | Medium | Sprint 0 contracts, CI bindings-diff check, one owner for `src-tauri/src` (Q10), rebase before merge | Maintainer |
| R14 | **Updater private key** lost (no more updates to existing installs) or leaked | Low | High | Encrypted offline backup; password kept as a separate secret; protected environment | Maintainer |
| R15 | Migration fails after an update, or a downgrade meets a newer schema | Low | High | Backup before migrating, transactional migration, refuse a newer schema with a clear error | `core` |
| R16 | Corporate TLS interception or proxies break AI calls | Low | Low | Use OS-native roots with rustls; honour proxy environment variables [verify reqwest behaviour] | `ai` |
| R17 | Default model IDs are unverified and change often | Medium | Low | One constant per provider, a model override, a real-key check in the provider issues (per the research) | `ai` |

## 6. ADR list

Numbering is proposed. ADRs 0019 to 0021 are *proposed decisions*, not yet made.

- **ADR-0001** Tauri 2 as the app shell, with Electron as the fallback if R0 fails. Main alternative: Electron from the start.
- **ADR-0002** One webview window per surface with its own Vite entry; the panel is pre-created hidden. Main alternative: one window with client-side routes.
- **ADR-0003** pnpm + Turborepo + Cargo workspace with `core`, `os` and `ai` crates. Main alternative: a single crate and a single package.
- **ADR-0004** Rust-to-TS contract through tauri-specta generated bindings. Main alternative: `ts-rs` or hand-written types.
- **ADR-0005** Capture by simulated copy chord with clipboard restore, no background monitor. Main alternatives: clipboard monitor, macOS `AXSelectedText`, X11 PRIMARY selection.
- **ADR-0006** Paste by clipboard write, simulated paste chord and restore. Main alternative: typing characters with synthetic key events.
- **ADR-0007** Non-activating floating panel per OS (NSPanel, `WS_EX_NOACTIVATE`, X11 hints). Main alternative: a normal focused window with focus restore. R0 may supersede the Windows part.
- **ADR-0008** Linux: X11 fully supported; Wayland degraded through the portal. Main alternatives: X11 only, `uinput` injection, the RemoteDesktop portal.
- **ADR-0009** SQLite plus image files on disk. Main alternatives: JSON files, or images as DB BLOBs.
- **ADR-0010** API keys in the OS keychain, one entry per provider. Main alternative: an encrypted file with a master password.
- **ADR-0011** Rich text as sanitised HTML plus a plain fallback, never rendered. Main alternatives: plain text only, or rendering sanitised HTML in a sandboxed frame.
- **ADR-0012** Direct provider HTTP from Rust: no SDKs, no proxy, no streaming. Main alternatives: vendor SDKs, a relay backend.
- **ADR-0013** Gemini `generateContent` rather than the Interactions API. Main alternative: the Interactions API.
- **ADR-0014** Local-only usage counters with export. Main alternative: opt-in remote telemetry or a crash service.
- **ADR-0015** Tauri updater through GitHub Releases; signed and notarized macOS, unsigned Windows beta, AppImage and `.deb`. Main alternatives: app stores, Homebrew/winget only.
- **ADR-0016** Release secrets only in a protected GitHub Actions environment. Main alternative: signing on the maintainer's machine.
- **ADR-0017** Test strategy: unit tests, Storybook stories with a11y and interaction tests, Playwright with mocked IPC, manual per-OS native smoke checklist. Main alternative: automated OS-level E2E.
- **ADR-0018** Rollout: all three OSes built from day 1, R0 first, validated macOS, then Windows, then Linux. Main alternative: macOS only first.
- **ADR-0019** (proposed) Migrations with `PRAGMA user_version` and embedded SQL. Main alternatives: `rusqlite_migration`, `sqlx migrate`.
- **ADR-0020** (proposed) No IPC command injects caller-supplied text; paste takes ids only. Main alternative: a generic `paste_text(text)`.
- **ADR-0021** (proposed) Settings as one typed JSON row. Main alternative: a key-value table.

### Decision Log

| Date | Decision | Why | Alternatives rejected |
| --- | --- | --- | --- |
| 2026-09-30 | Tauri 2 (Rust core, system webview); Electron if R0 fails | A Rust core reaches OS APIs directly; the system webview keeps a tray-resident utility small | Electron as the primary (larger bundle and memory); native UI per OS (three UI codebases for a solo maintainer) |
| 2026-09-30 | UI: Vite, React, TypeScript, Tailwind 4, Radix, Zustand | Accessible primitives, token-driven styling, small state library | Heavier component frameworks; Redux |
| 2026-09-30 | One webview window per surface with separate entries | Fast cold start per surface; the panel stays light | One window with routes |
| 2026-09-30 | pnpm + Turborepo + Cargo workspace; `core`/`os`/`ai` crates; one issue owns one directory | Disjoint directories let AI agents work in parallel worktrees | Single crate and package; Nx |
| 2026-09-30 | tauri-specta generated types | One source of truth for the Rust-to-TS boundary | Hand-written TS types |
| 2026-09-30 | Capture: simulate copy, read, always restore; no background monitor | Captures only on explicit user intent; nothing is logged in the background | Background clipboard monitor |
| 2026-09-30 | Paste: write, simulate paste, restore | Works in any app that accepts paste | Synthetic typing |
| 2026-09-30 | Non-activating panel; width 520/560, centred on the active display | The target app keeps focus and caret | A normal activating window |
| 2026-09-30 | X11 full; Wayland via portal with paste degraded to manual Ctrl+V; Windows elevated apps documented | Wayland forbids injection; UIPI blocks input into elevated apps | X11 only; `uinput`; running elevated |
| 2026-09-30 | macOS Accessibility permission with onboarding and revoked state | Required to post key events | None viable |
| 2026-09-30 | SQLite, images on disk, no DB encryption in MVP | Simple, local, crash-safe | JSON files; encrypted DB |
| 2026-09-30 | API keys only in the OS keychain, one entry per provider | OS-grade secret storage with no master password | Encrypted file; plaintext settings |
| 2026-09-30 | Item types: text, rich text (sanitised HTML, never rendered), links (regex), PNG ≤ 10 MB; dedupe within a group; retention 200; auto-delete off | Faithful paste with no HTML execution risk | Rendering HTML; plain text only |
| 2026-09-30 | AI: direct reqwest + rustls from Rust, no SDK, no proxy, no streaming | One endpoint per vendor; no backend cost; the design shows loading, then result | Vendor SDKs; relay server; streaming |
| 2026-09-30 | Gemini `generateContent`, not Interactions | Interactions stores data by default (55 days paid, 1 day free) | Interactions API |
| 2026-09-30 | Defaults `claude-haiku-4-5`, `gpt-5-nano`, `gemini-3.5-flash-lite` (not yet verified with a real key) | Fastest and cheapest per vendor docs | Larger models |
| 2026-09-30 | Local-only counters, 90-day retention, toggle, Export diagnostics | Privacy: nothing is transmitted | Remote telemetry; crash service |
| 2026-09-30 | Tauri updater via GitHub Releases; signed and notarized macOS from first beta; unsigned Windows beta; AppImage and `.deb` | No server cost; Gatekeeper requires notarization | App stores; a self-hosted update server |
| 2026-09-30 | Secrets only in a protected `release` environment; fork PRs get none; SHA pinning; CODEOWNERS | Public repo worked on by many agents | Signing locally; unpinned actions |
| 2026-09-30 | Tests: Rust unit tests with recorded fixtures, Vitest, Playwright on the web build, 3-OS CI, manual native smoke | Native flows can't be automated cheaply | OS-level E2E automation |
| 2026-09-30 | Rollout: R0 spike, then P1–P4; validate macOS, then Windows, then Linux | Retire platform risk before building screens | macOS only first |

## 7. Open technical questions

1. **Q1** `[NEEDS INPUT: How does Copy to group work on Wayland, where the copy chord can't be injected? (a) the user presses Ctrl+C first and the shortcut saves the current clipboard, (b) not supported on Wayland, (c) other.]`
2. **Q2** `[NEEDS INPUT: Terminals. Injected Ctrl+C can SIGINT a running command and Ctrl+V doesn't paste in Linux terminals. Maintain a terminal list that uses Ctrl+Shift+C/V, read the X11 PRIMARY selection instead, or document it as unsupported?]`
3. **Q3** `[NEEDS INPUT: If R0 shows Windows (or X11) can't take keys without activating, is the "Acceptable" focus-restore behaviour in R0-3 signed off as meeting "must not steal focus"?]`
4. **Q4** `[NEEDS INPUT: Which clipboard formats must restore guarantee: text, HTML and images only, or also copied files and app-private formats?]`
5. **Q5** `[NEEDS INPUT: Maximum size of a text item and of its HTML flavour (proposed: 1 MB plain, 2 MB HTML).]`
6. **Q6** `[NEEDS INPUT: Is an AI result ever saved as an item, or only pasted and then dropped?]`
7. **Q7** `[NEEDS INPUT: What does auto-delete delete (items older than N days?), and are Paste All order and separator remembered per group, globally, or not at all?]`
8. **Q8** `[NEEDS INPUT: Bundle identifier (reverse-DNS). It is permanent: macOS Accessibility grants, keychain access, app data paths and the updater all key off it.]`
9. **Q9** `[NEEDS INPUT: Minimum OS versions and architectures. macOS universal or arm64 only; Windows x64 only or arm64 too; oldest Linux distro/glibc (WebKitGTK 4.1 is required).]`
10. **Q10** `[NEEDS INPUT: Which issue owns src-tauri/src (commands, capabilities, tauri.conf.json) and the shared UI code (generated bindings, IPC mock harness)? The ownership table in the agent guide has no row for them.]`
11. **Q11** `[NEEDS INPUT: Update checks automatic, user-toggleable, or manual only; install silently or on prompt?]`
12. **Q12** `[NEEDS INPUT: Items marked as concealed by password managers (for example org.nspasteboard.ConcealedType): capture normally, warn, or refuse?]`
13. **Q13** `[NEEDS INPUT: R0 timebox, and who decides the Electron switch.]`
14. **Q14** `[NEEDS INPUT: When and how Windows gets signed (which certificate or signing service).]`
15. **Q15** `[NEEDS INPUT: Should HTML that adds no formatting (browsers attach HTML to almost every copy) still make an item "Rich text" with the Formatted chip? Should RTF-only sources such as TextEdit be converted to HTML on macOS?]`
16. **Q16** `[NEEDS INPUT: Which screen readers are in MVP scope: VoiceOver, Narrator and/or NVDA, Orca?]`
17. **Q17** `[NEEDS INPUT: On macOS, should the Main Window appear in the Dock and ⌘Tab while open, or stay accessory-only?]`
18. **Q18** `[NEEDS INPUT: The "active display" for centring: the display under the cursor or the one holding the focused window? And do ⌘1–⌘5 map to group position or to most recent use?]`
19. **Q19** `[NEEDS INPUT: Native window decorations (macOS overlay title bar, native Windows/Linux frames) or the custom-drawn Window Titlebar component for Main, Settings and Onboarding?]`

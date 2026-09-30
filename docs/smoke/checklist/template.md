# Native smoke checklist template

Native behaviour (global shortcut, capture, paste, panel focus, tray, keychain) has no unit test. Every native issue carries its steps in its own `## Smoke checklist` section, copied from this file.

## How to use

1. Copy the blocks below into the issue's `## Smoke checklist` section.
2. Delete the OS blocks the issue does not touch.
3. Replace each `<step>` and `<expected>` placeholder in the `Steps` list. Add or remove rows. Change nothing else.
4. Whoever runs the block fills the `Run by`, version, `Build` and `Result` lines and ticks a box only after performing that step on that OS. Unperformed boxes stay unticked, with the runner named.

Who runs what:

| OS | Who runs it | When |
| --- | --- | --- |
| macOS | The issue's author | Per pull request, before review |
| Windows | The sprint's Windows smoke story | After merge, in a Windows VM (see `vm-setup.md`) |
| Linux X11, Linux Wayland | The sprint's Linux smoke story | After merge, in a Linux VM (see `vm-setup.md`) |

Step rules:

- One action and one observable result per row.
- Name the target app from the list in the OS block.
- Repetitions follow `docs/technical-approach.md` section 1.7: text 20/20, image 5/5.
- A step that fails becomes a `type:bug` issue with a `severity:*` label. Link it on the row.

---

### macOS

- Run by: `<name>` on `<date>`
- macOS version: `<version>`
- Build: `<commit sha>`, from `make dev` or `make build`

Target apps: TextEdit, Safari textarea, VS Code, Terminal.

Preconditions:

- [ ] Accessibility permission granted to the app (System Settings, Privacy & Security, Accessibility). Revoked only if the issue tests the revoked state
- [ ] Notifications allowed for the app
- [ ] Clipboard holds a known text sentinel before the first step

Steps:

- [ ] `<step>`. Expected: `<expected>`
- [ ] `<step>`. Expected: `<expected>`

Result: `<pass | fail | blocked>`. Notes and linked bugs: `<notes>`

### Windows

- Run by: `<name>` on `<date>`
- Windows version and architecture: `<version, x64 or ARM64>`
- Build: `<commit sha>`

Target apps: Notepad, Edge textarea, VS Code, Windows Terminal. Elevated Notepad is recorded only.

Preconditions:

- [ ] App started as a normal user, not elevated
- [ ] WebView2 Evergreen runtime present
- [ ] Clipboard holds a known text sentinel before the first step

Steps:

- [ ] `<step>`. Expected: `<expected>`
- [ ] `<step>`. Expected: `<expected>`

Result: `<pass | fail | blocked>`. Notes and linked bugs: `<notes>`

### Linux X11

- Run by: `<name>` on `<date>`
- Distro and desktop: `<distro, version, desktop>`
- Session type: `<output of echo $XDG_SESSION_TYPE, expected x11>`
- Build: `<commit sha>`

Target apps: Text Editor, Firefox textarea, VS Code. GNOME Terminal is recorded only.

Preconditions:

- [ ] Logged in to an Xorg session
- [ ] Secret Service running (gnome-keyring or KWallet), absent only if the issue tests the missing-prerequisite state
- [ ] Tray extension active (AppIndicator), if the issue touches the tray
- [ ] Clipboard holds a known text sentinel before the first step

Steps:

- [ ] `<step>`. Expected: `<expected>`
- [ ] `<step>`. Expected: `<expected>`

Result: `<pass | fail | blocked>`. Notes and linked bugs: `<notes>`

### Linux Wayland

- Run by: `<name>` on `<date>`
- Distro and desktop: `<distro, version, GNOME or KDE Plasma>`
- Session type: `<output of echo $XDG_SESSION_TYPE, expected wayland>`
- Build: `<commit sha>`

Target apps: Text Editor, Firefox textarea. Keystroke injection is not possible on Wayland, so paste ends with the item on the clipboard and a manual Ctrl+V.

Preconditions:

- [ ] Logged in to a Wayland session
- [ ] `xdg-desktop-portal` running with a GlobalShortcuts backend for the desktop
- [ ] Clipboard holds a known text sentinel before the first step

Steps:

- [ ] `<step>`. Expected: `<expected>`
- [ ] `<step>`. Expected: `<expected>`

Result: `<pass | fail | blocked>`. Notes and linked bugs: `<notes>`

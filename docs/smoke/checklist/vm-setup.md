# VM setup for the Windows and Linux smoke runs

The Windows and Linux smoke stories run the blocks from `template.md` on virtual machines hosted on the author's Mac. macOS smoke runs on the host itself, so it needs no VM.

Items marked [verify] depend on the hypervisor or distro release and were not checked when this file was written.

## Hypervisor

Any hypervisor that runs the guest on the host's CPU architecture works. Pick one and use it for both guests.

| Hypervisor | Cost | Notes |
| --- | --- | --- |
| UTM | Free from the project site, paid on the Mac App Store | Apple Silicon and Intel |
| Parallels Desktop | Paid | Apple Silicon and Intel |
| VMware Fusion | Free for personal use [verify] | Apple Silicon and Intel |

On Apple Silicon the guests are ARM64: Windows 11 ARM and an arm64 Linux image. Record the architecture in the `Windows version and architecture` and `Distro and desktop` lines of the template. Results on ARM64 guests do not cover x64 timing.

## Settings for every guest

- Disable host-to-guest clipboard sharing and drag and drop. Shared clipboard agents write to the guest clipboard and break the capture, paste and restore steps.
- 4 CPUs, 8 GB memory, 64 GB disk, 3D acceleration on if offered.
- Take a snapshot named `clean-baseline` after the OS is installed and updated, and another named `ready` after the tooling below. Revert to `ready` before each smoke run.
- Check that the shortcut chord reaches the guest. A macOS host can capture ⌘ combinations before the guest sees them. If it does, map the host's ⌘ to the guest's Windows or Super key in the hypervisor's keyboard settings [verify].
- Add a Dvorak layout and a Russian layout to the guest for the layout steps.

## Windows guest

1. Install Windows 11 from a Microsoft ISO. Use the Microsoft Evaluation Center ISO for a licence-free guest [verify current availability].
2. Create two local accounts: `smoke` as a standard user and `smoke-admin` as an administrator. Run the app as `smoke` unless the step tests an elevated target.
3. Install the apps named in the Windows block of `template.md`: Edge (present), Notepad (present), VS Code, Windows Terminal.
4. WebView2 is present on Windows 10 1803 and later and on Windows 11. No separate install is needed.
5. Get the app. Until CI publishes installers, build inside the guest:
   - Microsoft C++ Build Tools with the "Desktop development with C++" workload.
   - Rust with the MSVC host triple: `winget install --id Rustlang.Rustup`, then `rustup default stable-msvc`. On an ARM64 guest the target is `aarch64-pc-windows-msvc`.
   - Node LTS and pnpm, then clone the repository and run `make install` and `make dev` (or `make build`).
   - Install GNU Make, or run the `pnpm` and `cargo` commands that the `Makefile` targets wrap [verify the Makefile runs under PowerShell].
6. Restart the guest, take the `ready` snapshot.

## Linux guest: X11 and Wayland

Use one Ubuntu Desktop LTS image (GNOME) for X11, and the same image for GNOME Wayland. Add a second guest with Kubuntu for KDE Plasma Wayland.

1. Install the distro and update it.
2. GNOME guest: pick the session at the login screen with the gear icon. "Ubuntu on Xorg" is the X11 session, "Ubuntu" is Wayland [verify names for the release]. Confirm with `echo $XDG_SESSION_TYPE` (prints `x11` or `wayland`) and write it on the Session type line.
3. Runtime libraries the app needs, from the Tauri prerequisites for Debian and Ubuntu:

   ```sh
   sudo apt update
   sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
   ```

4. Install the apps named in the Linux blocks: Text Editor (present), Firefox, VS Code, GNOME Terminal (present).
5. Secret Service: the GNOME guest ships gnome-keyring. Unlock it on first login. To test the missing-prerequisite state, stop the keyring service in a separate snapshot and revert afterwards.
6. Tray: stock GNOME hides AppIndicator icons unless the AppIndicator extension is enabled. Ubuntu ships and enables it [verify]. Enable it when the issue touches the tray.
7. Get the app: install Rust (`curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`), Node LTS and pnpm, clone the repository, run `make install` and `make dev`.
8. Take the `ready` snapshot once per session type you test.

## After a run

- Tick only the rows you performed in the issue's checklist and fill the `Run by`, version and `Build` lines.
- Revert the guest to `ready`.
- Open a `type:bug` issue with a `severity:*` label for each failing step, in the next sprint.

# Linux guests

Shared settings, snapshots and the run procedure are in `README.md`. The target apps and preconditions come from the Linux X11 and Linux Wayland blocks of `docs/smoke/checklist/template.md`.

Nothing here has been run on a VM. `[unverified]` marks a detail the vendor documentation I read did not confirm.

## Guests

| Guest | Image | Sessions |
| --- | --- | --- |
| Ubuntu 22.04 GNOME | `ubuntu-22.04.5-desktop-amd64.iso` from releases.ubuntu.com | X11 and Wayland, from the one guest |
| Kubuntu 22.04 KDE Plasma | `kubuntu-22.04.5-desktop-amd64.iso` from cdimage.ubuntu.com/kubuntu/releases/22.04/release/ | Plasma X11 and Plasma Wayland, from the one guest |

Both desktop ISOs are amd64 only. On Apple Silicon install `ubuntu-22.04.5-live-server-arm64.iso` from cdimage.ubuntu.com/releases/22.04/release/, then run `sudo apt install ubuntu-desktop` for the GNOME guest or `sudo apt install kubuntu-desktop` for the KDE guest `[unverified: end-to-end on arm64]`.
## Wayland global shortcut support by release

Ubuntu 22.04 cannot test the Wayland global shortcut path. The GlobalShortcuts portal needs three parts: the `xdg-desktop-portal` frontend, a desktop backend, and the app. The table shows which parts each release ships. Package versions come from packages.ubuntu.com; backend presence comes from upstream release notes and source. No release below was run. The frontend gained the interface in `xdg-desktop-portal` 1.16.0 (2022-12-12). `xdg-desktop-portal-gnome` gained its backend in 48. `xdg-desktop-portal-kde` has `src/globalshortcuts.cpp` at tag v5.27.0 and not at v5.24.4.

| Release | `xdg-desktop-portal` | GNOME backend | KDE backend | GlobalShortcuts |
| --- | --- | --- | --- | --- |
| Ubuntu and Kubuntu 22.04 | 1.14.x | `xdg-desktop-portal-gnome` 42.1 | `xdg-desktop-portal-kde` 5.24.4 | Absent on both desktops |
| 24.04 | 1.18.4 | 46.0 | 5.27.11 | KDE backend present, GNOME absent |
| 26.04 | 1.21.1 | 50.0 | 6.6.4 | GNOME and KDE backends present |

On 22.04 a Wayland run exercises only the fallback path for a missing portal. For the real portal path add one of these guests, built the same way as the 22.04 guests:

- Ubuntu 26.04.1 GNOME Wayland. An ARM64 desktop ISO is listed on https://ubuntu.com/download/desktop.
- Kubuntu 26.04.1 KDE Plasma Wayland. Only an amd64 desktop ISO is listed at cdimage.ubuntu.com/kubuntu/releases/26.04/release/.

The issue that owns the Wayland spike decides whether to add them. Treat this table as the reason to ask.

## Create the guest in UTM

1. In UTM choose `+`, Virtualize, Linux `[unverified: wizard path]`. Give it 4 CPUs, 8 GB memory and a 64 GB disk.
2. Install the distro and update it with `sudo apt update && sudo apt full-upgrade`.
3. Do not install `spice-vdagent`. UTM documents it as required for clipboard sharing and dynamic resolution. Skipping it costs auto-resize, so set the resolution by hand. https://docs.getutm.app/guest-support/linux/
4. Turn off clipboard sharing in the VM's UTM settings `[unverified: setting location]`.

## Tooling

Install the Tauri v2 Debian and Ubuntu prerequisites. Every package below exists for 22.04 on packages.ubuntu.com, including `libwebkit2gtk-4.1-dev` 2.50.x. https://v2.tauri.app/start/prerequisites/

```sh
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

Then install Rust with the rustup script from https://rustup.rs, Node LTS and pnpm, clone the repository and run `make install` and `make dev`.

Apps for the blocks: Text Editor, GNOME Terminal and Firefox come with the GNOME guest. Install VS Code from Microsoft's apt repository or the `.deb`. On the KDE guest use Kate in place of Text Editor and Konsole in place of GNOME Terminal `[unverified: the template names GNOME apps only]`.

Rig check tools, only for the sentinel line:

```sh
sudo apt install xclip wl-clipboard
```

Take the `ready` snapshot once per session type you test.

## Pick the session

GNOME guest: at the login screen choose the gear icon. `Ubuntu on Xorg` is the X11 session and `Ubuntu` is the Wayland session `[unverified: names]`.

KDE guest: at the SDDM login screen choose `Plasma (X11)` or `Plasma (Wayland)` `[unverified: names]`. If the Wayland entry is missing run `sudo apt install plasma-workspace-wayland`. The package exists for 22.04.

In every session run `echo $XDG_SESSION_TYPE`. It prints `x11` or `wayland`. Write that on the `Session type` line. It is the authority; the login-screen names are not.

## Rig check

Run the six lines in `README.md`, then these Linux lines:

| Template precondition | Check |
| --- | --- |
| Logged in to an Xorg session | `echo $XDG_SESSION_TYPE` prints `x11` |
| Logged in to a Wayland session | `echo $XDG_SESSION_TYPE` prints `wayland` |
| Secret Service running | GNOME: `gnome-keyring` is installed on 22.04 and unlocks at first login. KDE: KWallet answers the Secret Service. `busctl --user list \| grep org.freedesktop.secrets` prints a name `[unverified: command output]` |
| Tray extension active, if the issue touches the tray | GNOME: `gnome-shell-extension-appindicator` exists for 22.04. `gnome-extensions list --enabled` shows it `[unverified: enabled by default]`. KDE: the system tray shows the icon |
| `xdg-desktop-portal` running with a GlobalShortcuts backend | `gdbus introspect --session --dest org.freedesktop.portal.Desktop --object-path /org/freedesktop/portal/desktop \| grep GlobalShortcuts` prints a match. On 22.04 it prints nothing, which is the expected result per the table above `[unverified: command output]` |
| Clipboard holds a known text sentinel | X11: `printf smoke-sentinel \| xclip -selection clipboard`, then `xclip -selection clipboard -o`. Wayland: `printf smoke-sentinel \| wl-copy`, then `wl-paste` |

To test the missing-prerequisite state for the keyring, stop the keyring service in a separate snapshot and revert afterwards.

Record the distro, version, desktop and architecture on the `Distro and desktop` line.

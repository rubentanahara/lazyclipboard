# Linux guests

Shared settings, snapshots and the run procedure are in `README.md`. The target apps and preconditions come from the Linux X11 and Linux Wayland blocks of `docs/smoke/checklist/template.md`.

Nothing here has been run on a VM. `[unverified]` marks a detail the vendor documentation I read did not confirm.

## Guests

| Guest | Image | Sessions |
| --- | --- | --- |
| Ubuntu 22.04 GNOME | `ubuntu-22.04.5-desktop-amd64.iso` from releases.ubuntu.com | X11 and Wayland, from the one guest |
| Kubuntu 22.04 KDE Plasma | `kubuntu-22.04.5-desktop-amd64.iso` from cdimage.ubuntu.com/kubuntu/releases/22.04/release/ | Plasma X11 and Plasma Wayland, from the one guest |
| Ubuntu 26.04 GNOME | `ubuntu-26.04.1` desktop ISO from https://ubuntu.com/download/desktop, ARM64 listed | Wayland, the GlobalShortcuts path |
| Kubuntu 26.04 KDE Plasma | `kubuntu-26.04.1-desktop-amd64.iso` from cdimage.ubuntu.com/kubuntu/releases/26.04/release/, no ARM64 desktop ISO listed | Wayland, the GlobalShortcuts path |

The 22.04 desktop ISOs are amd64 only. On Apple Silicon install `ubuntu-22.04.5-live-server-arm64.iso` from cdimage.ubuntu.com/releases/22.04/release/, then run `sudo apt install ubuntu-desktop`, then `sudo reboot`. This path is in UTM's Ubuntu guide: https://docs.getutm.app/guides/ubuntu/. The KDE equivalent, `sudo apt install kubuntu-desktop`, is `[unverified]`, and no ARM64 Kubuntu 26.04 desktop ISO is listed, so that guest has no verified path on Apple Silicon.

## Wayland global shortcut support by release

The spike spec asks for Wayland on GNOME and KDE Plasma, latest stable (`docs/technical-approach.md` section 1.7). 22.04 cannot run that path, so the 26.04 guests above are the rigs that can pass the Wayland block. The 22.04 Wayland sessions exercise only the fallback for a missing portal.

The GlobalShortcuts portal needs three parts: the `xdg-desktop-portal` frontend, a desktop backend, and the app. The frontend gained the interface in `xdg-desktop-portal` 1.16.0 (2022-12-12). `xdg-desktop-portal-gnome` gained its backend in 48. `xdg-desktop-portal-kde` has `src/globalshortcuts.cpp` at tag v5.27.0 and not at v5.24.4. Backend presence comes from upstream release notes and source. No release was run.

| Release | `xdg-desktop-portal` | GNOME backend | KDE backend | GlobalShortcuts |
| --- | --- | --- | --- | --- |
| Ubuntu and Kubuntu 22.04 | 1.14.x | `xdg-desktop-portal-gnome` 42.1 | `xdg-desktop-portal-kde` 5.24.4 | Absent on both desktops |
| 24.04 | 1.18.4 | 46.0 | 5.27.11 | KDE backend present, GNOME absent |
| 26.04 | 1.21.1 | 50.0 | 6.6.4 | GNOME and KDE backends present |

The versions are from the release pocket on packages.ubuntu.com. `apt full-upgrade` can install newer point releases, so record what the guest has with `apt policy xdg-desktop-portal xdg-desktop-portal-gnome xdg-desktop-portal-kde`.

## Create the guest in UTM

Follow https://docs.getutm.app/guides/ubuntu/ for the wizard (`+`, Virtualize, Linux). Give the guest 4 CPUs, 8 GB memory and a 64 GB disk.

1. Install the distro. At the end of the install the reboot can leave a black screen with a blinking cursor. The guide says this is expected: quit the VM, clear the installer ISO, start it again. If the VM stops at an EFI screen, the guide lists the ISO and `FS0:` checks.
2. If networking fails after the install, the adapter name changed. The guide edits `/etc/netplan/00-installer-config.yaml` to carry the old adapter's block over to the new name.
3. Update with `sudo apt update && sudo apt full-upgrade`.
4. Take the `clean-baseline` snapshot.
5. Clipboard isolation: `ubuntu-desktop` and `kubuntu-desktop` depend on `spice-vdagent` in 22.04, which UTM documents as the guest side of clipboard sharing, so the agent is installed. Isolation rests on turning clipboard sharing off in the VM's UTM settings `[unverified: setting location]`. Rig check line 1 in `README.md` proves it.
6. Do not enable automatic login in the installer. The keyring unlocks with the login password `[unverified]`.

## Tooling

Install the Tauri v2 Debian and Ubuntu prerequisites. Every package below exists for 22.04 on packages.ubuntu.com, including `libwebkit2gtk-4.1-dev` 2.50.x. https://v2.tauri.app/start/prerequisites/

```sh
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

Then install what the desktop images do not ship:

```sh
sudo apt install git firefox xclip wl-clipboard
```

Firefox on Ubuntu is a snap-backed package `[unverified]`; install it from the vendor's recommended source if `apt` refuses. Then install Rust with the rustup script from https://rustup.rs, Node LTS and pnpm, clone the repository and run `make install` and `make dev`.

Apps for the blocks: Text Editor and GNOME Terminal come with the GNOME guest. Install VS Code from Microsoft's apt repository or the `.deb`. On the KDE guest use Kate in place of Text Editor and Konsole in place of GNOME Terminal `[unverified: the template names GNOME apps only]`.

Take the `ready-x11` or `ready-wayland` snapshot once per session type you test. The snapshot name carries the session so a revert is unambiguous.

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
| Secret Service running | `busctl --user list \| grep org.freedesktop.secrets` prints a name `[unverified: command output]`. The GNOME guest ships `gnome-keyring`. Kubuntu 22.04 ships KWallet from KDE Frameworks 5.92, and the Secret Service API is in the KWallet notes of Frameworks 5.97, so the KDE guest likely prints nothing. Install `gnome-keyring` there and check again `[unverified]` |
| Tray extension active, if the issue touches the tray | GNOME: `gnome-shell-extension-appindicator` exists for 22.04. `gnome-extensions list --enabled` shows it `[unverified: enabled by default]`. KDE: the system tray shows the icon |
| `xdg-desktop-portal` running with a GlobalShortcuts backend | `gdbus introspect --session --dest org.freedesktop.portal.Desktop --object-path /org/freedesktop/portal/desktop \| grep GlobalShortcuts` prints a match `[unverified: command output]`. On 22.04 it prints nothing, so the 22.04 Wayland rigs fail this line by design. Only the 26.04 guests can pass it |
| Clipboard holds a known text sentinel | X11: `printf smoke-sentinel \| xclip -selection clipboard`, then `xclip -selection clipboard -o`. Wayland: `printf smoke-sentinel \| wl-copy`, then `wl-paste` |

Producing the keyring-absent state for the missing-prerequisite test is `[unverified]`. The keyring is socket and D-Bus activated, so stopping the service may not keep it absent. Confirm that the `busctl` line prints nothing before recording a missing-prerequisite result.

Record the distro, version, desktop and architecture on the `Distro and desktop` line.

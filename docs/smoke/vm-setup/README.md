# VM test rigs for the Windows and Linux smoke runs

The Windows and Linux blocks of `docs/smoke/checklist/template.md` run on virtual machines hosted on a Mac. macOS smoke runs on the host and needs no VM.

| File | Covers |
| --- | --- |
| `README.md` | Hypervisor, settings every guest shares, snapshots, run procedure |
| `windows.md` | Windows 11 and Windows 10 guests |
| `linux.md` | Ubuntu 22.04 GNOME on X11 and Wayland, Kubuntu 22.04 KDE Plasma, Ubuntu and Kubuntu 26.04 Wayland guests |

`docs/smoke/checklist/vm-setup.md` is an earlier draft with the same purpose, and `template.md` still points to it. Use this directory instead. The draft and the pointer sit outside this directory's owner; the follow-up is on issue #68.

## Status of this document

Written on 2026-09-30 from vendor documentation. No step in these files has been run on a VM, so every rig is untested. A person with a real VM must follow the files once and record the result in the table below. `[unverified]` marks a detail the vendor documentation I read did not confirm.

| Rig | Setup followed end to end | Rig check passed | By and date |
| --- | --- | --- | --- |
| Windows 11 | no | no | |
| Windows 10 | no | no | |
| Ubuntu 22.04 GNOME, X11 session | no | no | |
| Ubuntu 22.04 GNOME, Wayland session (no GlobalShortcuts portal, fallback path only) | no | no | |
| Kubuntu 22.04 KDE Plasma, X11 session | no | no | |
| Kubuntu 22.04 KDE Plasma, Wayland session (no GlobalShortcuts portal, fallback path only) | no | no | |
| Ubuntu 26.04 GNOME, Wayland session | no | no | |
| Kubuntu 26.04 KDE Plasma, Wayland session | no | no | |

The two 22.04 Wayland rigs fail the GlobalShortcuts precondition of the Wayland block by design. See `linux.md`. Only the 26.04 rigs can pass it.

## Hypervisor

Use UTM for both guests. It is free from the project site, and its Windows, Ubuntu and guest-support guides are the ones the steps follow. No step here is written or checked for another hypervisor. The other two are listed only for cost.

| Hypervisor | Cost | Source |
| --- | --- | --- |
| UTM | Free from getutm.app, paid on the Mac App Store | https://docs.getutm.app |
| VMware Fusion | Free for personal use with Fusion Pro 13.5.2 or newer, needs a Broadcom Support Portal account. Older versions give "Not Entitled" | https://knowledge.broadcom.com/external/article/368667/vmware-fusion-and-workstation-now-free-for-personal-use.html |
| Parallels Desktop | Paid `[unverified]` | |

## Architecture

On Apple Silicon the guests are ARM64 and run on the host's CPU. Results on ARM64 guests do not cover x64 timing. Record the architecture on the `Windows version and architecture` and `Distro and desktop` lines of the template.

Not every guest has an ARM64 image. `windows.md` and `linux.md` name the gaps. An x64 guest on Apple Silicon runs under emulation, so its R0-6 timing (p95 of 150 ms over 50 opens) and NFR-1 results in `docs/technical-approach.md` section 1.7 are not valid. The functional criteria (R0-1 to R0-5, R0-7, R0-8) still run there `[unverified: emulation speed]`.

## Settings for every guest

- Turn clipboard sharing off in the hypervisor. UTM documents clipboard sharing as a per-VM setting plus guest tools; turning it off is inferred, not documented. The guest tools are installed anyway: the Windows SPICE tools carry the network drivers, and the Ubuntu and Kubuntu 22.04 desktop metapackages depend on `spice-vdagent`. The rig check below proves the isolation.
- 4 CPUs, 8 GB memory, 64 GB disk.
- Take a snapshot named `clean-baseline` after the OS is installed and updated, and another named `ready` after the tooling in the guest file (`ready-x11` and `ready-wayland` on Linux). Revert to `ready` before each smoke run. Snapshots are `[unverified]`: the UTM docs I read do not mention them. If UTM has none, keep clones of the VM under the same names.
- Add a Dvorak layout and a Russian layout for the layout steps.
- A macOS host can capture ⌘ combinations before the guest sees them. If a chord does not arrive, change the hypervisor's keyboard capture or key mapping `[unverified: setting names]`.

## Rig check

Run it once after the `ready` snapshot, and again after any change to the guest. A rig passes when every line holds.

1. Clipboard isolation: copy a known string on the Mac, paste in the guest. Nothing arrives. Copy a string in the guest, paste on the Mac. Nothing arrives.
2. Sentinel: put a known text sentinel on the guest clipboard with the command in the guest file, and read it back.
3. Target apps: every app in the OS block of `template.md` opens.
4. Chord: the chord the issue under test binds reaches the guest while a target app has focus.
5. Preconditions: every precondition in the OS block of `template.md` can be made true. The guest file lists the check for each.
6. Layouts: Dvorak and Russian can be selected and type their characters.

Any line that fails means the rig cannot execute the template. Fix it before running a smoke block, and record the failure in the table above.

## After a run

- Tick only the rows you performed in the issue's checklist and fill the `Run by`, version and `Build` lines.
- Revert the guest to `ready`.
- Open a `type:bug` issue with a `severity:*` label for each failing step, in the next sprint.

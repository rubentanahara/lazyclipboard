# Windows guests

Shared settings, snapshots and the run procedure are in `README.md`. The target apps and preconditions come from the Windows block of `docs/smoke/checklist/template.md`.

Nothing here has been run on a VM. `[unverified]` marks a detail the vendor documentation I read did not confirm.

## Images

| Guest | Image | Architecture on Apple Silicon |
| --- | --- | --- |
| Windows 11 | Windows 11 Enterprise evaluation ISO from the Microsoft Evaluation Center, 90 days, x64 and Arm64 ISOs listed (26H2 at the time of writing). https://www.microsoft.com/en-us/evalcenter/evaluate-windows-11-enterprise | ARM64 |
| Windows 10 | Windows 10 22H2 multi-edition ISO (Home and Pro) from https://www.microsoft.com/en-us/software-download/windows10ISO. The page I read showed an error where the download links belong, so the download is `[unverified]`. | No ARM64 ISO is listed. Needs an x64 host or x86_64 emulation, so timing is not valid |

Windows 10 reached end of support on 2025-10-14, so the rig gets no security updates. Use it only for the smoke run and keep it off the network outside the run `[unverified: whether the guest needs network after install]`.

Windows 10 Enterprise is not listed on the Evaluation Center page.

An Arm64 Windows 11 ISO can also come from CrystalFetch or Microsoft's Windows 11 ARM64 page, per the UTM guide: https://docs.getutm.app/guides/windows/

## Create the guest in UTM

1. In UTM choose `+`, Virtualize, Windows.
2. Give it 4 CPUs (Windows needs at least 2 to install), 8 GB memory and a 64 GB disk.
3. Keep both options checked: install Windows 10 or higher, and install drivers and SPICE tools. Select the ISO. Pick the ARM64 ISO on Apple Silicon and the amd64 ISO on Intel, since the wrong one lands in the EFI shell.
4. Save the VM, wait for the guest tools to finish downloading, start it, press any key at boot and follow the installer.
5. If the screen goes black on 24H2, eject the guest tools ISO, reboot, and mount it again from `Install Windows Guest Tools` after setup. If setup has no network, press Shift+F10 and run `OOBE\BYPASSNRO`, or `start ms-cxh:localonly` on newer builds.
6. Turn off clipboard sharing in the VM's UTM settings, then restart the guest `[unverified: setting location]`.

Steps 1 to 5 follow https://docs.getutm.app/guides/windows/ and https://docs.getutm.app/guest-support/windows/. Newer UTM enables Secure Boot and TPM for the Windows 11 guest. On an older UTM the guide bypasses the checks with the `LabConfig` registry key.

## Accounts

Create `smoke` as a standard user and `smoke-admin` as an administrator. Run the app as `smoke` unless the step tests an elevated target. The Evaluation Center page says sign-in uses a Microsoft account and promises a local account only at final release, so the local-account setup is `[unverified]`.

## Tooling

The Windows block needs Notepad, Edge, VS Code and Windows Terminal.

- Notepad and Edge ship with Windows.
- Install VS Code and Windows Terminal with `winget install --id Microsoft.VisualStudioCode` and `winget install --id Microsoft.WindowsTerminal` `[unverified: package ids]`.
- WebView2 comes preinstalled on Windows 10 1803 and later and on Windows 11. https://v2.tauri.app/start/prerequisites/

Until CI publishes installers, build the app in the guest. The steps are the Tauri v2 Windows prerequisites:

1. Install the Microsoft C++ Build Tools from https://visualstudio.microsoft.com/visual-cpp-build-tools/ with the "Desktop development with C++" workload.
2. In PowerShell run `winget install --id Rustlang.Rustup`, then `rustup default stable-msvc`. On an ARM64 guest the host triple is `aarch64-pc-windows-msvc`. Restart the terminal afterwards.
3. Install Node LTS and pnpm, clone the repository, then run `make install` and `make dev` or `make build`. GNU Make for Windows is not covered by the Tauri docs, and whether the `Makefile` runs under PowerShell is `[unverified]`. If it does not, run the `pnpm` and `cargo` commands the targets wrap.

Restart the guest and take the `ready` snapshot.

## Rig check

Run the six lines in `README.md`, then these Windows lines:

| Template precondition | Check |
| --- | --- |
| App started as a normal user, not elevated | Sign in as `smoke`. Task Manager, Details tab, Elevated column shows `No` for the app |
| WebView2 Evergreen runtime present | `Get-ItemProperty 'HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}' \| Select-Object pv` prints a version `[unverified: registry path]` |
| Clipboard holds a known text sentinel | `Set-Clipboard -Value 'smoke-sentinel'`, then `Get-Clipboard` prints `smoke-sentinel` |

Record `winver` output and `$env:PROCESSOR_ARCHITECTURE` on the `Windows version and architecture` line.

# Critique: 05 Screen · Copy Notification

**Screen:** the confirmation banner after copying to a group. Job: confirm the save, name the destination, allow a quick undo. Seen: app icon, "Saved to Design refs", a two-line preview, "Undo" and "Open group" actions, time "now". Shown top-right on macOS, top-centre on Linux, above the taskbar on Windows.

## Walkthrough

Confirmation that names the group and shows what was saved is exactly what a background action needs (heuristic 1 holds), and Undo is offered at the moment it is needed (heuristic 3).

**Important (UX):** the banner has no visible lifetime. I can't tell from the frame how long Undo stays available or how it disappears. State the timeout (for example 6 seconds) and keep the banner up while the pointer is on it. Undo that vanishes in two seconds is worse than none.

**Important (UX, accessibility):** the notification must be announced to screen readers and must not be the only confirmation. Use the OS notification API rather than a custom overlay so VoiceOver, Narrator and Orca announce it, and respect Do Not Disturb and Focus modes.

**Important (UX):** failure is not designed. If the capture is empty, the group was deleted, or storage fails, the user gets no feedback at all. Design one error variant ("Nothing to save").

**Nit (UI):** "Undo" and "Open group" are 12px accent text with no button affordance; they pass contrast (6.3:1 on `surface`) but are small targets. Use 13px medium with padding for 32px-high hit areas.

**Nit (UI):** the app icon is a placeholder clipboard glyph; the real icon should follow each OS's notification conventions.

## Start here
1. Define the Undo timeout and hover-to-pause.
2. Use native notifications so assistive tech announces them.
3. Design the failure banner.

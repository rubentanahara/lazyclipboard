# Critique: 05 Screen · Tray Menu

**Screen:** the menu opened from the menu-bar (macOS), top panel (Linux) or taskbar tray (Windows) icon. Job: reach the two primary actions and jump to a recent group. Seen: Copy to group and Paste from group with shortcuts, three recent groups with chevrons, Open, Settings, Quit.

## Walkthrough

Structure follows platform conventions: primary actions first, a labelled recent-groups section, then app-level commands separated by dividers (heuristic 4 holds). Repeating shortcuts in the menu teaches them, which supports the keyboard-first goal.

**Important (UX):** "Copy to group" and "Paste from group" from a menu only make sense if the previous app still holds focus and its selection. Clicking a tray item steals focus on some platforms, so the selection can be lost. I can't tell from the frame how this is handled; test on all three OSes, and consider that on macOS menu-bar items do not take focus from the front app but on Windows the flyout does.

**Important (UX):** the three recent groups show chevrons, which promise a submenu, but there is no indication whether clicking pastes, opens the picker or opens the main window. Rename the section "Paste from…" or show the action in a submenu header.

**Important (UX):** there is no state for "capture is paused" or a way to pause it. That is a privacy-relevant control in a clipboard tool, and the tray is where users expect it. This is a feature suggestion, not a defect; skip if capture is only on-demand.

**Nit (UI):** the "RECENT GROUPS" overline is 10px `text-3`; raise to 11px. The highlighted first row uses solid accent, which suggests a selected state that is meaningless before the pointer moves; use it only on hover or keyboard focus.

**Nit (UI):** on Windows the shortcut column holds "Ctrl+Alt+C", which is wider than the label; make sure the menu width flexes instead of clipping.

## Start here
1. Verify focus and selection survive opening the tray menu on each OS.
2. Clarify what the recent-group rows do.
3. Drop the permanent accent highlight on the first row.

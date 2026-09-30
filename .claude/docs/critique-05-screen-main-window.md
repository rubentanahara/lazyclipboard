# Critique: 05 Screen · Main Window

**Screen:** the management window. Job: browse groups and their items, inspect one, copy or delete it. Seen: 240px sidebar of groups with counts, header with group name, item count and modified time, search field, rename and delete buttons, item list with type icons, source and age, a 300px detail pane with preview, metadata, Copy and Delete buttons.

## Walkthrough

Three panes are the correct pattern for this job: navigation, content, inspection. The detail pane shows exactly the metadata the brief asked for (type, source app, copied, group) and puts Copy above Delete, so the primary action leads. Type icons and the thumbnail make the list scannable.

**Important (UX):** Delete is one click with no confirmation and no undo, next to Copy in the pane, and the trash button in the header deletes the whole group with the same weight. Nielsen 5 (error prevention) and 3 (control) both fail. Delete on an item should be immediate with an Undo toast (an item is cheap to lose); deleting a group needs a confirmation that names the item count.

**Important (UX):** the group header shows a pencil and a trash icon with no labels. Icon-only destructive controls are ambiguous and, at about 32px, small for a pointer target. Add tooltips and use a "…" menu for rename and delete, so the destructive one is not one mis-click from rename.

**Important (UX):** no empty states (empty group, no groups) and no sort control are visible. I can't tell how the list is ordered.

**Important (UI, accessibility):** the selected sidebar row and the selected item use tint only (about 1.15:1); there is no visible focus ring. Add the leading accent bar and a `border-focus` ring on keyboard focus.

**Nit (UI):** metadata at the right of each row ("Safari · 2m ago") is 12px `text-3` and right-aligned, so it competes with nothing but is far from the content it describes. Keep, but consider moving it under the content, as in the paste panels, for consistency.

**Nit (UI):** the detail pane's DETAILS labels are 12px `text-3` over a 90px label column; fine, but values in `text` weight 500 look heavier than the preview text. Use regular weight.

## Start here
1. Undo for item delete, confirmation for group delete.
2. Replace header icon buttons with a labelled menu.
3. Selection bar and focus ring.

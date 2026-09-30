# Critique: 05 Screen · Paste · Pick Item

**Screen:** second paste step. Job: pick one item from the group and paste it. Seen: group name with back chevron and count, in-group search, "Paste all 12 items" row, five item rows (text, image, link, file, text) with type icon, source app and age, footer with `↵ Paste`, `⇧↵ Plain text`, `← Back`, `⌥↵ AI`.

## Walkthrough

The item rows are the best-designed part of the app so far: a type icon, one line of content, and a metadata line make five very different content kinds scannable (heuristic 2, real-world match, holds), and the image row shows a thumbnail instead of a filename. Newest-first ordering is stated in the header.

**Important (UX):** the "Paste all 12 items" row is styled as an item and sits in the list at index zero, so arrow-down from the top lands on it first. The user's likely intent, pasting the latest item, needs one extra keypress. Keep the row, but start the selection on the first real item and reach "Paste all" with `⌘↵` (already hinted). Alternatively, move it into the header as an action.

**Important (UX):** long content truncates to one line ("Use a 4pt spacing scale so components stay aligned…"). There is no way to see the full text before pasting except pasting it. Add a preview line or a preview pane for the highlighted item, at least for text and links.

**Important (UI, accessibility):** selection is tint only, and the item's trailing `↵` cap is the only other cue. Use the leading accent bar.

**Nit (UI):** the footer holds five hints and clips the last label at the panel's right edge at the narrowest widths ("⌥↵ AI"); on Windows the key caps are longer (`Shift+↵`, `Alt+↵`). Drop `← Back` (the header already has a back chevron) to keep four.

**Nit (UX):** the file row shows a home-relative path with no indication of whether the file still exists. If files can be moved, show a "missing" state.

## Start here
1. Preview for the highlighted item.
2. Start selection on the first item, not on "Paste all".
3. Trim the footer to four hints.

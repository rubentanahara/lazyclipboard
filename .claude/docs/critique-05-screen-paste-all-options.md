# Critique: 05 Screen · Paste All · Options

**Screen:** configuration step for pasting every item of a group as one block. Reached with `⌘↵` from the group list or the item list. Job: choose order and separator, optionally enable AI, then paste. Seen: order chips, separator chips, "Reformat with AI" toggle (off), a preview of the joined text, a "1 image skipped" note, footer `↵ Paste all`, `⌥A Toggle AI`, `esc Back`.

## Walkthrough

The screen is honest about what will be pasted: the preview shows the joined text, so the user verifies before committing (heuristic 1 and 5 hold). The order/separator/AI stack reads top to bottom in the order the decisions matter.

**Important (UX):** the preview does not change with the separator chips in a way I can verify. It shows four lines with what looks like blank-line spacing whichever chip is selected. The preview must reflect the chosen separator (`, ` as a single comma line, bullets with `•`), otherwise the control feels decorative.

**Important (UX):** the note "1 image skipped" is 12px `text-3` at the bottom, below the fold of attention. A silent omission is a data-loss-adjacent surprise; promote it to a visible row under the preview with the count in the header ("11 of 12 items") and offer "include images as files" if the target app supports it.

**Important (UX):** chips have no explicit default or remembered state. Users who always paste bullets must re-select every time. Persist the last choice per group.

**Important (UI):** two chip rows plus a toggle row plus a preview are the same visual weight, so there is no primary. Make the footer's `↵ Paste all` a real primary button on the panel, with the shortcut in its label, so the end of the flow is obvious.

**Nit (UI):** the toggle-off track uses `text-3`, which is correct for contrast (5.8:1), but the row background is `surface-alt`, so "off" and "disabled" look alike. Add the word "Off" or keep the sparkles icon `text-3` when off.

**Nit (UI):** "Newest first / Oldest first" are two chips; a two-option segmented control communicates exclusivity better.

## Start here
1. Live separator preview.
2. Promote the skipped-image notice.
3. Add a primary "Paste all" button.

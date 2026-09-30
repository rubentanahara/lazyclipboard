# Critique: 05 Screen · Paste · Pick Group

**Screen:** first step of paste, opened by the global paste shortcut. Job: choose which group to paste from, or paste everything in a group without opening it. Seen: a search field, five groups with counts and recency, a "Paste all" chip on the highlighted row, `⌘n` and chevron per row, footer with `→ Open` and `⌘↵ Paste all`.

## Walkthrough

This screen carries the most decisions per pixel in the app: open a group, filter across groups, or paste all. Layout copes: search on top, rows below, actions in the footer. The added "Paste all" chip is the right kind of shortcut, because it removes a step for the common "dump the whole group" case, and it appears only on the highlighted row so the list stays calm (heuristic 8, minimalist design, holds).

**Important (UX):** the row now carries five things: icon, name and meta, "Paste all" chip, number key cap, chevron. The chip and the key cap both mean "shortcut", but only the chip has a label. A first-time user cannot tell that `⌘1` jumps to the group while `⌘↵` pastes all. Either drop the number cap on the highlighted row, or label the chip with its key ("Paste all ⌘↵") so the mapping is explicit.

**Important (UX):** the chevron and `→ Open` say "enter this group", while `↵` is not described anywhere. What does Enter do here? I can't tell from the frame. State it in the footer ("↵ Paste latest" or "↵ Open"), because Enter is the key users press first.

**Important (UI, accessibility):** selection is a tint only (about 1.15:1 against the surface). Same fix as the other panels: leading accent bar.

**Nit (UI):** the search field placeholder "Search groups & items" says items are searched, but nothing shows that typing switches the panel into results mode. The footer should gain "type to search" while the field is empty.

**Nit (UI):** metadata "12 items · updated 2m ago" is 12px `text-3`; it passes (5.0:1 on the tint) but this is the row's only differentiator between groups. Consider showing a one-line preview of the newest item instead of the count.

## Start here
1. Resolve what `↵` does and say it in the footer.
2. Label the Paste-all chip with its key and drop the number cap on the highlighted row.
3. Leading accent bar for selection.

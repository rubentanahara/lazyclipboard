# Critique: 05 Screen · Paste · Search

**Screen:** results mode of the paste panel after typing "spacing". Job: find one item across every group and paste it. Seen: query with caret, "3 results", rows with type icon, wrapped content, source and time, group tag on the right, footer `↵ Paste`, `⌘⌫ Clear`, `esc`.

## Walkthrough

Group tags on the right answer the question search always raises, which group is this from, without extra clicks. The results count and the visible caret make status obvious (heuristic 1 holds).

**Important (UX):** matches are not highlighted. The query is "spacing", but the eye has to hunt for it inside two-line content. Bold or tint the matched term (`accent-soft` background, `text` colour).

**Important (UX):** the panel has no filter for content type or group, and there is no empty state visible ("no results for 'x'"). Design the zero-results row with the suggestion to widen the query.

**Important (UI):** the first result wraps to two lines with an ellipsis ("aligned…") while the tag stays on the right; the second wraps to two lines without truncation. Row heights are inconsistent. Pick one rule: max two lines, then ellipsis.

**Important (UI, accessibility):** selection is tint-only. Same fix.

**Nit (UI):** the group tag is 11px accent text on `accent-soft`; it passes (5.5:1) but is the smallest text on the panel and carries real information. Raise to 12px.

**Nit (UX):** `⌘⌫ Clear` is easy to confuse with deleting an item. Label it "Clear search".

## Start here
1. Highlight the matched term.
2. Design the no-results state.
3. Equalise row height rules.

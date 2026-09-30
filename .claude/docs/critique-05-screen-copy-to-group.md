# Critique: 05 Screen · Copy to Group

**Screen:** floating panel (520px) opened by the global shortcut after the user selects content in another app. Job: file the selection into a group in one keystroke. Seen: captured-content strip, five groups with `⌘1`–`⌘5`, "New group…", key-hint footer. Not visible: empty state, long-content state, a group list longer than five.

## Walkthrough

The screen answers its one question well: what did I grab, and where should it go. The captured strip sits above the list on a tinted surface, so the user sees the thing before choosing a home for it (*UX*, good: recognition over recall). Number hints on rows make the whole task one chord, which is exactly right for a utility invoked mid-work (heuristic 7, flexibility and efficiency, holds).

The problems are about state and scale. **Important (UX):** the list shows five groups and numbers them 1–5. I can't tell what happens with a sixth group; the panel offers no type-to-filter, so power users with 15 groups are stuck arrowing. Add a filter row that appears on first keystroke, and only number the first nine. **Important (UX):** "New group…" leads to a separate naming screen. That is fine, but the row gives no hint of the shortcut until you read the footer; move `⌘N` onto the row as a key cap.

**Important (UI, accessibility):** the selected row differs from the others only by an `accent-soft` tint (`#ECEEFF` on `#FFFFFF`, about 1.15:1). Selection is a state the user must see; the tint alone falls far under the 3:1 non-text threshold, and the accent folder icon is small. Add a 2px `accent` bar on the row's leading edge (accent on surface is 6.3:1) and keep the tint.

**Nit (UI):** the "CAPTURED" overline is 10px `text-3`; the metadata line "Text · Safari" is 11px. Both pass contrast but sit at the floor of legibility. Raise the overline to 11px and the metadata to 12px.

**Nit (UX):** no confirmation that pressing `↵` will save into the highlighted group; the hint says "Save", which is correct. Consider naming the target: "Save to Design refs".

## Start here
1. Add a leading accent bar to the selected row (all list panels share this).
2. Add type-to-filter and cap numeric hints at nine.
3. Raise 10/11px text to 11/12px.

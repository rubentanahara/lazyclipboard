# Critique: 05 Screen · Paste · AI Reformat

**Screen:** single-item AI reformat, reached with `⌥↵` from the item list. Job: rewrite the selected item with AI and paste either version. Seen: original in a grey box, prompt chips (Default, Fix grammar, Shorter, Bullet list), the result in an accent-soft box, footer `↵ Paste result`, `⌘↵ Paste original`, `⌘R Regenerate`, `esc Back`.

## Walkthrough

The vertical order (original, prompt, result) mirrors the mental model, and putting the result in a tinted box with a sparkles icon and a "RESULT" label keeps AI content identifiable without relying on colour alone. Offering "Paste original" is the right escape hatch (heuristic 3, user control, holds).

**Critical (UX):** every AI state except success is missing. I can't tell from the frame what the user sees while the model is running, when the API key is missing or invalid, when the request fails, or when the provider is slow. A paste tool that hangs silently after a shortcut is a broken tool. Design at least loading (skeleton plus "Reformatting…" and a cancel), no-key (link to Settings), and error (message plus Retry) states.

**Important (UX):** selecting a prompt chip appears to re-run the model, but nothing says so. Show a spinner on the chip or the result while it regenerates.

**Important (UX):** the result is not editable. Small manual fixes are the most common follow-up; allow editing in place or add "Edit" to the footer.

**Important (UI):** the four chips are outlined with a 1px border; the unselected ones read as disabled. Selected uses solid accent, unselected should use `surface-alt` fill so all four look pressable.

**Nit (UI):** the model chip "Anthropic" in the header is decoration. If the provider matters, make it a menu; otherwise remove it.

**Nit (UX):** "AI can make mistakes" appears only on the paste-all variant; add it here or nowhere.

## Start here
1. Loading, no-key and error states.
2. Editable result.
3. Fix chip affordance for unselected chips.

# Critique: 05 Screen · Paste All · AI Result

**Screen:** the options panel with AI on, showing a merged, bulleted result of twelve items. Job: review the model's combined output, then paste it or fall back. Seen: toggle on, prompt chips (Merge & clean up, Bullet list, Summarise, Custom…), a "RESULT" box with four bullets, "AI can make mistakes" note, footer `↵ Paste result`, `⌘↵ Paste without AI`, `⌘R Regenerate`.

## Walkthrough

Reusing the options panel keeps the mental model intact: turn AI on and the result appears. The disclaimer, the `RESULT` label and the sparkles icon mark AI output without colour alone. Offering "Paste without AI" preserves control (heuristic 3).

**Critical (UX):** twelve items collapsed into four bullets. Nothing says what was dropped or merged. A user pasting a summary of their own content needs a way to check fidelity: show "12 items → 4 points", a "Show source items" expander, and make the result editable. Without this, the feature is a black box on the user's data.

**Critical (UX):** as with the single-item AI screen, loading, failure and missing-key states are absent. Twelve items is a larger request, so the wait is longer; a progress state with cancel is required.

**Important (UX):** `⌘↵` means "Paste without AI" here and "Paste original" on the single-item screen, and "Paste all" on the group screens. Three meanings for one chord. Give the escape hatch one consistent shortcut.

**Important (UX):** privacy. The user is about to send twelve clipboard items to a third party. The frame shows no provider name, no size, no warning that items may contain secrets. Show "Sends 12 items to Anthropic" near the toggle and offer a per-group "never send to AI" setting.

**Important (UI):** the panel is 560px and taller than the others (about 690px); on a 13-inch display it crowds the screen. Collapse the order and separator chips into one line when AI is on.

**Nit (UI):** "Custom…" is a chip that opens something invisible. Show a text field or a menu affordance.

## Start here
1. Loading, error and no-key states.
2. Fidelity: item count, source expander, editable result.
3. Data-sharing notice before sending twelve items.

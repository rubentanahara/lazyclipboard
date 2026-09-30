# Design Review: icons pass and canvas conformance

Reviewed against: DESIGN.md (ten sections, incl. new Iconography), 00 Brief, 01 Journey, 03 Tokens · Icons
Date: 2026-09-30

## Summary

The canvas conforms: across all 86 screen frames there are no hex fills, no literal font sizes below 12, no off-scale icons, no literal radii, gaps or padding, and 59 reusable components. The one real gap is eight multi-line text nodes that carry a literal line height of 1.45 instead of `$leading-body`. The icon set is on its own board and every glyph is on-scale; two small contract gaps remain between DESIGN.md and what the canvas uses.

Method: visitors over 43 light screens and their 43 Dark refs, the four `03 Tokens` boards and the new icons board; screenshots of Main Window, Settings · General, Paste All · AI Result, Tray Menu, Confirm Dialog, Copy Notification, Search No Results, and the Windows Copy to Group and Linux Paste All flows (board scale only).

## Must fix

None.

## Should fix

1. **Literal line height 1.45 on eight text nodes** — `Main Window > Preview`, `Main Window · Search > Preview`, `Paste · AI Reformat > Original box` and `> Result box`, and `Settings · AI` (plus its Test Failed, Test Succeeded and Testing variants) `> Prompt field`. _Fix: set `lineHeight: "$leading-body"` on each; `State Message` in the State Box already does._
2. **DESIGN.md Iconography omits two colour tokens the canvas uses** — `text` (icons on Secondary buttons: `ellipsis`, `plug-zap`, `rotate-ccw`, `loader`) and `success` (`circle-check` in Test Status Row). _Fix: add both to the Colour paragraph, `text` for icons inside a labelled control, `success` for a passed state._
3. **`Icon Button` has a bare `pencil` glyph, no label, and zero instances.** It is speculative, and its default is exactly the icon-only rename control the standing rule forbids. _Fix: delete it, or give it a required label and tooltip slot before anything instances it._

## Could improve

1. **`image-off`, `info`, `link` are raw icons in one-off layouts** (Paste All Options ×2, AI Result note, Main Window Search). Correct under the third-use rule; revisit if a third screen needs any of them.
2. **`03 Tokens · Icons` is static text.** Sizes, colours and owners go stale when icons change. Re-run the `design-icons` enumeration (with `resolveInstances`) after any icon edit, or before handoff.
3. **`03 Tokens · Spacing, Radius, Elevation` still has 8 literal radii** in specimen bars (carried over from the last review).
4. **Five screens report "clipped" nodes** (Tray Menu, Tray Menu · No Groups, Copy Notification, Copy Notification · Error, Settings · General). All are the hidden Windows/Linux variants toggled by `enabled` variables; a visitor false positive, not a layout fault. Filter on `enabled` in future scripts.
5. **Windows and Linux flows were checked at board scale only.** Structure and lanes are intact; label-level review needs a zoomed screenshot per lane.

## Journey conformance

Unchanged since `design-review-full-canvas.md`: all six opportunities addressed. This pass touched icons and one empty state only. Opportunity 3 (No results names a next step) is now also built from the State Box component with a "Clear search" action, so the no-results screen follows the same pattern as every other empty state.

## What works

- **Icon discipline landed cleanly.** 52 glyphs, one library, every size on the 12/14/16/18/24/32 scale, every colour a token. The slot-sets-the-size rule explains why `folder` has four sizes without being drift.
- **State Box now covers search no-results**, so empty and error states share one component from the tray to the AI screens.
- **Token boards reference variables throughout**: zero literals on Color, Type and the new icons board.
- **Dark refs exist for all 43 screens**, so the theme axis is covered end to end.

## Promotion into DESIGN.md

No finding recurred across three screens, so no new rule. The eight literal line heights share one cause (multi-line text boxes built before `leading-body` existed); if another appears, add to **Do not do**: "Do not set a numeric line height; bind `leading-body`."

## Status after applying the findings (2026-09-30)

- **Should 1** fixed: the eight multi-line text nodes now bind `$leading-body`. That variable is 1.4, so those boxes tightened from 1.45.
- **Should 2** fixed: DESIGN.md Iconography now lists `text` and `success`.
- **Should 3** fixed: `Icon Button` deleted (zero instances); removed from the Components table in DESIGN.md.
- **Could 1** partly applied: the three Main Window · Search rows were raw frames duplicating `Search Result Row` and now use it (the `link` icon is covered by an override). `image-off` (two identical Notes) and `info` (a different, plain caption) stay raw: two uses, below the third-use threshold.
- **Could 2** done: the icons board was re-read; 3 texts updated (Used by, after the Icon Button deletion and the row swap).
- **Could 3** fixed: new `rounded-xs` (2) token binds the eight spacing specimen bars; added to DESIGN.md Radius.
- **Could 4** addressed in the `design-review` skill: hidden-variant false positives are now documented.
- **Could 5** checked: Windows Copy to Group and Linux Paste All flows exported and read at 1:1 scale. Platform chords (`Win+Shift+C`, `Ctrl+J/K`, `Ctrl+1`, `Ctrl+Alt+V`) and OS chrome are correct.
- **New finding, not fixed:** `Item Row`'s Thumbnail placeholder carries a literal gradient (`#C7D2FE` to `#FBCFE8`). The earlier "no hex fills" check was string-only and missed it. It is an image placeholder; bind it to tokens only if Pencil gradients accept variables.

## Final pass (2026-09-30)

- **Could 1** completed: new `Note` component (`04 Components` > Feedback, "Cell Note (base: image skipped)") replaces the two `image-off` notes and the `info` footnote (footnote = `fill` surface, padding 0, action disabled). Eleven single-use icons remain in screen layouts after the component pass below.
- **Gradient** fixed: thumbnail placeholders bind to new themed variables `thumb-from` and `thumb-to` (dark values `#3B4270`, `#6E3A5A`), verified in light and dark. DESIGN.md "No literal colours" updated; only macOS traffic lights stay literal.

## Componentisation pass (2026-09-30)

A structural scan of the 43 light screens (raw frames with the same child shape, 3+ times) produced ten new components: Menu Item (+ Selected, Danger), Detail Row, Overline Group (with a content slot), Options Summary, Field Head, Shortcut Row, Captured Strip, Reformat Header, Source Row and Prompt Field. About 70 raw frames became instances; every touched screen reports no layout problems.

**Incident:** swapping the 16 overline groups first cloned their content with a depth-1 `Get`, which returns children as `"..."`, so chips and text boxes were dropped and the old nodes deleted. Content was rebuilt from the original build transcripts (Original text, chip sets and selections, preview lines) and checked against earlier screenshots of Paste All Options and AI Result. The AI Reformat chip selections (Default selected) come from the transcript, not a screenshot; worth a glance.

Scan leftovers, not extracted: Preview pane (2 uses), Name input (2), Result box, Back link, Edit link (1 each).

# Design Review: flows, edge states, Settings resize

Reviewed against: DESIGN.md (no 00 Brief, 01 Journey or 02 IA boards exist on the canvas)
Date: 2026-09-30

## Summary

Token discipline holds: 0 unresolved variable references, 0 hex fills outside the macOS traffic-light chrome, 0 text under 12px in any screen, 0 real clipping in screens. The biggest gap is a broken standing rule: Paste All with AI has a loading state but no error or no-key state, so the "no AI action without loading, error and no-key" rule fails on that path.

## Must fix

1. **Paste All AI has no error or no-key state** — `05 Screen · Paste All · AI Loading` exists, but a failed request or missing key has no screen, and `06 Flow · Paste All from Group` edge band stops at loading → result. Breaks "Do not ship an AI action without loading, error and no-key states". _Fix: add `Paste All · AI Error` (retry, paste without AI, reason) and `Paste All · AI No Key` (Open Settings, paste without AI), then draw both in the Flow 3 edge band._
2. **Flow 4 ends on a placeholder** — `06 Flow · Manage Items and Groups`, cell "Main Window after group deleted" is a "Not designed yet" box. _Fix: design `Main Window · Group Deleted` (next group selected, Undo or empty state if it was the last group) and swap the placeholder for a ref._
3. **Windows default `Win+Alt+V` is unverified** — carried from the handoff. _Fix: verify free on Win 11 before the spec, or pick another default._

## Should fix

1. **10px annotation labels in `04 Components`** ("Label Foundations", "Label Buttons", "Label Inputs" and 3 more) — below the 12px minimum; 11px is allowed only for uppercase overlines. _Fix: set them to `$text-overline` uppercase or `$text-caption`._
2. **Windows and Linux variants of the new work are unchecked** — Flow 2 and Flow 3 edge bands and all of Flow 4 were screenshot-verified in light macOS (Flow 4 also dark macOS) only. Label widths differ per OS (`Ctrl+Shift+↵ paste original` is wider than the 200px arrow). _Fix: screenshot the Windows · Light refs of Flows 2–4 and widen arrows or the gutter if labels wrap._
3. **Main Window · Search, `Segment > Content`** measures 116+224=340 inside a 317px parent (partially clipped by the visitor). Screenshot shows no visible cut. _Fix: check whether the node is a disabled OS variant; if visible, set the width to `fill_container`._
4. **Settings screens now match Main Window (1100×720)** and every tab has empty body space below its content (General, Shortcuts, AI most). _Fix: accept as room for growth, or add the missing rows the space implies (shortcut reset, "Test connection succeeded")._
5. **Board titles, font weights and stroke widths are still literal** (32px flow titles, weights, 1px strokes). _Fix: bind to `$text-display` and `$weight-bold`, add a `stroke-hairline` token._
6. **No "Test connection succeeded" state** in Settings · AI. _Fix: add the state beside `AI · Test Failed`._

## Could improve

1. **Four swatch cells in `03 Tokens · Color` overshoot by under 1px** — _snap to the cell grid._
2. **Fixed heights on some screens** may not fit content after edits — _switch to `fit_content(<height>)` where content is variable._
3. **Flow arrows use one accent for every branch** — failure paths (`request fails`, `esc cancel`) read the same as the happy path. _Pair failure arrows with a `danger` label and icon, never colour alone._
4. **Visitor false positives** — hidden Windows and Linux variant text (`·win`, `·lnx`, Linux top-bar nodes in Tray Menu and Copy Notification) report "clipped" because `enabled` is a variable string. Not defects.

## Journey conformance

No `01 Journey` board is on the canvas, so no opportunity could be checked. The handoff's open items and the 12 critiques stood in as the pain-point list: AI loading, no-key, error, no-results, destructive confirmation, and undo are now each drawn in a flow (AI Reformat error and no-key in Flow 2; delete group and item deleted in Flow 4; Paste All loading and sources in Flow 3). Paste All AI failure remains open (Must fix 1).

## What works strongest

- Every value is a token; the visitor found no stray hex, no unresolved variable, and no text under 12px.
- Flow boards are built from real screen refs, so an edit to a screen shows in all 24 flow variants. Edge states sit in a titled band under each flow rather than crowding the happy path.
- Entry points are stated in step captions ("AI Reformat · no API key") and every new arrow carries the user action.
- Canvas order is now tokens → components → screens → flows with 120px row pitch and no overlaps.

## Status after the fix pass (2026-09-30)

- **Must 1** fixed: `Paste All · AI Error` and `Paste All · AI No Key` (light and dark) drawn in the Flow 3 edge band.
- **Must 2** fixed: `Main Window · Group Deleted` (empty state and confirmation toast) replaces the Flow 4 placeholder.
- **Must 3** resolved: `Win+Alt+V` is absent from Microsoft's shortcut list; recorded in DESIGN.md.
- **Should 1** fixed: 10px annotation labels now use `text-overline` (11px). **Should 2** checked: Windows label variants render inside the 200px arrows.
- **Should 3** fixed: the Search row text segment was shortened to fit. **Should 4** accepted as room for growth.
- **Should 5** fixed: weights bind to `fw-*`, strokes to `stroke-*`, 10/12/14/32px sizes to tokens (1,103 nodes).
- **Should 6** fixed: `Settings · AI · Test Succeeded` added (icon, message, neutral field border).
- **Could 1** not reproducible: no swatch overshoot found. **Could 2** verified: no screen content overflows its fixed frame. **Could 3** fixed: failure arrows carry a `circle-alert` icon and `danger` colour.

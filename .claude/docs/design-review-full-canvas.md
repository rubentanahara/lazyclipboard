# Design Review: full canvas (screens, flows, Vim mode, Brief, Journey, IA)

Reviewed against: DESIGN.md, `00 Brief · lazyclipboard`, `01 Journey · Mara · Research to reuse`, `02 IA · Screen Inventory`
Date: 2026-09-30

## Summary

The canvas conforms to its contract: across all 44 top-level frames the visitors found 0 hex fills outside the macOS traffic lights, 0 text under 12px, 0 literal sizes, weights, strokes or spacing, and 0 unresolved variable references. The IA inventory matches the canvas (32 designed screens, 10 declared gaps, none undeclared). The biggest finding is journey conformance: the AI-trust opportunity is only half built (no "never send this group to AI" setting), and the copy-failure toast that the Journey asks for is still an IA gap.

## Must fix

1. **"Never send to AI" group setting is missing** — Journey opportunity 4 (`01 Journey`, Combine with AI) asks for a per-group opt-out beside the "Sends N items" notice. The notice exists (`Paste All · Options`, `AI Loading`, `AI Error`, `AI No Key`), the opt-out exists on no screen, including `Settings · Groups`. _Fix: add a "Never send to AI" toggle per row in `05 Screen · Settings · Groups` (label, not icon-only) and a locked state on `Paste All · Options` ("AI off for this group") with a link to Settings._
2. **Copy failure has no feedback** — Journey opportunity 2 ("Nothing to save" failure toast) and the IA gap `Copy Notification · Error`. A global-shortcut copy that captures nothing gives the user silence. _Fix: draw `05 Screen · Copy Notification · Error` from the `Notification Toast` component with a `danger` icon and text, then add it to Flow 1 as the failure terminal._
3. **Flow 1 draws no failure branch** — `06 Flow · Copy to Group` has only success and new-group paths; the "Cancelled / Failure terminals" rule of design-flows is unmet there. _Fix: add esc-cancel and capture-failed branches once item 2 exists._

## Should fix

1. **Undo and delete toasts are hand-built, twice** — `Main Window · Item Deleted` (Undo toast) and `Main Window · Group Deleted` (Deleted toast) each build an inverse-surface toast from raw frames, while `04 Components` has only `Notification Toast`. The Brief lists "Undo Toast" and "Delete Group Dialog" as New components but neither is a component. _Fix: extract `Undo Toast` (message, optional count, Undo action, lifetime) and `Confirm Dialog` into `04 Components` and replace both screens' local frames with instances._
2. **Vim hints are absent from the panels** — Vim mode is on by default, yet the footers of `Paste · Pick Group`, `Pick Item`, `Search` still read "↑↓ Navigate". Users who never open Settings never learn `Ctrl+J/K`. The 520px footer has about 32px spare. _Fix: when Vim is on, show one cap per hint ("⌃J⌃K Navigate") and drop "Type to search" to a placeholder hint in the search field; keep the arrow-key footer on the `Vim Off` variants._
3. **Vim Off dims keycaps to 40% opacity** — `05 Screen · Settings · Shortcuts · Vim Off`. Dimmed caps and `text-2` labels fall below 3:1 and read as disabled rather than "not active"; the shortcut is still information. _Fix: keep the row labels at `text` and mark the inactive column with a "Vim keys off" caption instead of opacity._
4. **Undo lifetime pause is not shown** — Journey opportunity 2 asks for the timer to pause on hover; `Main Window · Item Deleted` shows "6s" only. _Fix: add a hover state (countdown paused, "Paused" caption) beside it, or state the rule in DESIGN.md Interaction behaviour._
5. **Ten IA gaps are undrawn** — Tray Menu No Groups, Copy to Group Empty and New Group, Copy Notification Error, Pick Group Empty, Pick Item Empty, Main Window Empty Group and Rename Group, Settings Groups Delete Group confirm, Settings AI Testing. Empty and confirm states are the ones users hit on first run. _Fix: build in priority order P1 first (Tray No Groups, Pick Group Empty, Copy Notification Error), reusing the empty-state pattern from `Main Window · Group Deleted`._
6. **Board content is unreadable in thumbnails** — `00 Brief`, `01 Journey`, `02 IA` use 12px text on 1500–2200px boards; screenshots at board size render as grey lines. Not a defect on the canvas, but it blocks review by image. _Fix: none needed; review at zoom, or export at scale 2._

## Could improve

1. **`Brief` accent bars are fixed 128px** (`Row · Principles`) — they will not grow if a principle's text lengthens. _Bind the bar to the row with `fill_container` height inside a horizontal frame._
2. **`03 Tokens · Spacing, Radius, Elevation`** has 8 literal radii (2px specimen `Bar` nodes). _Bind to a `rounded-xs` token or leave as documented specimens._
3. **Tray Menu and Copy Notification report 10 to 11 "fully clipped" nodes** — these are the hidden Linux top-bar variants (`enabled` is a variable), a visitor false positive. _Filter on `enabled` in future review scripts._
4. **Platform key labels are wider than the arrows on Windows** — the label text is fixed-width and wraps at 176px; screenshots show it fits. _If more labels are added, widen the arrow column instead of wrapping._
5. **Sort order** — Journey friction "No sort control; the order is not stated": Pick Item states "Newest first", Main Window does not offer a sort control. _Add a labelled sort control to the `Main Window` group header._

## Journey conformance

| Journey opportunity | Status |
| --- | --- |
| 1 Capture from the shortcut, selection survives | Addressed (Copy to Group flow); failure toast open (Must 2) |
| 2 Undo lifetime, "Nothing to save", conflict warning | Undo lifetime and conflict warning addressed; failure toast and hover pause open |
| 3 ↵ stated in footer, highlight, preview, No results next step | Addressed (Pick Item preview, Search highlight, No Results) |
| 4 "Sends N items" notice, never-send setting, Test connection, sources expander | Notice, Test connection (failed and succeeded), sources addressed; never-send open (Must 1) |
| 5 Skipped items count, one chord for paste original | Addressed (11 of 12 items, `⌘⇧↵` everywhere) |
| 6 Delete: Undo toast, group delete confirm, labelled menu | Addressed (Actions Menu, Delete Group, Item Deleted, Group Deleted) |

## What works

- **Token discipline is complete**: the whole canvas passes every measurable check, including the boards built by parallel agents.
- **The IA inventory is honest**: every screen on the canvas is listed and the ten gaps are named rather than hidden.
- **Every AI path now has loading, error and no-key states** in both AI flows, each with "paste without AI" reachable, which the DESIGN.md "Do not do" rule requires.
- **Vim mode is scoped sensibly**: Ctrl-based so type-to-search still works, on by default, arrow keys never removed, and documented with the reason in DESIGN.md.
- **Flows are live instances**: 4 flows across 3 OS and 2 themes update with the screens.

## Promotion into DESIGN.md

No finding recurred across three or more screens, so nothing is proposed as a new rule. Should-fix 1 (hand-built toast) appeared twice; if it appears a third time it qualifies as "Do not hand-build toasts, use `Undo Toast` / `Notification Toast`".

## Status after the fix and componentisation pass (2026-09-30)

- **Must 1** fixed: Settings · Groups has a labelled "Never send to AI" toggle per group; `Paste All · Options · AI Off` shows the locked state.
- **Must 2** fixed: `Copy Notification · Error` (light and dark). **Must 3** fixed: Flow 1 has a "no selection" failure lane and an `esc` cancel lane.
- **Should 1** fixed: `Toast Inverse` (Undo, Paused, Info) and `Confirm Dialog` are components; Item Deleted, Group Deleted and both delete screens use them.
- **Should 2** fixed: the Navigate hint shows `⌃J/K` (`Ctrl+J/K`). The Pick Group "type to search" hint was dropped because the Windows footer overflowed 520px.
- **Should 3** fixed: Vim Off keeps full opacity; state is carried by the toggle and its note. **Should 4** fixed: "Paused" toast state on the board.
- **Should 5** fixed: all 10 IA gaps are drawn (43 designed screens, each with a Dark ref); inventory and sitemap updated.
- **Componentisation**: Window Titlebar, Nav Item, Footer Hint, Panel Header (+ Detail), Group Row, State Box, Result Head, AI Toggle Row, Confirm Dialog, Toast Inverse, Section Heading, Choice Chip, Select, Shortcut Field, Button Small, Binding Row now live on `04 Components` with their states and are instanced in the screens.
- **Second pass**: OS chrome, Search Result Row, Item Row and thumbnail rows, Pick Item and Search headers, and the Settings controls and rows (Settings Row, Group Settings Row, Stepper, Segmented Control, Secret Field, Test Status Row) are components. Only layout containers (panes, lists, card shells, popover slots) stay raw.
- **Known limit**: `width` and `height` cannot bind to `size-*` variables in this Pencil version.

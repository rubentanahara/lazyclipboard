---
name: lazyclipboard
description: Keyboard-first macOS clipboard organizer. Cool neutral surfaces, one indigo accent, dense 13px type.
version: alpha
colors:
  light:
    bg-desktop: "#E4E7EE"
    surface: "#FFFFFF"
    surface-alt: "#F4F5F8"
    surface-raised: "#FFFFFF"
    surface-inverse: "#16181D"
    border: "#E2E5EB"
    border-focus: "#4F46E5"
    text: "#16181D"
    text-2: "#4A5160"
    text-3: "#5F6675"
    text-on-accent: "#FFFFFF"
    text-on-inverse: "#FFFFFF"
    accent: "#4F46E5"
    accent-hover: "#4338CA"
    accent-active: "#3730A3"
    accent-soft: "#ECEEFF"
    success: "#15803D"
    warning: "#B45309"
    danger: "#C81E1E"
    info: "#0369A1"
    overlay: "#16181D66"
    shadow-color: "#16181D26"
    menubar: "#FFFFFFB3"
    selection: "#B4D5FE"
  dark:
    bg-desktop: "#0E1117"
    surface: "#171A21"
    surface-alt: "#1F232C"
    surface-raised: "#262B36"
    surface-inverse: "#E6E8EC"
    border: "#2E3440"
    border-focus: "#8B87FF"
    text: "#E6E8EC"
    text-2: "#A9AFBB"
    text-3: "#9096A4"
    text-on-accent: "#0E1117"
    text-on-inverse: "#0E1117"
    accent: "#8B87FF"
    accent-hover: "#A5A2FF"
    accent-active: "#BDBBFF"
    accent-soft: "#262A55"
    success: "#4ADE80"
    warning: "#FBBF24"
    danger: "#F87171"
    info: "#38BDF8"
    overlay: "#00000099"
    shadow-color: "#00000066"
    menubar: "#171A21CC"
    selection: "#3B4A8A"
typography:
  overline: { fontFamily: Inter, fontSize: 11px, fontWeight: 600, lineHeight: 1.2 }
  caption: { fontFamily: Inter, fontSize: 12px, fontWeight: 400, lineHeight: 1.4 }
  small: { fontFamily: Inter, fontSize: 12px, fontWeight: 400, lineHeight: 1.4 }
  body: { fontFamily: Inter, fontSize: 13px, fontWeight: 500, lineHeight: 1.4 }
  ui: { fontFamily: Inter, fontSize: 14px, fontWeight: 500, lineHeight: 1.4 }
  title: { fontFamily: Inter, fontSize: 15px, fontWeight: 600, lineHeight: 1.4 }
  heading: { fontFamily: Inter, fontSize: 20px, fontWeight: 600, lineHeight: 1.2 }
  display: { fontFamily: Inter, fontSize: 28px, fontWeight: 600, lineHeight: 1.2 }
spacing: { 2xs: 2px, xs: 4px, sm: 8px, md: 12px, lg: 16px, xl: 24px, 2xl: 32px, 3xl: 48px }
rounded: { sm: 6px, md: 8px, lg: 12px, xl: 14px, pill: 9999px }
elevation: { menu-y: 12, menu-blur: 32, panel-y: 16, panel-blur: 40 }
---

# DESIGN.md

## Product design direction

lazyclipboard is a keyboard-first clipboard organizer for macOS, Windows and Linux. The system clipboard holds one thing; this product lets you copy into a named group with a global shortcut, then paste from a group through a floating panel, with optional AI reformat and a Paste All that joins a group's items (order, separator, optional AI summary) into one block.

It is for people who collect text, links and images across windows while they work: developers, writers, researchers. They already live on shortcuts, so the panels are opened by shortcut, read in under two seconds and closed with esc.

It must feel like a quiet, fast utility that gets out of the way: closer to Raycast or Spotlight than to a dashboard, a chat window or a skeuomorphic clipboard board. The brand is the canvas tokens (cool neutral surfaces, one indigo accent, Inter, a designed dark mode), so no visual archetype is layered on top. Out of MVP: cloud sync and accounts, sharing between people, mobile and web clients, editing item content or images, an automatic log of everything copied. Board: `00 Brief · lazyclipboard`.

## Visual principles

- **Shortcuts are shown, not hidden** (discoverability against speed). Every action carries its key cap in the row or footer, so a new user learns by looking and a power user never has to. Every shortcut is remappable; one chord, one meaning.
- **Density over whitespace** (dense against approachable). Panels are read in under two seconds, so 13px type, 54px rows and five visible rows win ties against web-style breathing room. Whitespace is spent in the main window and in empty states.
- **Cheap to undo, costly to lose** (speed against safety). Deleting an item is immediate with an Undo toast of about 6s; deleting a group asks first and names the item count. AI never replaces the original silently: it is labelled and has loading, error and no-key states.

## Colour system

### Role mapping

The canvas variables predate this file, so the canonical roles map onto the existing names. Do not rename them; every screen references these.

| Role | Token | Use |
| --- | --- | --- |
| canvas | `bg-desktop` | The simulated desktop behind floating panels and the app window backdrop. |
| surface | `surface` | Panels, windows, list rows at rest. |
| surface-sunken | `surface-alt` | Captured-content strip, footers, key caps, sidebar, input fields. |
| surface-raised | `surface-raised` | Menus and popovers. Equals `surface` in light; one step lighter in dark. |
| surface-inverse | `surface-inverse` | Tooltips and toasts only. |
| border | `border` | Dividers and card edges. Decorative — never the only signal of a control. |
| border-focus | `border-focus` | Keyboard focus ring and the active text input. |
| text-primary | `text` | Content and labels. |
| text-secondary | `text-2` | Supporting lines, descriptions, hints. |
| text-muted | `text-3` | Metadata (source app, time, counts) and overline labels. Still passes 4.5:1. |
| accent | `accent` / `accent-hover` / `accent-active` | The selected row, the primary button, links, toggle-on. |
| accent-soft | `accent-soft` | Selected-row background, AI result box, group tags. |
| status | `success` `warning` `danger` `info` | Feedback only. |

### Rules

- **One accent.** Indigo is the only brand hue. If a second colour is needed to tell two things apart, use weight, an icon or position first.
- **Selection is accent-soft, a 2px accent bar on the leading edge, and accent text or icon.** The tint alone is about 1.15:1 against the surface and never counts as the signal. Never accent as a solid row fill; solid accent is reserved for the one primary action in a view and for toggle-on.
- **Colour is never the only signal.** Danger always pairs with a trash icon or the word "Delete"; a selected row also carries the `↵` hint; AI content carries the sparkles icon and a "RESULT" label.
- **Text tokens are solved against the darkest surface they land on**, not against the page. `text-3` was solved on `accent-soft`, not on `surface`.
- **Dark mode is designed, not inverted.** Cool slate surfaces (`#0E1117` → `#262B36`); text is off-white `#E6E8EC`, never `#FFFFFF`. Accents are lightened (`#8B87FF`) and `text-on-accent` flips to near-black so the button keeps 6:1.
- **Elevation in dark is lightness, not shadow.** Each level is a lighter surface: `bg-desktop` → `surface` → `surface-alt` → `surface-raised`. Shadows stay in the file for light mode and are near-invisible on dark.
- **Hover and active are lighter in dark, darker in light.** `accent-hover` and `accent-active` exist for every accent-filled control; a button without both is incomplete.
- **Theme is a user setting: Light, Dark or System.** Settings → Appearance holds a three-way segmented control. System follows macOS and switches live. Screens set no theme of their own; the app applies `mode` at the root. On the canvas, a frame's `theme: {mode: "dark"}` previews dark.
- **No literal colours in screens.** Only macOS window traffic lights stay literal; the placeholder image gradient is bound to `thumb-from` and `thumb-to`, themed light and dark. Toggle knobs use `surface-raised`; toggle-off track uses `text-3`.
- **Alpha is only for `overlay` and `shadow-color`.** Never fake a tint with a transparent fill on a surface.

### Contrast verification (WCAG 2.2 AA)

Worst measured pair per role, against the darkest surface it is used on. Threshold 4.5:1 for text, 3:1 for focus ring.

| Pair | Light | Dark |
| --- | --- | --- |
| `text` on `surface-raised` / `accent-soft` | 15.4 | 11.1 |
| `text-2` on `accent-soft` | 6.9 | 6.2 |
| `text-3` on `accent-soft` | 5.0 | 4.6 |
| `accent` on `accent-soft` (selected row) | 5.5 | 4.5 |
| `text-on-accent` on `accent` | 6.3 | 6.3 |
| `danger` on `surface-alt` | 5.3 | 5.7 |
| `success` on `surface-alt` | 4.6 | 9.0 |
| `warning` on `surface-alt` | 4.6 | 9.4 |
| `info` on `surface-alt` | 5.4 | 7.3 |
| `border-focus` on `bg-desktop` (3:1) | 5.1 | 6.3 |

The original `text-3` (`#8A91A0`) measured 2.9:1 on `surface-alt` and failed; `danger` was tightened from `#DC2626` (4.4:1 on `surface-alt`) to `#C81E1E` to clear 4.5. Dark `accent` on `accent-soft` and `text-3` on `accent-soft` sit within 0.1 of the threshold — do not lighten `accent-soft` or darken `text-3` in dark without re-measuring.

## Typography

One family: **Inter**. Four weights: 400, 500, 600, 700. Nothing else.

| Role | Size | Weight | Line height | Use |
| --- | --- | --- | --- | --- |
| overline | 11 | 600, caps, +0.8 tracking | 1.2 | Section labels (GROUPS, CAPTURED, PREVIEW). |
| caption | 12 | 400 | 1.4 | Key-cap legends, tags, timestamps inside tags. |
| small | 12 | 400 | 1.4 | Metadata lines, footer hints, descriptions. |
| body | 13 | 500 | 1.4 | Item content, sidebar rows, form labels. |
| ui | 14 | 500 | 1.4 | Group names, panel row titles, buttons. |
| title | 15 | 600 | 1.4 | Panel headers. |
| heading | 20 | 600 | 1.2 | Main-window group title. |
| display | 28 | 600 | 1.2 | Empty states and onboarding only. |

### Rules

- **Body is 13px on purpose.** This is a dense desktop utility opened by shortcut and read in under two seconds. Do not bump it to 16 to "match the web"; macOS has no input-zoom rule to satisfy.
- **Minimum size is 11px, and only for uppercase overlines.** Everything else is 12px or larger. Metadata at 12px must use `text-3` or darker, never lighter.
- **Ratio is roughly 1.2 and steps are not interpolated.** No 17px or 22px; pick the nearest role.
- **Weight carries hierarchy before size does.** Prefer 500 → 600 over adding a size.
- **Never bold an item's own content.** Weight 600 is for chrome (headers, selected group names), so user content stays visually distinct from UI.
- **Line length.** Item previews wrap at the panel width (about 60 characters at 13px in a 520px panel) and truncate to one line in lists; the detail pane may wrap freely. Never let a paragraph exceed about 75 characters.
- **Keys use `text-caption` at weight 500** inside `surface-alt` caps, one glyph group per cap (`⌘⌥C` is one cap).

## Layout & spacing

**Base grid: 4px.** Every gap, padding and size is a multiple of 4. Never ship a 10px padding or a 6px gap; round to the grid. The one exception is a 2px half-step (`space-2xs`) for optical tuning inside chips and key caps.

| Token | px | Typical use |
| --- | --- | --- |
| `space-2xs` | 2 | Text-to-icon nudge inside key caps. |
| `space-xs` | 4 | Gap between stacked title and metadata. |
| `space-sm` | 8 | List padding, icon-to-label gap in tight rows. |
| `space-md` | 12 | Row padding, gap between icon and text. |
| `space-lg` | 16 | Panel body gap, footer hint gap. |
| `space-xl` | 24 | Header padding, major group gaps. |
| `space-2xl` | 32 | Settings section padding. |
| `space-3xl` | 48 | Documentation boards only. |

**Radius.** `rounded-xs` 2 (spacing specimen bars only) · `rounded-sm` 6 (key caps, small chips) · `rounded-md` 8 (rows, inputs, buttons) · `rounded-lg` 12 (windows) · `rounded-xl` 14 (floating panels) · `rounded-pill` for tags and toggles. Nested radii step down one level from the container.

**Sizes.** Floating panel 520 (560 when it hosts AI or paste-all options). Main window sidebar 240, detail pane 300. Rows in a list are one line-height pair tall (title + metadata) and never grow past two lines.

**Density.** Compact by default: rows are about 54px, footers 37px, headers 51px. A floating panel must show at least five rows with no scrolling.

**Responsive behaviour.** Fixed-width panels centered on the active display; they never resize with content. The main window collapses the detail pane first (below about 900px), then the sidebar to icons (below about 700px). It reorganises rather than reflows: panes disappear, they do not stack.

### Binding status

Every padding, gap, corner radius, font size, line height, letter spacing, panel width and shadow offset/blur in the mockups is bound to a token. Off-grid literals were snapped to the nearest step (ties round up): 6→8, 10→12, 14→16, 18→16, 20→24, 36→32, radii 5→6, 9→8, 10→12. Chips, tags, toggles and number badges use `rounded-pill`; key caps use `rounded-sm`.

Font weights bind to the string variables `fw-regular`, `fw-medium`, `fw-semibold`, `fw-bold` (the number-typed `weight-*` variables cannot bind `fontWeight`). Documentation-board titles use `text-board-title` (32px). Stroke widths use `stroke-hairline` (1), `stroke-emphasis` (2) and `stroke-bar` (3, the selection bar).

Dark previews are `ref` instances of the light screens with `theme: {mode: "dark"}`, so edits to the source screen propagate. The source screens (`Settings Window`, `Main Window`, `Copy to Group`) are `reusable`.

## Iconography

The set lives on the `03 Tokens · Icons` board: 52 product icons in seven roles (navigation 5, actions 19, appearance 3, status 12, content types 5, AI 1, platform chrome 7), each shown with the sizes and colour tokens it appears in and the components that use it. The count includes icons set through instance overrides (State Box, Button, Nav Item), which a plain walk of the component frames misses; enumerate with `resolveInstances`. Pick from the board by role; a glyph the board lacks is added there first, then used. The board is written from an enumeration, so re-run it when the set changes.

**Library and stroke.** One library, `lucide`, at its default outline stroke. Never mix in filled glyphs or a second library; mixed stroke weights read as different products. The Journey, Brief and IA boards use a further 11 documentation glyphs (faces, heart, target, users and similar). They are not product icons and do not belong on the board.

**One glyph, one meaning.** `folder` is always a group, `type` a text item, `link` a URL item, `layers` several items at once (Paste All), `sparkles` AI-generated or AI-transformed content, `clipboard-list` the app mark in OS chrome. `circle-alert` means something failed (error states, test status); `triangle-alert` means act with care (a shortcut conflict, a destructive confirm). Do not swap one for the other.

**Sizes.** The slot that hosts an icon sets its size, not the glyph: `folder` is 12 in a Group Tag, 16 in a row, 18 in a header and 32 in a large empty state, and `sparkles` follows the same rule. `size-icon-xs` 12, `size-icon-sm` 14, `size-icon` 16, `size-icon-md` 18, `size-icon-lg` 24, `size-icon-xl` 32. 12 and 14 sit inside chips, rows and controls; 16 is the default beside body text; 18 leads headers and large rows; 24 and 32 belong to empty and confirm states. `width` and `height` cannot bind to variables in this Pencil version, so nodes carry the number and the token name is the source of truth.

**Colour.** `text` for an icon inside a labelled control (Secondary button), `text-2` for passive icons, `text-3` for decorative or disabled ones, `accent` for selected, active or AI, `danger` for destructive and error, `warning` for a degraded preview, `success` for a passed state (`circle-check`), `text-on-accent` on filled buttons. OS chrome uses `os-panel-text`. Never a literal hex, and never colour as the sole signal: every state icon sits beside text.

**Pairing.** An icon-only control carries an accessible label and a tooltip. Destructive controls are never icon-only (see **Do not do**).

**Drift status.** The off-scale sizes (11, 13, 15, 20, 28) were snapped to the scale: 11→12, 13→14, 15→16, 20→24 in the app icon and confirm badge, 28→32 in the search empty state. The browser-mockup lock in the flows was snapped to 12 as well. The one exception is the flow connector arrows (20), which are diagram chrome, not product UI. The search no-results screen uses the State Box component, and `image-off` and `info` now sit inside the `Note` component. Eleven icons still sit directly in screen layouts, each a single use (the Copy to Group `plus` row, a back chevron, the source-expander chevrons, an edit link, folder glyphs in name inputs); extract a component when one repeats a third time.

## Components

Reusable on the `04 Components` board. Every value is a token, so instances follow the active theme. Override text and icons on the instance; never detach.

| Component | Use |
| --- | --- |
| Key Cap | Shortcut legend (`⌘1`, `↵`). One cap per chord. |
| Divider | 1px `border` line. |
| Group Tag | Group name on a search result. |
| Chip | Neutral count or provider label. |
| Toggle On / Off | Binary setting. Off uses `text-3` track so it stays visible in dark. |
| Button Primary / Hover / Active | One primary action per view. Hover and active use `accent-hover` and `accent-active`. |
| Button Secondary, Danger | Secondary actions. Danger pairs red with a trash icon and the word "Delete". |
| Search Field | Search inside a panel or the main window header. |
| Row Default / Selected (Group Row) | Group or item row: icon, title, meta, optional Paste-all chip, key cap, chevron. Selected is `accent-soft` with the 2px accent bar. Turn optional parts on with `enabled`. |
| Nav Item / Nav Item Selected | Sidebar group row and Settings tab. Optional count. The "New group" row overrides icon and label to `accent`. |
| Panel Header / Panel Header Detail | Floating-panel header. Detail adds a back chevron and a count chip. |
| Footer Hint | One `Key Cap` plus its label. Every panel footer is a row of these. |
| Window Titlebar | macOS lights, Windows controls, Linux controls; `Title` is optional (off in the Main Window). |
| Segment Selected | Selected option of the theme control. |
| Choice Chip / Selected | Pick-one option (order, separator, prompt). |
| Select, Shortcut Field, Button Small / Small Danger, Binding Row | Settings controls: dropdown, hotkey field, Rename and Delete… buttons, one Vim/arrow key pair. |
| Panel Header Nav | Pick Item and Search headers: back chevron, title, meta; Query mode turns on the search icon, caret and clear. |
| Search Result Row / Selected | Result with the matched term in a `selection` chip, meta and Group Tag. |
| Item Row / Selected | Main Window item: type icon or thumbnail, title, meta. Group Row also takes a thumbnail for image items. |
| Settings Row | Label plus a Control slot (Toggle, Select, Stepper, Secret Field). Swap the slot with Replace. |
| Group Settings Row | Settings · Groups row: grip, name, Never send to AI toggle, Rename, Delete…. |
| Stepper, Segmented Control, Secret Field / Error, Test Status Row (idle, Testing, Failed, Succeeded) | Settings controls with their states. |
| OS Menu Bar, OS Top Panel, OS Taskbar | Per-platform desktop chrome; set `enabled` to `$is-macos`, `$is-linux` or `$is-windows` on the instance. |
| Section Heading | Title plus note above a group of settings. |
| State Box | Empty, error or action-required block: icon, title, message, optional Primary and Secondary action. States shown on the board: error, empty, no key. |
| Note | Inline notice on a panel: icon, one-line message, optional text action ("1 image skipped. Paste as file"). A plain footnote overrides `fill` to `surface`, padding to 0 and disables the action. |
| Menu Item / Selected / Danger | Row in an actions menu: 16px icon and one-line label. Selected is `accent-soft`; Danger pairs a `danger` trash icon with a `danger` label ("Delete group…"). |
| Detail Row | Key-value line in the detail pane: 90px `text-3` label, `text` value at weight 500 (Type, Source app, Copied, Group). |
| Overline Group | Overline label (`text-overline`, `text-3`) above a slot: chip rows, the Original box, the Preview box. Content goes in the slot; never rebuild the label. |
| Options Summary | `surface-alt` strip with a one-line summary and an `Edit` link, shown on the Paste All AI loading, error and no-key screens. |
| Field Head | Settings field heading: label left, caption hint right (Default prompt template). |
| Shortcut Row | Bordered row for a global shortcut: label, Shortcut Field, `Reset` link. |
| Captured Strip | Copy to Group header strip: CAPTURED overline, the captured text, and a type icon with source app. |
| Reformat Header | AI Reformat panel header: sparkles, title, and the `Anthropic` model chip. |
| Source Row | Numbered source line in the AI result sources list with a fate pill (`Point n` on `accent-soft`, `Dropped` on `surface-alt`). |
| Prompt Field | `surface-alt` read-only box for the default prompt template in Settings · AI. |
| Result Head | Overline with icon above a result: result, loading, error, no key. |
| AI Toggle Row (Off / On / Locked) | Reformat toggle with the "Sends N items" note. Locked replaces the toggle with an "Open Settings" link when the group is set to Never send to AI. |
| Confirm Dialog | Destructive confirmation: names what is lost, Cancel plus Danger button. |
| Toast Inverse (Undo / Paused / Info) | Inverse-surface toast under a window: message, optional action, detail (`6s`, `Paused`, `11 items removed`). |
| Notification Toast | The OS notification for a copy (success or failure). |

Retrofitted into screens: every component above. Repeated structures in a screen must be an instance; a raw frame is allowed only for one-off layout containers (a pane, a list, a body). Layout containers stay raw: a pane, a list, a card shell, a popover slot. Everything with content is an instance.

Binding limits: colours, type, spacing, radius, elevation and strokes are variables. `width` and `height` cannot bind to a variable in this Pencil version, so `size-*` values (`size-panel` 520, `size-toast` 340, `size-icon` 16, …) are documented numbers that components copy. Motion tokens (`duration-*`, `ease-standard`) are variables with no node property to bind; they are specified for the build.

### Platforms

A second theme axis, `os` (`macos` default, `windows`, `linux`), sits beside `mode`. Set `theme: {mode, os}` on a frame or instance and everything below follows it:

- Boolean variables `is-macos`, `is-windows`, `is-linux`, `has-topbar` drive `enabled` on platform chrome: traffic lights, Windows caption buttons, Linux window buttons, the macOS menu bar, the GNOME-style top panel and the Windows taskbar.
- Popovers sit under the top bar on macOS and Linux (Linux notifications are centred) and above the taskbar on Windows.
- Shortcut text cannot bind to a variable (text `content` does not resolve variables), so each label carries three text nodes, one per platform, toggled by `enabled`. Key Cap has `Label`, `Label Windows` and `Label Linux`; override all three on an instance.
- Defaults: macOS `⌘⌥C` / `⌘⌥V`; Windows `Win+Shift+C` / `Win+Alt+V`; Linux `Ctrl+Alt+C` / `Ctrl+Alt+V`. Windows avoids `Ctrl+Alt`, which is AltGr on many international keyboards. `Win+Shift+C` is unassigned on Windows 11 (checked against Microsoft's shortcut list); `Win+Shift+V` is taken (cycle notifications), so paste uses `Win+Alt+V`, which is absent from Microsoft's Windows shortcut list (the Win+Alt letters in use are B, D, H, K), so it is free at OS level; apps can still claim it. Both stay remappable. Windows quit is `Alt+F4`.
- One chord, one meaning: `⌘↵` is always "paste all". "Paste without AI" and "paste original" use `⌘⇧↵` (`Ctrl+Shift+↵`).

Flows are reusable boards, laid out as macOS, Windows, Linux across and Light above Dark, named `06 Flow · <name> · <OS> · <Mode>`.

Screens under `05 Screen ·` are `reusable`. Each has a `· Dark` instance with `theme: {mode: "dark"}`, and flow boards contain instances of these screens, so edits propagate everywhere.

## Interaction behaviour

Focus, recording and error states exist as components: Search Field Focus, Button Primary Focus, Row Selected Focus, Shortcut Recording, Shortcut Conflict. Every asynchronous action ships loading, error and missing-prerequisite states (AI: loading, no API key, error with retry and "Paste original"). Motion tokens exist on the canvas: `duration-fast` 100ms, `duration-base` 160ms, `duration-slow` 240ms, `ease-standard` `cubic-bezier(0.2, 0, 0, 1)`._

### Navigation keys (Vim mode)

Settings → Shortcuts → Navigation keys holds one toggle, **Vim key bindings**, **on by default**. Bindings are Ctrl-based because every panel type-to-searches, so bare `h j k l` must stay typeable. They apply on every screen (panels, Main Window, Settings). Arrow keys, Tab and Esc always work; turning Vim mode off removes only the Ctrl alternates. In text fields, Ctrl+J and Ctrl+K still move through results and nothing else is captured.

| Action | Vim | Always |
| --- | --- | --- |
| Next / previous item | `Ctrl+J` / `Ctrl+K` | `↓` / `↑` |
| Back to previous pane / open group or next pane | `Ctrl+H` / `Ctrl+L` | `←` / `→` |
| Half page down / up | `Ctrl+D` / `Ctrl+U` | `PgDn` / `PgUp` |
| First / last item | `Ctrl+G` / `Ctrl+Shift+G` | `Home` / `End` |
| Cancel or close | `Ctrl+[` | `Esc` |

On macOS the modifier is `⌃`, never `⌘`. Canvas: `05 Screen · Settings · Shortcuts` (on), `· Vim Off` (off, Vim keycaps at 40% opacity), plus the Recording and Conflict variants.

## Accessibility

- Selection is never tint alone: pair the tint with a 2px accent bar (3:1 or better) and the accent icon or text. Keyboard focus is a `border-focus` ring, separate from selection.
- Text is 12px minimum, except 11px uppercase overlines. Body text 4.5:1; UI and focus ring 3:1 (measured table under Colour system).
- Colour is never the only signal: errors carry an icon and a message, selection carries a bar, AI content carries the sparkles icon and a label.
- Every shortcut is remappable and shows in the UI; recording, conflict and reset states are designed.

## Do not do

- **Do not use icon-only destructive controls.** Delete and rename carry a label (labelled button or menu). Deleting an item is immediate with an Undo toast (about 6s). Deleting a group asks first and names the item count. *(critiques of Main Window and Settings)*
- **Do not signal selection with tint alone.** *(critiques of every list screen)*
- **Do not ship an AI action without loading, error and no-key states.** *(critiques of AI Reformat and Paste All)*
- **Do not give one shortcut two meanings across screens.** *(critique of Paste All AI Result)*
- **Do not default to `Ctrl+Alt+<letter>` on Windows.** It collides with AltGr. *(critique of Settings)*

Source critiques: `.claude/docs/critique-05-screen-*.md`.

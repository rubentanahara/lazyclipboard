# PRD Title: lazyclipboard

**Author:** Ruben Tanahara (owner), drafted with AI agents (product-owner, tech-lead, product-designer)
**Team:** Ruben Tanahara plus AI coding agents working in parallel git worktrees

| Role | Name |
|---|---|
| Product Manager | Ruben Tanahara |
| Engineering Lead/Team Lead | Ruben Tanahara |
| Designer | Ruben Tanahara (design system and 52 screens complete) |
| Approvers/Sign-Off | Ruben Tanahara |

**PM Epic:** https://github.com/users/rubentanahara/projects/13
**Status of PRD:** In Review

*The PRD refers to other documents by title: "Design system document", "AI provider research note", "Technical approach document". The screens and flows are on the design canvas, named `05 Screen · ...` and `06 Flow · ...`.*

---

## One Pager

### Overview

lazyclipboard is a keyboard-first clipboard organizer for macOS, Windows and Linux. It is open source (MIT OR Apache-2.0, public repository `rubentanahara/lazyclipboard`).

The system clipboard holds one thing. lazyclipboard lets you:

1. Copy a selection into a named group with a global shortcut.
2. Paste from any group through a floating panel opened by another global shortcut.
3. Optionally reformat one item with AI.
4. Use Paste All to join a group's items into one block, with order, separator and an optional AI summary.

Data stays on the device: SQLite plus image files, no accounts, no cloud, no server. AI is optional and uses the user's own key for Anthropic, OpenAI or Gemini, called directly from the app.

Panels open by shortcut, are read in under two seconds and close with Esc. The reference feel is Raycast or Spotlight, not a dashboard. Design is complete (52 screens, 5 flows, three OSes, light and dark). Implementation has not started.

### Problem

- **One slot.** Someone doing research or writing copies five things from five windows, then pastes them one by one, going back to each source window. The OS clipboard forgets everything except the last copy.
- **History tools are noisy.** Clipboard history tools log everything copied, including passwords, so the user digs for the one thing they meant to keep.
- **Cross-OS gap.** Keyboard-first tools of this kind are mostly single-platform. Someone with a Mac laptop and a Windows desktop has no tool that behaves the same on both.
- **AI trust.** Where a clipboard tool offers AI, users cannot tell what leaves the machine, to whom, or whether the original was overwritten.

The problem to solve: make collecting deliberate (you choose what to keep and where), retrieval instant, and AI use visible and optional.

### Objectives

1. **Fast primary path.** Copy into a group and paste from it using the keyboard only. Median shortcut-to-pasted-text is at most 2 s in the manual test (10 runs per OS) on macOS, Windows and Linux X11.
2. **Trustworthy AI and data handling.**
   - 100% of AI actions show "Sends N items to <provider>" before sending.
   - 0 cases of an AI result replacing an original without a user choice.
   - 0 network requests except user-triggered AI calls and the update check.
3. **Ship on three OSes and get adopted.**
   - All three OSes pass the per-OS smoke checklist for slices P1 to P3 at first public beta.
   - Within 90 days of the public beta, at least 300 release downloads and 100 GitHub stars (targets confirmed by the owner).

### Constraints

1. **Solo maintainer plus AI agents.** One issue owns one directory, one worktree, one branch, one pull request. Slices follow screen priority P1 to P4.
2. **OS limits.**
   - Linux Wayland cannot inject keystrokes, so paste degrades to "item placed on the clipboard, press Ctrl+V". X11 is fully supported.
   - Windows cannot paste into an app running as admin from a non-elevated app.
   - macOS needs the Accessibility permission for capture and paste.
   - Native behaviour (global shortcut, capture, paste) cannot be unit tested, so each needs a manual per-OS smoke checklist.
3. **Public repo, no secrets.** API keys live only in the OS keychain. Signing material lives only in GitHub Actions secrets.
4. **No server.** No accounts, no sync, no telemetry that leaves the device. Updates come from GitHub Releases through the Tauri updater.
5. **Signing cost.** macOS builds are signed and notarized from the first beta (paid Apple developer membership). Windows is unsigned in beta (SmartScreen warning). Linux ships as AppImage and deb.
6. **AI vendor drift.** Model IDs change often. The three default model IDs are not yet verified against a real key (AI provider research note).
7. **Three OSes from day one, validated in order** macOS, Windows, Linux. First slice is spike R0: one global shortcut, one floating panel and one paste working on all three OSes.

### Persona

| Persona | Description |
|---|---|
| **Key persona: Mara**, developer who also writes docs | Juggles snippets, links and prompts across Safari, VS Code, Slack and Notion. MacBook plus a Windows desktop, so needs the same shortcuts and behaviour on both. About 8 groups, about 12 items in the busiest. Sets up an AI key only some days, so AI must never nag when no key is set. Wants: capture without leaving the current app, paste in under 2 s, Paste All of a group into a doc or a prompt. |
| **Persona 2: Tomas**, researcher | Collects quotes, links and screenshots into one group per topic. Needs oldest-first Paste All to rebuild reading order, and an AI summary with sources ("12 items to 4 points"). Handles interview material, so needs "Never send to AI" and a clear statement of what leaves the device. Wants trust and provenance, not only speed. |
| **Persona 3: Jonas**, freelance writer and support engineer on Ubuntu (GNOME) | Reuses canned replies, boilerplate and links all day, on Linux (Wayland or X11). Mostly paste, rarely AI. Wants paste that works or says plainly why not, and a tray icon that appears. Represents the degraded-paste path. |

Evidence status: personas are working assumptions from the journey map and owner input. No user interviews exist. Only Mara has journey-map evidence.

### Use Cases

**Scenario 1: Collect and paste (Mara, macOS).** Mara reads a bug thread in Safari and selects an error message. She presses ⌘⌥C. The Copy to Group panel shows the captured text with its source app. She types "bug", presses ↵, and a toast says "Saved to Bug 412" with Undo. In VS Code she presses ⌘⌥V, opens "Bug 412", and presses ↵ on the highlighted item. The text lands at the cursor. No mouse used.

**Scenario 2: Combine with AI (Tomas).** Tomas has 12 quotes in "Lit review". He opens the paste panel, selects the group and presses ⌘↵. Paste All options appear: order, separator, AI summary on. The note reads "Sends 12 items to Anthropic". He confirms, sees a loading state, then a result labelled RESULT with the sparkles icon: 4 points, each with its sources, dropped items marked. He pastes the summary into Notion, or presses ⌘⇧↵ to paste the joined original without AI. His "Interviews" group is set to Never send to AI, so AI is locked there.

**Scenario 3: Paste on Wayland and clean up (Jonas, Linux).** Jonas presses Ctrl+Alt+V in a Wayland session and picks a canned reply. The footer already says "Copy, then press Ctrl+V". He presses ↵, the panel closes, and a notification confirms the item is on the clipboard. He presses Ctrl+V. Later, in the main window, he deletes an outdated reply. It disappears at once with an Undo toast of about 6 s. Deleting a whole group would have asked first and named the item count.

---

## PRD

### Features In

Slices: R0 is the spike. P1 is the primary path (copy and paste plus onboarding essentials). P2 is entry and retrieval (tray, search, main window). P3 is manage and settings. P4 is AI and edge states. Paste All without AI is P2, and the Paste All Options screen moves from P3 to P2.

| Group | Feature | Slice | Why |
|---|---|---|---|
| Spike | One global shortcut, one floating panel, one paste on macOS, Windows, Linux | R0 | Proves the riskiest OS behaviour before anything else is built. |
| Capture | Copy to group shortcut and panel (Captured Strip, group filter, inline New group) | P1 | The core action; without it there is nothing to paste. |
| Capture | Item types: plain text, rich text (HTML plus plain fallback), links, PNG up to 10 MB | P1 | Covers what developers, writers and researchers copy. |
| Capture | Capture feedback: toast naming the group with Undo; failure notification when nothing is selected | P1 | Removes doubt about whether the shortcut worked. |
| Capture | Duplicate handling: recapture moves an identical item to the top | P1 | Prevents clutter. |
| Paste | Floating paste panel: groups, items, preview, footer naming what ↵ does, Esc closes | P1 | The second half of the core loop. |
| Paste | ⇧↵ pastes plain text only; images paste as images | P1 | Rich text must be pasteable without foreign formatting. |
| Paste | Degraded paste on Wayland and elevated Windows apps | P1 | An honest fallback beats a silent failure. |
| Onboarding | Welcome, macOS Accessibility (needed, granted, revoked), Wayland notice, Shortcuts check | P1 | Without permission macOS cannot capture or paste. |
| Keys | Default shortcuts per OS, Esc, arrows, Ctrl-based Vim navigation | P1 | Keyboard-first is the product identity. |
| Search | Type-to-search across all groups, match highlight, Group Tag, useful no-results state | P2 | Finding the item is the common cost. |
| Paste All | Join a group: order (oldest or newest first), separator, plain text (the Options screen ships in P2 with Options AI Off) | P2 | Turns a group into one usable block. |
| Entry | Tray icon and menu | P2 | A tray-only app needs a visible home. |
| Main window | Sidebar, item list, detail pane, item actions, empty states | P2 | Browsing and tidying beyond the panels. |
| Manage | Delete item (immediate, Undo about 6 s); delete group (confirm with item count) | P3 | Cheap to undo, costly to lose. |
| Manage | Group rename, reorder, delete | P3 | Basic tidy-up. |
| Settings | Shortcut remap (recording, conflict, reset), Vim toggle, theme | P3 | One chord, one meaning; remappable everywhere. |
| Settings | Retention (200 per group, auto-delete off) | P3 | Nothing is deleted unless the user chooses. |
| Settings | Launch at login, local usage stats toggle, Export diagnostics | P3 | Local-only stats with user control. |
| Onboarding | Launch at login plus stats step | P3 | Asked once. |
| Lifecycle | Tray-only, single instance, hide on close, shortcuts with no window open | P1 | The capture and paste loop cannot run as a real app without them. |
| Lifecycle | Updater from GitHub Releases | P3 | Stays current; needs the release pipeline, which arrives late. |
| AI | Provider choice (Anthropic, OpenAI, Gemini), own key in the OS keychain, default model plus Advanced override, Test connection | P4 | Optional value with user control. |
| AI | Reformat one item with loading, error and no-key states; original never silently replaced; ⌘⇧↵ pastes original | P4 | AI trust rules. |
| AI | Paste All AI summary with "Sends N items to <provider>", sources list and dropped items | P4 | Makes AI output checkable. |
| AI | "Never send to AI" per group | P4 | Privacy control for sensitive groups. |
| AI | Onboarding AI key step (skippable) | P4 | The app is fully usable without it. |
| Accessibility | Selection never tint alone, focus ring, labelled controls, keyboard reach | P1 to P3 | Baseline from the design rules. |

### Features Out

| Feature | Reason |
|---|---|
| Cloud sync and accounts | Needs a server and a privacy model; contradicts local-only; solo-maintainer capacity. |
| Sharing between people | Depends on accounts and sync; a different product. |
| Mobile and web clients | Global shortcuts and floating panels are desktop concepts. |
| Editing item content or images | A clipboard organizer stores what was copied. |
| Automatic log of everything copied | Capture is explicit by design; a monitor records passwords and creates the noise this product avoids. |
| Rich-text or HTML join in Paste All | Plain-text join is enough for MVP. Post-MVP. |
| Streaming AI output | Loading then result is enough. |
| Wayland full-parity paste | Wayland forbids keystroke injection; degraded paste ships instead. |
| Telemetry that leaves the device | Local counters only. |
| An item in more than one group | One item lives in one group; keeps ordering, duplicates and Paste All simple. |
| Images larger than 10 MB | Skipped with a "1 image skipped" note. |
| Signed Windows builds in beta | Cost; revisit before 1.0. |
| Files and non-PNG images | Not in the decided item set; behaviour on unsupported copies is open. |
| Editing the AI prompt template | Settings shows the default prompt read-only. |

### Requirements

**User stories**

1. As Mara, I want to capture a selection into a named group with one shortcut, so that I do not lose my place in the app I am reading.
2. As Mara, I want to open a floating panel and paste from a group in two keystrokes, so that I stay on the keyboard.
3. As Mara, I want to type part of a word and see matching items across all groups highlighted, so that I find a snippet without remembering the group.
4. As Tomas, I want Paste All to join a group's items in the order and with the separator I choose, so that I rebuild my notes as one block.
5. As Tomas, I want an AI summary that says how many items go to which provider before sending, so that I stay in control of my data.
6. As Tomas, I want the summary to show which items became which point and which were dropped, so that I can check it against my sources.
7. As Tomas, I want to mark a group "Never send to AI", so that my interview notes cannot leave the device by accident.
8. As Mara, I want to delete an item at once with a short Undo, so that tidying is fast and mistakes are cheap.
9. As Mara, I want deleting a group to ask first and name how many items I would lose, so that I never delete a group by reflex.
10. As Jonas, I want to be told plainly when paste cannot happen directly, so that I know to press Ctrl+V myself.
11. As a first-time macOS user, I want to be told about the Accessibility permission and see when it is granted, so that I know why capture does not work yet.
12. As a privacy-conscious user, I want to turn local usage stats off and export them as a file, so that I decide what is recorded and what I share in a bug report.

**Functional requirements.** Priority: Critical blocks R0 or P1; High is required for its slice; Medium is in MVP and cut first. Non-functional requirements are in the section that follows.

| ID | Requirement | Acceptance criteria (Given/When/Then) | Priority |
|---|---|---|---|
| CAP-1 | MUST open the Copy to Group panel from the global capture shortcut with no app window open. On Linux Wayland the shortcut saves what is currently on the clipboard (the user copies first). Known Linux terminals use Ctrl+Shift+C. Items marked concealed by a password manager are refused with "Password manager item not saved". | Given the app runs in the tray and text is selected in another app, when the user presses the capture shortcut, then the panel opens showing a Captured Strip (text, type icon, source app) and a group list. | Critical, P1 |
| CAP-2 | MUST save the captured item into the chosen group at the top using only the keyboard. | Given the panel is open, when the user types to filter, moves with arrows and presses ↵, then the item is stored at the top of that group and the panel closes. | Critical, P1 |
| CAP-3 | MUST allow creating a group inline, including on first capture with zero groups. | Given no group matches (or none exists), when the user chooses New group, types a name and presses ↵, then the group is created and the item saved in it. | High, P1 |
| CAP-4 | MUST store text, rich text, links and PNG images. Text is capped at 1 MB plain and 2 MB HTML; any clipboard image is re-encoded to PNG; copied files and other content show "Unsupported content". An item is Rich text only when its sanitised HTML has real formatting, otherwise it is plain text. Rich text keeps the HTML flavour and a plain fallback; previews are plain text; rich rows show a neutral "Formatted" chip; URLs are detected by pattern and shown with the link icon. | Given a selection of each type, when captured, then the item shows the matching type icon; rich text shows the Formatted chip and a plain preview; a URL shows the link icon. | Critical, P1 |
| CAP-5 | MUST skip PNG images above 10 MB and say so. | Given the image is larger than 10 MB, when the user saves, then it is not stored and a Note shows "N image(s) skipped"; other content is still saved. | High, P1 |
| CAP-6 | MUST NOT create a duplicate of an identical item within a group. | Given an identical item exists in the chosen group, when the user captures it again, then it moves to the top and the item count does not change. | High, P1 |
| CAP-7 | MUST confirm every save naming the group and offering Undo for about 6 s. | Given an item was saved to "Bug 412", when the confirmation shows, then it names "Bug 412" and Undo; pressing Undo within about 6 s removes the item. (Undo is a button on the OS notification on macOS and Windows; none on Linux, where the user deletes the item in the Main Window.) | High, P1 |
| CAP-8 | MUST show a failure message when nothing is captured. | Given nothing is selected, when the user presses the capture shortcut, then a failure notification says nothing was selected and no item is stored. | High, P1 |
| PST-1 | MUST open the paste panel from the global paste shortcut, centred on the active display, showing at least five rows without scrolling. Esc MUST close it and return focus to the previous app. | Given any app has focus, when the user presses the paste shortcut, then the panel opens with groups listed; when the user presses Esc, then it closes and the previous app regains focus. | Critical, P1 |
| PST-2 | MUST list a group's items newest first with a plain-text preview; ↵ MUST paste the selected item into the previously focused app. | Given a group is open, when the user selects an item and presses ↵, then the panel closes and the text is inserted at the cursor of the previous app. | Critical, P1 |
| PST-3 | MUST show a Footer Hint naming what ↵ does in the current context. | Given an item is selected, when the panel shows, then the footer reads what ↵ does for this OS and item ("Paste", or "Copy, then press Ctrl+V" on Wayland). | High, P1 |
| PST-4 | MUST paste only the plain flavour of a rich text item on ⇧↵. | Given a rich text item is selected, when the user presses ⇧↵, then the inserted content has no formatting. | High, P1 |
| PST-5 | MUST paste an image item as an image. | Given an image item is selected and the target accepts images, when the user presses ↵, then the image is inserted. | High, P1 |
| PST-6 | On Linux Wayland, MUST place the item on the clipboard and say to press Ctrl+V. On X11, MUST paste directly. | Given a Wayland session, when the user presses ↵, then the item is on the clipboard, the panel closes and a notification says to press Ctrl+V. Given X11, the text is inserted directly. | Critical, P1 |
| PST-7 | SHOULD place the item on the clipboard and explain when the target app is elevated. | Given the previous app runs as admin and lazyclipboard does not, when the user presses ↵, then the item is on the clipboard and a message says paste cannot reach that app. | Medium, P1 |
| SRC-1 | MUST search all groups by typing in the paste panel and main window, highlight the matched term and show the Group Tag. Images are not matched by content. | Given the panel is open, when the user types "err", then results across all groups appear with "err" highlighted, each with its Group Tag. | High, P2 |
| SRC-2 | MUST show a useful no-results state. | Given no item matches, then a State Box shows the query and how to recover. | Medium, P2 |
| PAL-1 | ⌘↵ (Ctrl+↵ on Windows and Linux) MUST open Paste All options for the selected group with order, separator and the item count. Separators: New line, Blank line, Space, Comma, Bulleted list, Numbered list. The last-used order and separator are remembered globally. | Given a group is selected, when the user presses the chord, then options show the count, order choices and the six separators, preselecting the last used. | High, P2 |
| PAL-2 | MUST join the plain text of the items in the chosen order and separator; images left out with the count shown; disabled with a reason for an empty group. | Given 5 text items, oldest first and separator "new line", when the user confirms, then one block is inserted in that order; a group with 2 images shows "2 images skipped". | High, P2 |
| AI-1 | MUST support Anthropic, OpenAI and Gemini with one active provider chosen in Settings; key in the OS keychain; defaults `claude-haiku-4-5`, `gpt-5-nano`, `gemini-3.5-flash-lite`; an Advanced model field overrides the default. | Given a provider and key are saved, when Settings is reopened, then the key is not shown; an empty override uses the default; a model ID uses that ID. | High, P4 |
| AI-2 | MUST offer Test connection (idle, testing, failed, succeeded) without generating text. | Given a key is entered, when the user presses Test connection, then the Test Status Row shows Testing, then Succeeded or Failed naming the cause. | High, P4 |
| AI-3 | Prompts are four presets (Fix grammar, Shorten, Make formal, Summarise) plus Custom. An AI result is pasted then dropped, never saved as an item. MUST show "Sends N items to <provider>" before any send, then loading, then a result labelled RESULT with the sparkles icon; MUST NOT replace the original; ⌘⇧↵ (Ctrl+Shift+↵) pastes the original. | Given a key is set and the group allows AI, when the user starts a reformat, then the note names the count and provider, a loading state shows, the result appears beside the original, and ⌘⇧↵ pastes the original. | Critical, P4 |
| AI-4 | MUST show a no-key state and send nothing. | Given no key is set, when the user starts an AI action, then a State Box shows "No key" with an action opening Settings · AI and no request is made. | High, P4 |
| AI-5 | MUST show an error state per failure class with Paste original; Retry only for rate limit, overloaded and network. | Given the provider fails (invalid key, rate limit, quota exhausted, overloaded, blocked content, network), then an error State Box names the cause and offers Paste original; Retry is absent for invalid key and exhausted quota. | High, P4 |
| AI-6 | MUST lock AI for groups set to "Never send to AI", enforced in the Rust core as well as the UI. | Given the group has Never send to AI on, when the user opens reformat or Paste All options, then AI controls are locked with an "Open Settings" link and no request can start. | High, P4 |
| AI-7 | The Paste All AI summary MUST list each source item with its fate ("Point n" or "Dropped") and the source count. | Given 12 items and AI summary on, when the result shows, then Source Rows list all 12 with a fate pill. The model returns points with source item numbers as JSON; if the JSON cannot be parsed, the result shows without the Sources view and a note. | High, P4 |
| WIN-1 | MUST open the main window from the tray with sidebar, item list and detail pane; the detail pane collapses under about 900 px, then the sidebar to icons under about 700 px. | Given the main window is open, when the user selects a group, then items show newest first; when the window narrows below 900 px, the detail pane disappears. | High, P2 |
| WIN-2 | MUST show Type, Source app, Copied, Group and Format in the detail pane. | Given a rich text item is selected, then the pane lists Format: Rich text and a plain preview; an image shows a thumbnail. | Medium, P2 |
| WIN-3 | MUST offer item actions in a labelled menu and empty states for no groups, an empty group and no search results. | Given an item is selected, when the actions menu opens, then labelled actions show. Moving an item between groups is out of MVP. | Medium, P2 |
| SET-1 | MUST let the user remap every global shortcut with recording, conflict and reset; a conflicting chord MUST NOT save. | Given the user records a chord already in use, then Shortcut Conflict shows and the old chord stays; Reset restores the platform default. | High, P3 |
| SET-2 | MUST list groups with rename, reorder, Never send to AI and a labelled Delete…. | Given Settings · Groups, when the user renames a group, then the new name shows in panels; toggling Never send to AI applies AI-6. | High, P3 |
| SET-3 | MUST offer theme Light, Dark, System; System follows the OS live. | Given theme is System, when the OS switches, then the app switches without restart. | Medium, P3 |
| SET-4 | MUST expose Launch at login (default on) and Local usage stats (default on) in Settings · General. | Given a fresh install after onboarding, then both show as on and can be turned off. | High, P3 |
| TRY-1 | MUST show a tray icon with a menu to open the main window, open Settings and quit, with the shortcuts shown. | Given the app runs, when the tray menu opens, then those items are present and both shortcuts display for the current OS. | High, P2 |
| ONB-1 | MUST show onboarding once on first run in one window: Welcome, platform step, Shortcuts, Launch at login plus stats, AI key; each step carries a Chip naming it and a Footer Hint. | Given a first launch, then onboarding opens with Welcome; when finished or dismissed it does not reappear. | High, P1 |
| ONB-2 | On macOS MUST show the Accessibility step (needed, granted, revoked); on Wayland the Wayland Notice; on Windows no permission step. Only macOS permission may block use. | Given macOS without Accessibility, then the step shows "needed" with a way to open System Settings and moves to "granted"; if later revoked, the revoked screen appears when capture or paste is attempted. | Critical, P1 |
| ONB-3 | MUST ask Launch at login and local stats once; the AI key step MUST be skippable. | Given the user skips the AI key, then the app is fully usable and AI actions show the no-key state. | High, P3 |
| UND-1 | MUST delete an item immediately with an Undo toast for about 6 s that restores it in its position. | Given an item is selected, when the user deletes it, then it disappears and a toast with Undo shows; Undo within about 6 s restores it; after that the deletion is final. | Critical, P3 |
| UND-2 | MUST ask before deleting a group, naming the item count, with Delete pairing the danger colour, a trash icon and the word "Delete". | Given a group has 12 items, when the user chooses Delete group…, then a dialog says 12 items will be lost with Cancel and Delete; Cancel changes nothing. | High, P3 |
| KEY-1 | Every action MUST show its key cap; one chord has one meaning on every screen; defaults macOS ⌘⌥C and ⌘⌥V, Windows Win+Shift+C and Win+Alt+V, Linux Ctrl+Alt+C and Ctrl+Alt+V; Esc closes any panel. | Given any panel, then every available action shows its chord; on each OS the defaults match. | High, P1 |
| KEY-2 | Vim key bindings MUST default to on with Ctrl chords (Ctrl+J/K next/previous, Ctrl+H/L back/forward, Ctrl+D/U half page, Ctrl+G/Ctrl+Shift+G first/last, Ctrl+[ cancel); arrows, Tab and Esc always work; typing h, j, k, l in a search field types the letter. | Given Vim is on and a search field has focus, when the user presses Ctrl+J, then the selection moves and no character is typed; with Vim off, Ctrl+J does nothing and arrows still work. | High, P1 (panels), P3 (toggle) |
| A11Y-1 | Selection MUST use tint, a 2 px accent bar and accent text or icon; colour is never the only signal. | Given a selected row in Light or Dark, then it shows the bar and accent icon as well as the tint; an error shows an icon and text. | High, P1 |
| A11Y-2 | Every feature MUST be keyboard-reachable with a visible focus ring; icon-only controls carry an accessible label and tooltip; destructive controls are never icon-only. | Given the main window, when the user tabs through it, then every control shows a focus ring and no action needs the mouse. | High, P1 to P3 |
| A11Y-3 | SHOULD expose row selection and toast text to screen readers. | Given VoiceOver (macOS) or NVDA (Windows) is on, when a row is selected or a toast appears, then its text is announced. Orca is best effort. | Medium, P3 |
| RET-1 | Default retention MUST be 200 items per group with auto-delete off; nothing is deleted unless the user chooses. | Given a fresh install, then Settings shows 200 items per group and auto-delete off. | High, P3 |
| RET-2 | At the limit MUST block capture and never drop items silently. Auto-delete, when on, removes items older than N days. | Given a group holds 200 items, when the user captures into it, then nothing is saved and a message says "Group is full (200). Delete an item or raise the limit". | High, P3 |
| TEL-1 | MUST store local counts and durations only, in SQLite, for 90 days; a Settings toggle (turning it off stops recording and deletes existing counters); Export diagnostics saves a JSON file with no item content. | Given stats are on, when the user exports diagnostics, then a JSON file with counters only is saved; with stats off, no new counters are recorded. | Medium, P3 |
| TEL-2 | MUST NOT transmit usage data. | Given stats are on, then no request contains counters or item data; the only outgoing requests are AI calls the user starts and the update check. | High, P3 |
| LIF-1 | The app MUST run tray-only; closing a window hides it; shortcuts work with no window open; only one instance runs. | Given the main window is closed, when the user presses the capture shortcut, then capture still works; a second launch opens the existing window and starts no second process. | Critical, P1 |
| LIF-2 | MUST check GitHub Releases with the Tauri updater and tell the user an update exists before installing. | Given a newer release exists, then the app shows the version and installs only when the user confirms. The check runs automatically once a day and has a toggle in Settings. | Medium, P3 |

### Design

The design is complete and lives on the Pencil canvas (file `lazyclipboard.pen`, opened only through the Pencil tools) and in the Design system document. Every screen has a dark instance; every flow exists for macOS, Windows and Linux in light and dark. Build to the tokens in the Design system document.

| Area | Screens (`05 Screen · ...`) | Flow board (`06 Flow · <name> · <OS> · <Mode>`) |
|---|---|---|
| Foundations | `00 Brief`, `01 Journey`, `02 IA · Sitemap`, `02 IA · Screen Inventory`, `03 Tokens`, `04 Components` | none |
| Onboarding | Welcome, Permission, Permission Granted, Permission Revoked, Wayland Notice, Shortcuts, Launch at Login, AI Key | First run |
| Capture and tray | Copy to Group (Empty, New Group), Copy Notification (Error), Tray Menu (No Groups) | Copy to Group |
| Paste | Pick Group (Empty), Pick Item (Empty), Search (No Results), AI Reformat (Loading, No Key, Error) | Paste from Group |
| Paste All | Options (AI Off), AI Loading, AI Result (Sources), AI Error, AI No Key | Paste All from Group |
| Main Window | Main Window (Search, Empty Group, Actions Menu, Delete Group, Group Deleted, Item Deleted, Rename Group) | Manage Items and Groups |
| Settings | General, Shortcuts (Recording, Conflict, Vim Off), Groups (Delete Group), AI (Testing, Test Failed, Test Succeeded, Provider Menu) | Manage Items and Groups |

**Intended flows** (keys in brackets)

1. **Copy to Group.** Select content and press [capture chord]; the panel shows the Captured Strip and the group list; move with [↑/↓ or Ctrl+J/K] or type to filter; the `+ New group` row opens a name field; [↵] saves, closes the panel and shows the confirmation toast (or the error toast on failure); [Esc] saves nothing.
2. **Paste from Group.** Press [paste chord] to open Pick Group; type to filter or move; [↵, →, Ctrl+L] opens a group; Pick Item shows five rows without scrolling; typing switches to Search with the match highlighted and a Group Tag; [↵] pastes into the previous app; [⇧↵] pastes plain; [←, Ctrl+H] goes back; [Esc] closes. On Wayland [↵] places the item on the clipboard and shows the paste notice.
3. **Paste All.** From Pick Group or Pick Item press [⌘↵]; the 560 px panel shows order and separator Choice Chips; the AI Toggle Row is Off, On ("Sends N items") or Locked; skipped images show "N images skipped. Paste as file"; [↵] pastes the joined block, or with AI on shows loading then the result with sparkles and RESULT; [⌘⇧↵] pastes the original; AI Error offers retry and Paste original; AI No Key offers Open Settings.
4. **Manage.** Open the Main Window from the tray; sidebar (240), item list and detail pane (300); the Actions Menu lists labelled actions; item delete is immediate with a 6 s Undo toast; group delete asks first naming the count, then shows an info toast; Settings · Groups repeats rename, reorder, Never send to AI and Delete….
5. **First run.** Welcome; then macOS Permission (blocks until granted), Windows no step, Linux Wayland the Wayland Notice; Shortcuts; Launch at Login plus local stats; AI Key (skippable). Step chips name the step because counts differ per OS.

**Regressions to prevent**

1. AI never replaces the original silently; a result carries the sparkles icon and RESULT; the original is one chord away; "Sends N items" shows before any send.
2. The Undo toast lasts about 6 s and pauses on hover and focus; group delete has no Undo.
3. The Footer Hint names `↵` on every screen; never a bare `↵` cap.
4. The matched term is highlighted in a `selection` chip; the chip alone must not carry meaning.
5. Skipped images never disappear silently.
6. One chord, one meaning: ⌘↵ Paste All, ⌘⇧↵ paste original or without AI, ⇧↵ plain. A remap that duplicates a chord shows Conflict.
7. Delete and rename are never icon-only.
8. Never default to Ctrl+Alt+<letter> on Windows (AltGr).
9. Panels are fixed width (520, or 560 with AI or Paste All), centred, never resize with content, show at least five rows, close with Esc.
10. Selection is accent-soft, a 2 px bar and accent text; never tint alone, never a solid accent fill.

**Accessibility acceptance checks**

1. Body and metadata text are at least 4.5:1 against the darkest surface they sit on, in both themes. Re-measure dark `accent` on `accent-soft` (4.5) and dark `text-3` on `accent-soft` (4.6) if colours change.
2. Focus ring, selection bar, toggle-off track and control edges are at least 3:1 against the adjacent surface, measured on `surface`, `surface-alt`, `accent-soft` and `accent`.
3. A selected and focused row shows both the 2 px bar and a `border-focus` ring, visually distinct; focused-only shows the ring; selected-only shows the bar.
4. The focus ring on a primary button is visible against the `accent` fill.
5. No text is below 12 px except 11 px uppercase overlines (computed-style scan of every panel and window).
6. Every flow completes with the keyboard only. With Vim on, Ctrl+J/K/H/L/D/U/G and Ctrl+[ work on panels, Main Window and Settings; arrows, Tab and Esc work with Vim off.
7. In a search field with Vim on, only Ctrl+J and Ctrl+K are captured; typing h j k l inserts characters.
8. Every icon-only control has an accessible name and tooltip.
9. Colour is never the only signal: errors show an icon and text; AI content shows sparkles and RESULT; danger shows a trash icon or the word "Delete".
10. The Undo toast pauses on hover and focus, is announced to assistive tech, and is dismissible by keyboard.
11. Motion is 100, 160 and 240 ms and is removed or minimal under the OS reduced-motion setting.
12. After a panel closes, focus returns to the previous app; after a dialog or toast closes, focus returns to its trigger; the Confirm Dialog traps focus and Esc cancels.

**Pending design items** (found while reviewing the design against this PRD; none is drawn)

1. The Reformat Header hard-codes an "Anthropic" chip; provider is user-selected.
2. The Formatted chip is not on Search Result Row.
3. No copy exists for what local stats record, that they stay on the device, or what the export contains; done: copy added to Settings · General · Privacy.
4. No screens for Linux X11, a Windows elevated-target message, Wayland shortcut registration, or GNOME tray absence.
5. The match highlight is a tint on a tint (selected row on `accent-soft`); done: the matched term is underlined as a non-tint cue.
6. Vim Off keycaps use 40% opacity, which conflicts with the "alpha only for overlay and shadow" rule and has no measured contrast.
7. Toast contrast (`text-on-inverse` on `surface-inverse`, the Undo action colour) is not in the measured contrast table.
8. Paste All Options was P3 while Options AI Off is P2; done: Options is P2 in the inventory.
9. No Appearance or Theme screen is in the inventory; done: the inventory notes that Theme lives in Settings · General.
10. `Ctrl+[` is in the design rules but not in the inventory's key list.
11. macOS text-field Emacs bindings (Ctrl+K, Ctrl+H, Ctrl+D) overlap the Vim keys; only Ctrl+J and Ctrl+K are captured in fields.
12. Missing states: Main Window search no-results, collapsed Main Window layouts, Rename Group and New-group validation, a Locked AI state in the Reformat panel, disabled buttons and toggles, an onboarding completion screen.
13. The Design system document front matter still says "macOS clipboard organizer"; `caption` and `small` are identical; weight 700 is unused.
14. The first-run flow lane calls the step "Startup"; the screen is "Launch at Login".
15. Undo scope when items are deleted from the floating panel is not defined.
16. Screen-reader and high-contrast behaviour is not specified beyond labels, tooltips and reduced motion.

### Technical Considerations

The full engineering approach is in the Technical approach document. Summary:

- **Shape.** One Rust process (Tauri 2) plus the system webview. Crates: `core` (database, models, classification, retention, usage counters), `os` (shortcuts, capture, paste, panel, tray, keychain, per OS), `ai` (provider trait and three providers). Four windows: `panel` (created hidden at startup and reused), `main`, `settings`, `onboarding` (created on demand). Rust owns all persistent and cross-window state; webviews hold view state only and never touch the clipboard.
- **Capture and paste.** Capture simulates the OS copy chord, reads the clipboard and always restores the previous clipboard (scope guard, `panic = "unwind"`). Paste writes the item, simulates the paste chord, then restores. No background monitor.
- **Panel.** Non-activating: NSPanel on macOS, `WS_EX_NOACTIVATE` on Windows, utility hints on X11.
- **Data.** SQLite (`groups`, `items`, `settings` as one typed JSON row, `usage_daily` histogram buckets), images on disk, forward-only migrations with a backup.
- **Contracts for Sprint 0** so parallel worktrees can start: domain types, command and event signatures with stubs, error unions, schema plus seed fixture, `os` API with a fake clipboard, `AiProvider` trait and classifier, CSS tokens, window entries and capabilities, IPC mock harness, CI skeleton, smoke checklist template.
- **R0 spike pass criteria** (macOS, Windows, X11 each): the shortcut opens the panel from each target app with no chord character leaking (20/20); the panel receives keys; the target keeps focus (Strict on macOS; Acceptable with sign-off elsewhere); ↵ inserts at the caret (20/20); the clipboard is restored (20/20 text, 5/5 image); shortcut to panel ready p95 ≤ 150 ms; the panel shows over fullscreen apps; ⇧ held and non-QWERTY layouts work; macOS shows the permission state when Accessibility is off; the spike builds in the 3-OS CI. Wayland: the portal shortcut fires 20/20 and the sentinel lands on the clipboard. If the R0 criteria fail because of Tauri or a plugin, switch to Electron; if the OS is the cause, amend the interaction in an ADR.

**Non-functional requirements**

| ID | Requirement | Target | How measured |
|---|---|---|---|
| NFR-1 | Panel open, warm | Hotkey to panel painted and accepting keys: p95 ≤ 150 ms | `panel_open_ms` histogram, 50 opens per OS |
| NFR-2 | Startup | Process start to shortcuts registered and tray visible ≤ 2 s | Log timestamps on launch at login |
| NFR-3 | Capture latency | Modifiers released to panel showing content p95 ≤ 300 ms; hard timeout 500 ms | `capture_ms` histogram |
| NFR-4 | Paste latency | ↵ to paste chord sent p95 ≤ 150 ms; ↵ to text visible ≤ 300 ms | `paste_ms` histogram; screen-recording check |
| NFR-5 | Clipboard restore, crash safe | Restored (or deliberately skipped when someone else wrote) in 100% of sequences ending in success, error or panic; a hard kill loses at most one in-flight snapshot, never written to disk | Unit tests with a fake clipboard injecting failure and panic at every step; CI check that the release profile is not `abort`; smoke sentinel 20/20 |
| NFR-6 | Idle CPU | ≤ 0.1% average over 10 min with all windows hidden; no polling outside a capture window | Activity Monitor, Task Manager, `top` |
| NFR-7 | Idle memory | App plus webview processes ≤ 200 MB (revise after the R0 baseline) | Process monitors |
| NFR-8 | Memory stability | ≤ 20 MB growth after 500 panel open/close cycles | Scripted hotkey loop |
| NFR-9 | Installer size | `.dmg` ≤ 20 MB (≤ 40 MB universal), Windows ≤ 15 MB, `.deb` ≤ 15 MB, AppImage ≤ 120 MB (set from R0) | CI prints sizes and fails over budget |
| NFR-10 | DB performance and size | With 20 groups × 200 items: group list ≤ 10 ms p95, search ≤ 50 ms p95, DB ≤ 20 MB | Ignored perf test on the seed, per release |
| NFR-11 | Accessibility | WCAG 2.2 AA; zero serious or critical axe violations per screen; keyboard-only operation | Storybook a11y test per component and screen state, axe with Playwright plus keyboard-only test per window flow; manual screen-reader pass per OS |
| NFR-12 | Offline | Every non-AI feature works offline; AI fails with a network error within 10 s connect and 60 s total; a failed update check is silent | Smoke run with networking off; a connection monitor |
| NFR-13 | Data crash safety | A kill at any point leaves `PRAGMA integrity_check` ok and no orphan image after restart | Core crash-simulation test |
| NFR-14 | Single instance | A second launch never creates a second process, tray icon or hotkey registration | Smoke checklist |

**Security and privacy**

- **Captured HTML never reaches a webview.** Item payloads carry plain text plus a `has_rich_text` flag; previews are text nodes; HTML is sanitised at capture and only written back to the clipboard; strict CSP.
- **Least privilege.** Per-window Tauri capabilities; the webview has no clipboard permission; no command accepts arbitrary text to paste (paste takes an item id or an AI result id held in Rust).
- **Keys** live only in the OS keychain, one entry per provider, read per request, never returned to a webview, never logged; the Gemini key travels in a header.
- **Logs and diagnostics** follow an allowlist: no item text, group names, source apps, prompts, AI output or keys; the export schema is checked by a test.
- **Data at rest** is unencrypted in an owner-only app data directory; the README states where.
- **Supply chain and CI.** Lockfiles, `cargo deny`, `pnpm audit`, actions pinned by SHA, `pull_request` (never `pull_request_target`), read-only default permissions, release only on `v*` tags in the protected `release` environment with owner approval, signed and verified updates.
- **What leaves the device:** the AI request you start (prompt plus plain text to the one configured provider), Test connection, and the update check (reveals IP and version to GitHub). Nothing else.

**Risks** (platform risks first; the full table is in the Technical approach document)

| ID | Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|---|
| R1 | Windows: a `WS_EX_NOACTIVATE` window cannot take keyboard focus | High | High | Test first in R0; fallback: activate the panel and restore the target on hide (needs sign-off) |
| R2 | macOS NSPanel through a community plugin | Medium | High | Pin the commit; R0 checks; vendor it if unmaintained |
| R3 | Capture races: clipboard not written yet, held modifiers, non-US layouts | High | High | Wait for modifier release, poll the change counter, layout-aware key, per-OS constants tuned in R0 |
| R4 | Terminals: injected Ctrl+C sends SIGINT; Ctrl+V does not paste on Linux terminals | High on Linux | High | Open decision; R0 records behaviour |
| R5 | Wayland: portal shortcut support varies; no capture injection; tray hidden on stock GNOME | High | Medium | Onboarding notice; proposed CLI fallback bound in desktop settings |
| R6 | Windows elevated apps block input silently | Medium | Medium | Check elevation first and show a specific message |
| R7 | macOS Accessibility revoked or lost when the signature changes | High | High | Check before every injection; one stable dev certificate; never change bundle id or Team ID |
| R8 | Apple notarization outage or rejection blocks a release | Medium | Medium | Notarize a test tag in Sprint 0; API-key auth |
| R9 | Tauri plugin maturity (NSPanel, tauri-specta release candidates, WebKitGTK) | Medium | Medium | Every plugin sits behind an `os` function; pin versions; validate in R0 |
| R10 | Restore fidelity and timing | Medium | Medium | Tune `RESTORE_DELAY` per OS; skip restore if someone else wrote |
| R11 | Unsigned Windows beta flagged by SmartScreen and antivirus | Medium | Medium | Publish checksums; report false positives; sign later |
| R13 | Parallel worktrees collide on lockfiles, command registration, capabilities | High | Medium | Sprint 0 contracts, CI bindings-diff check, one owner for `src-tauri/src` |
| R14 | Updater private key lost or leaked | Low | High | Encrypted offline backup; protected environment |
| R17 | Default model IDs unverified | Medium | Low | One constant per provider; a real-key check in the provider issues |

**ADRs to write.** 0001 Tauri 2 with an Electron fallback; 0002 one window per surface, pre-created hidden panel; 0003 pnpm, Turborepo and Cargo workspace with `core`/`os`/`ai`; 0004 tauri-specta contract; 0005 capture by simulated copy with restore; 0006 paste by clipboard write, simulated chord and restore; 0007 non-activating panel per OS; 0008 Linux X11 full, Wayland degraded; 0009 SQLite plus image files; 0010 keys in the OS keychain; 0011 rich text as sanitised, never rendered HTML; 0012 direct provider HTTP from Rust; 0013 Gemini `generateContent`; 0014 local-only counters; 0015 updater and distribution; 0016 release secrets in a protected environment; 0017 test strategy; 0018 rollout; and proposed 0019 migrations with `user_version`, 0020 no IPC command injects caller-supplied text, 0021 settings as one typed JSON row.

### Success Metrics

Nothing leaves the device, so metrics come from manual tests, local counters in Export diagnostics files that users attach to issues, GitHub signals and dogfooding. All numeric targets were confirmed by the owner on 2026-09-30.

**Manual test targets** (ten runs each, median reported, per OS each release)

| Test | Target |
|---|---|
| Capture-to-paste: shortcut pressed, item chosen, text present in the target app | Median ≤ 2 s, worst run ≤ 4 s |
| Keyboard-only capture and paste | 0 mouse actions |
| Find a known item in a group of 12 in the paste panel | ≤ 2 s |
| P1 to P3 per-OS smoke checklist | 100% pass on macOS, Windows, Linux X11 before beta |
| Wayland degraded path | Item on the clipboard and notification shown, 10 of 10 |
| AI trust check, each of 3 providers, reformat and Paste All summary | "Sends N items to <provider>" shown in 100% of runs; 0 originals replaced |
| Provider defaults with a real key | 3 of 3 succeed on Test connection and one reformat |

**Local counters** (90 days): `captures`, `capture_failures` (≤ 10% of attempts), `pastes`, `panel_open_to_paste_ms` (median ≤ 3000 ms, p90 ≤ 6000 ms), `panel_cancel_rate` (≤ 30%), `search_no_result_rate` (≤ 25%), `undo_rate` (informational; above 20% suggests accidental capture), `ai_requests_by_outcome` (error rate ≤ 5% excluding invalid key during setup), `paste_degraded` (informational; drives Wayland investment).

**Dogfooding:** daily use for 14 days on macOS plus Windows during beta; at least 5 captures a day; 0 items lost without a user deletion (a data-loss bug blocks release); P1 and P2 bugs fixed before the public announcement.

**GitHub signals** (90 days after the public beta): at least 300 release downloads, 100 stars, 10 issues from other people (3 with a diagnostics file), and 0 open P1 bugs older than 14 days. More than 20% of issues about Wayland or Windows-admin limits triggers a review of the fallback UX.

### GTM Approach

**One-line pitch:** lazyclipboard is a keyboard-first clipboard organizer: copy into named groups, paste from a floating panel, and join a group into one block. It runs on macOS, Windows and Linux, stores everything locally, and uses AI only if you bring your own key.

**Pillars:** (1) keyboard first, with every key shown; (2) your clipboard, kept on purpose, with nothing recorded unless you press the shortcut; (3) local and open, with no account, no cloud, no telemetry sent, under MIT OR Apache-2.0; (4) AI you can see, which says what it sends and to whom, never overwrites your original, and can be locked per group.

**Launch order:** private beta on macOS (signed and notarized), then a Windows beta (unsigned, SmartScreen note), then a Linux beta (AppImage and deb), then a public announcement when P1 to P3 pass the smoke checklist on all three OSes (assumption).

**Channels** (free, maintainer-run): the GitHub repository and Releases; a "Show HN" post; Reddit communities for macOS apps and open source; a short screen recording; after the beta, a Homebrew cask and winget or Scoop.

**The README (and any site) must show:** a 10 to 15 second capture-then-paste recording; a shortcut table per OS; a plain privacy statement (what is stored, what leaves the device, that stats stay local); install steps per OS naming the unsigned Windows warning and the Wayland limit; the item types and the 10 MB image limit; current status (it says "design complete, implementation not started" today); the licence; and how to attach a diagnostics file to a bug report.

### Open Issues

The 28 questions from the first draft were answered by the owner on 2026-09-30 and are recorded in the Decision Log and the requirements above. What remains open:

| # | Issue | Resolved by |
|---|---|---|
| 1 | The three default models are unverified with a real key; Gemini's 429 body and the real `retry-after` behaviour on OpenAI and Gemini need a live check (AI provider research note). | Acceptance criteria on the three provider issues |
| 2 | Whether Windows and X11 meet the "Acceptable" focus behaviour, the tuned timing constants, and the real idle memory and installer sizes. | R0 spike results; NFR-7 and NFR-9 are revised from them |
| 3 | Linux Wayland support varies by compositor (the portal shortcut is documented for KDE; GNOME and wlroots need checking). | R0 Wayland checks W1 to W4 |
| 4 | Personas Tomas and Jonas rest on assumptions. | Beta issues and feedback |
| 5 | Design gaps left for the issue that builds each screen: Linux X11 and Windows elevated-target screens, disabled states, Rename and New-group validation, Main Window search no-results and collapsed layouts, a Locked AI state in the Reformat panel, an onboarding completion screen, Vim Off keycap contrast, toast contrast, dynamic provider chip, and the Formatted chip on Search Result Row. | The owning screen issue |
| 6 | Windows code signing after beta. | Before 1.0; apply to a free open-source signing service (verify eligibility) |

**Assumptions committed to** (a wrong one should be caught at review): recapturing an identical item in a different group creates a separate item; search matches plain and link text, not image content; Paste All leaves images out and shows the count; deleting a group has no Undo after confirmation; the interface is English only for the MVP; elevated-app paste on Windows falls back to the clipboard message; the Copy notification shows the group name and item type, not the content.

### Q&A

| Asked by | Question | Answer |
|---|---|---|
| Linux user | Will paste work on Wayland? | Partly. Paste degrades to "item placed on the clipboard, press Ctrl+V". X11 works fully. On Wayland the capture shortcut saves the clipboard the user already copied. |
| Windows user | Why does the installer warn me? | Beta builds are unsigned, so SmartScreen warns. Signing after beta is Open Issue 6. |
| Privacy-conscious user | Does anything leave my machine? | Only the AI request you start (to the provider you picked, with your key), Test connection, and the update check. Stats stay local. |
| Mara | What happens at 200 items in a group? | Capture is blocked with "Group is full (200)"; nothing is deleted without her choosing. |
| Tomas | Does the summary tell me which item became which point? | Yes: every source shows "Point n" or "Dropped". The model returns the mapping as JSON; if it cannot be parsed the result shows without Sources. |
| Contributor | Can I test paste and capture automatically? | No. Native behaviour has no unit tests; each issue adds steps to the per-OS smoke checklist, run by hand. |
| Contributor | How do I know which files an issue may touch? | One issue owns one directory; see the ownership table in the agent guide. |

### Feature Timeline and Phasing

Sprints run Monday to Sunday, one week each except Sprint 0. Capacity is 3 parallel worktrees and 45 points per sprint (XS 1, S 2, M 3, L 5), recalibrated after Sprint 1. R0 is a hard gate: no P1 issue starts until R0 passes on all three OSes. Sprint 0 may extend by at most one week, after which the owner decides the Electron switch; an extension shifts every later date by seven days.

| Feature | Status | Dates |
|---|---|---|
| Sprint 0: scaffolding, shared contracts C1 to C11, CI, tokens, components, Storybook and e2e harnesses, R0 spike (macOS, Windows, Linux); R0 timeboxed to week 1, week 2 for fixes and the gate review | Backlog | 2026-10-05 to 2026-10-18 |
| P1: capture, paste panel, onboarding essentials, default shortcuts, lifecycle core (Sprints 1 and 2) | Backlog | 2026-10-19 to 2026-11-01 |
| P2: tray, search, Paste All, main window (Sprints 3 and 4) | Backlog | 2026-11-02 to 2026-11-15 |
| P3: manage, settings, retention, stats, updater (Sprints 5 and 6); release pipeline and macOS notarization start in Sprint 5 | Backlog | 2026-11-16 to 2026-11-29 |
| P4: AI providers, reformat, Paste All summary, Never send to AI (Sprints 7 and 8) | Backlog | 2026-11-30 to 2026-12-13 |
| Beta hardening (Sprint 9): release, README, bug fixes; beta build ready 2026-12-20 | Backlog | 2026-12-14 to 2026-12-20 |
| Public beta and announcement (all three OSes validated, no open `severity:blocker` issue) | Backlog | 2027-01-11 |

Every sprint from Sprint 1 carries two validation stories, Windows smoke and Linux smoke, size S, run on VMs. macOS smoke runs per pull request. Bugs found become issues in the next sprint, labelled `type:bug` and `severity:blocker`, `severity:major` or `severity:minor`. Only `severity:blocker` gates the beta.

### Decision Log

Append only. Date, decision, why, alternatives rejected.

| Date | Decision | Why | Alternatives rejected |
|---|---|---|---|
| 2026-09-30 | Open source, public repo, `MIT OR Apache-2.0`; secrets only in GitHub Actions, private vulnerability reporting on | The owner wants an open project; nothing sensitive is committed | Private repo; a single licence |
| 2026-09-30 | Tauri 2 (Rust core, system webview); Electron only if R0 fails for a Tauri cause | A Rust core reaches OS APIs directly; the system webview keeps a tray-resident utility small | Electron first; native UI per OS (three UI codebases for a solo maintainer) |
| 2026-09-30 | UI: Vite, React, TypeScript, Tailwind 4, Radix, Zustand; one webview window per surface | Accessible primitives, token-driven styling, fast cold start | Heavier frameworks; one window with routes |
| 2026-09-30 | pnpm, Turborepo and Cargo workspace; `core`/`os`/`ai` crates; one issue owns one directory | Disjoint directories let agents work in parallel worktrees | Single crate and package |
| 2026-09-30 | tauri-specta generated types | One source of truth for the Rust-to-TS boundary | Hand-written types |
| 2026-09-30 | Capture by simulated copy with clipboard restore; paste by clipboard write, simulated paste and restore; no background monitor | Captures only on explicit intent; nothing is logged in the background | Clipboard monitor; synthetic typing |
| 2026-09-30 | Non-activating panel; width 520/560, centred on the active display | The target app keeps focus and caret | A normal activating window |
| 2026-09-30 | X11 full; Wayland via portal with paste degraded to manual Ctrl+V; Windows elevated apps documented | Wayland forbids injection; UIPI blocks input into elevated apps | X11 only; `uinput`; running elevated |
| 2026-09-30 | Onboarding with macOS Accessibility (needed, granted, revoked), Wayland notice, shortcuts, launch at login, optional AI key | Required to post key events; asked once | None viable |
| 2026-09-30 | Items: text, rich text (sanitised HTML, never rendered, `⇧↵` pastes plain), links (regex), PNG ≤ 10 MB; one group per item; newest first; dedupe within a group; 200 per group; auto-delete off | Faithful paste with no HTML execution risk | Rendering HTML; plain text only; items in several groups |
| 2026-09-30 | SQLite, images on disk, no DB encryption in MVP; keys only in the OS keychain, one entry per provider | Simple, local, crash-safe; OS-grade secret storage | JSON files; encrypted DB; encrypted key file |
| 2026-09-30 | AI: Anthropic, OpenAI, Gemini with the user's key; fixed default model per provider with an Advanced override; direct calls, no SDK, no proxy, no streaming; Gemini `generateContent`, not Interactions | One endpoint per vendor; no backend cost; Interactions stores data by default | Vendor SDKs; a relay server; streaming; Interactions API |
| 2026-09-30 | Defaults `claude-haiku-4-5`, `gpt-5-nano`, `gemini-3.5-flash-lite` (not yet verified with a real key) | Fastest and cheapest per vendor docs | Larger models |
| 2026-09-30 | Local-only usage counters in SQLite, 90 days, toggle, Export diagnostics; nothing transmitted | Privacy and trust in an open-source tool | Remote telemetry; crash service |
| 2026-09-30 | Tray-only, single instance, hide on close, launch at login default on | A utility that stays out of the way | Dock app |
| 2026-09-30 | Tauri updater via GitHub Releases; signed and notarized macOS from first beta; unsigned Windows beta; AppImage and deb | No server cost; Gatekeeper requires notarization | App stores; a self-hosted update server |
| 2026-09-30 | Release secrets only in a protected `release` environment; fork PRs get none; actions pinned by SHA; CODEOWNERS on `.github` | A public repo worked on by many agents | Signing locally; unpinned actions |
| 2026-09-30 | Tests: Rust unit tests with recorded fixtures, Vitest, Playwright on the web build, 3-OS CI, manual native smoke checklist | Native flows cannot be automated cheaply | OS-level E2E automation |
| 2026-09-30 | Wayland capture saves the clipboard the user already copied; known Linux terminals use Ctrl+Shift+C/V | Wayland forbids injecting the copy chord; Ctrl+C in a terminal sends SIGINT | Unsupported on Wayland; injecting into terminals |
| 2026-09-30 | Windows and X11 may meet "Acceptable" focus (target regains focus with the caret unchanged within 100 ms); macOS must be strict | The Windows no-activate panel may not accept keys | Requiring strict focus everywhere |
| 2026-09-30 | Bundle identifier `io.github.rubentanahara.lazyclipboard` | Permanent; permissions, keychain and updater key off it | An owned domain |
| 2026-09-30 | macOS 13+ universal; Windows 10+ x64; Linux Ubuntu 22.04+ (WebKitGTK 4.1) | Covers current users without extra CI cost | arm64-only or older OSes |
| 2026-09-30 | Retention cap blocks capture at 200 ("Group is full"); auto-delete means items older than N days | Nothing is lost silently | Dropping the oldest; a soft limit |
| 2026-09-30 | One Sprint 0 contracts issue owns `src-tauri/src`, shared UI code and capability files | Prevents merge collisions in parallel worktrees | Per-issue edits to shared files |
| 2026-09-30 | Update check automatic once a day, toggleable, install on confirmation | The only non-AI network call; the user stays in control | Silent updates; manual only |
| 2026-09-30 | Concealed password-manager items are refused; restore covers text, HTML and images; text 1 MB plain and 2 MB HTML; images re-encoded to PNG; unsupported content fails visibly | Privacy and predictable behaviour | Capturing everything |
| 2026-09-30 | Rich text only when sanitised HTML has real formatting; RTF-only sources out of MVP; AI results pasted then dropped | Avoids "Formatted" on every browser copy | Any HTML counts as rich |
| 2026-09-30 | Paste All order and separator remembered globally; six separators; four AI presets plus Custom; attribution as model-returned JSON with a fallback | Predictable and checkable | Per-group memory; app-computed attribution |
| 2026-09-30 | Capture Undo is a notification button on macOS and Windows, none on Linux; no item moves, no "Paste as file", no pause-shortcuts feature in MVP (the Undo toast still pauses its timer on hover) | Linux notifications cannot carry actions; scope | An in-app toast window |
| 2026-09-30 | Relaunch opens the Main Window when the tray is missing (GNOME); Main Window shows in the Dock while open; native window frames | No hard extension dependency; less custom chrome | Requiring an extension; custom titlebars |
| 2026-09-30 | Panel centres on the display under the cursor; ⌘1 to ⌘5 map to sidebar position; turning stats off deletes counters | Predictable; privacy-first | Focused-window display; keep counters |
| 2026-09-30 | Screen readers in scope: VoiceOver and NVDA; Orca best effort; numeric targets confirmed; announcement waits for all three OSes; Windows unsigned through beta; R0 timebox one week, owner decides the Electron switch | Sets the accessibility bar and launch gates | Wider scope |
| 2026-09-30 | Rollout: all three OSes built from day one; R0 first; P1 to P4; validated macOS, then Windows, then Linux | Retire platform risk before building screens | macOS only first |
| 2026-09-30 | Backlog is one nested JSON file (epics, stories, tasks) that maps to GitHub Project 13; slugs link nodes, issue numbers are resolved on creation | Reads as the hierarchy; the creation script flattens it | A flat list with parent slugs |
| 2026-09-30 | Fifteen capability epics (E0 to E14); a story is one user-visible behaviour with Given/When/Then; a task owns exactly one directory; the slice lives in the Priority field | Disjoint directories let tasks run in parallel worktrees | One epic per slice |
| 2026-09-30 | Project fields: Priority (P0 to P4), Size (XS to L, larger must split), Discipline, Sprint (1-week iteration), plus Status options In Review and Blocked; labels `area:*`, `prio:*`, `type:*`, `native`, `severity:*` | One agent, one PR per task; blockers are visible | Labels only; a platform label |
| 2026-09-30 | Sprints are one week, Monday start; Sprint 0 is two weeks (2026-10-05 to 2026-10-18); 3 worktrees, 45 points per sprint, recalibrated after Sprint 1 | R0 needs a week of results and a week for fixes and the gate | One-week Sprint 0 |
| 2026-09-30 | R0 is a hard gate for all P1 issues; the owner decides the Electron switch at the end of Sprint 0; an extension is at most one week; tokens and components are built in Sprint 0 | Avoids building on an unproven platform; keeps worktrees busy during R0 | A soft gate with UI work against mocks |
| 2026-09-30 | Lifecycle core (tray-only, single instance, hide on close, shortcuts with no window) is P1; the updater stays P3 | The core loop cannot run as a real app without them | Whole Lifecycle row P3; updater in P1 |
| 2026-09-30 | Validation: macOS smoke per pull request; Windows smoke and Linux smoke stories every sprint on VMs; native checklist steps live in each issue | Per-task three-OS runs serialise everything through the owner | Per task on three OSes; per slice only |
| 2026-09-30 | Beta includes P4; a Release and beta epic with pipeline and notarization from Sprint 5 and Beta hardening in Sprint 9; beta build 2026-12-20, announcement 2027-01-11; only `severity:blocker` gates the beta | Apple setup has lead time; the announcement promises bring-your-own-key AI | Beta without AI |
| 2026-09-30 | An issue owns a directory or a module path inside it (`crates/os/src/<os>/<concern>`, `crates/core/src/<module>`, `windows/panel/<mode>`); native tasks are one per OS per concern; the window entry file and mode router belong to contract C8 | Whole-directory ownership would make every native and panel task collide | One issue per whole directory |
| 2026-09-30 | Storybook 10 (`@storybook/react-vite`) in its own app `apps/storybook`; stories sit beside the component or window code and are owned by that task; every design state is a story; `@storybook/addon-a11y` and `@storybook/addon-vitest` run the stories as tests on the Linux runner (a11y `test: 'error'`); the C9 IPC mock and the token CSS load in the preview; Turborepo tasks `storybook` (persistent, uncached), `build:storybook` (outputs `storybook-static/**`) and `test:storybook`; built in CI, not published in the MVP | Design states (loading, error, no key, locked) are hard to reach in a running app; isolated, checkable states catch accessibility and interaction defects before the window e2e specs; Turborepo documents this layout | Stories inside `packages/components` only; publishing the static site now; the Storybook test-runner |
| 2026-09-30 | With Storybook, the axe scan per component and screen state moves to Storybook; Playwright specs keep keyboard-only flows per window and one axe scan per window flow | Avoids scanning the same state twice | Both tools scan every state |
| 2026-09-30 | UI e2e: Playwright on the Vite web build with the C9 IPC mock, one project per window, `@axe-core/playwright` scans in the same specs (fail on serious and critical); Chromium on one Linux runner per pull request; specs at `apps/ui/e2e/<window>/<story>.spec.ts` owned by the story's UI task; one harness task in Sprint 0; real-app WebDriver deferred until after beta | The risky behaviour is native and stays on the smoke checklist; `tauri-driver` has no macOS support | Real-app WebDriver e2e now; standalone axe CLI |
| 2026-09-30 | Tests live inside each task; only the per-sprint validation stories are test-only; flow boards exist for 5 flows (Copy to Group, Paste from Group, Paste All, Manage Items and Groups, First run), each in macOS, Windows and Linux Light variants (15 boards) | Keeps tasks shippable in one pull request | Separate test tasks |
| 2026-09-30 | Backlog JSON at `docs/backlog/sprints.json`: 15 epics, 40 stories, 130 tasks; Linux P1 native work is scheduled in Sprint 3 (macOS then Windows then Linux); Linux smoke stories start in Sprint 3, Windows smoke in Sprint 1; R0 spike code lives in `spikes/r0-*`; each slice ends with a contracts maintenance task that applies change requests to shared files | Keeps every sprint within 45 points, follows the validation order, and respects the contracts ownership rule | All OSes in one sprint; per-issue edits to shared files |
| 2026-09-30 | Design references: light screens (about 52) plus the 5 flow boards exported to `docs/images/design/`; a story gets its screens, an epic gets its flow board; the export commit lands before issue creation | Raw GitHub links work only once the images are on `main` | All light and dark; names only |

### Change Log

Newest first. The body always reflects the current decision only.

| Date | Change | Why |
|---|---|---|
| 2026-09-30 | Backlog JSON drafted and 67 design images exported (light screens plus flow boards, compressed to about 4.5 MB); Linux P1 scheduled in Sprint 3 | Sprint grilling output |
| 2026-09-30 | Storybook added: harness task in Sprint 0, stories for every design state in the component and UI tasks, Storybook a11y tests in CI; Definition of Done, contracts C9 and C10, and the Sprint 0 goal updated | Owner request |
| 2026-09-30 | Sprint grilling rounds 4 and 5: module-path ownership, decomposition shape (one task per OS per concern), UI e2e stack and gates, tests inside tasks | Owner review |
| 2026-09-30 | Sprint grilling rounds 1 to 3: real sprint dates, R0 hard gate, Lifecycle row split (core P1, updater P3), validation cadence, Release and beta epic, capacity and board rules; Open Issue 7 closed | Owner review |
| 2026-09-30 | All 28 open issues answered; requirements, Decision Log and Open Issues updated; Paste All Options moved to P2 | Owner review |
| 2026-09-30 | First full draft merged from the product-owner, tech-lead and product-designer passes; the Technical approach document added | Sign-off preparation |

---

## PRD Checklist

| # | Topic | Done |
|---|---|---|
| 1 | Title | Done |
| 2 | Author | Done |
| 3 | Decision Log | Done |
| 4 | Change Log | Done |
| 5 | Overview | Done |
| 6 | Success Metrics | Done (targets confirmed) |
| 7 | Messaging (GTM Approach) | Done |
| 8 | Timeline/Release Planning | In progress (dates come from sprint grilling) |
| 9 | Personas | Done (assumptions, no interviews) |
| 10 | User Scenarios | Done |
| 11 | User Stories/Features/Requirements | Done |
| 12 | Features In | Done |
| 13 | Features Out | Done |
| 14 | Design | Done (16 pending items listed) |
| 15 | Open Issues | Done (7 remain, all owned) |
| 16 | Q&A | Done |
| 17 | Other Considerations | Done (Technical Considerations, NFRs, Security, Risks, ADRs) |

---

*Source structure: Product School, "Product Requirements Document (PRD) Template", extended with Requirements and Decision Log.*

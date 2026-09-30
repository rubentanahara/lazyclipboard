# Critique: 05 Screen · Settings

**Screen:** preferences window (760px wide, about 1,250px tall). Job: set the theme, remap shortcuts, manage groups, set retention, configure AI. Seen: Appearance (Light, Dark, System), Keyboard shortcuts, Groups (reorderable, rename, delete), Content retention (max items, auto-delete), AI formatting (toggle, provider, masked key, prompt template).

## Walkthrough

Sections are named in plain language, each has a one-line explanation, and controls sit in bordered rows with labels left and values right, which is the correct desktop settings idiom. Settings that can be wrong (the API key) is masked with an eye toggle. Appearance is first, which is the setting users most often want.

**Critical (UX):** the window is about 1,250px tall in a single scroll. On a 900px laptop screen the AI section, the one that needs the most configuration, is below the fold and the window would need to scroll. Split into a sidebar or tab bar (General, Shortcuts, Groups, AI); each pane then fits without scrolling.

**Important (UX):** shortcut fields say "Click a field and press a new combination" but show no recording state, no conflict warning, and no reset. Design the three states: recording ("Press keys…"), conflict ("Used by Spotlight, replace?") and default with a "Reset" link. On Windows and Linux, the default `Ctrl+Alt+C` collides with AltGr on many international keyboards, so typing "c" with AltGr triggers capture. Recommend a different Windows default (for example `Win+Shift+C`) and verify.

**Important (UX):** the AI section has no "Test connection", so a wrong API key is discovered only at paste time. Add a Test button with success and failure feedback. Also show which prompts are built in.

**Important (UX):** nothing shows whether changes are saved. State that changes apply immediately, or add a Save bar.

**Important (UI, accessibility):** the theme control's selected segment relies on a white raised chip on a light grey track with a 1px border, and dark-mode instances still show "Light" selected. Bind the selection to the current mode.

**Nit (UI):** group delete icons in red are 15px with no label; destructive icon-only again. Use the same menu pattern as the main window.

**Nit (UI):** the section headings are 15px semibold and the row labels 13px medium; the jump is fine, but the overline labels inside sections do not appear here, so the vocabulary differs from the panels.

## Start here
1. Split into tabs so each pane fits one screen.
2. Shortcut recording, conflict and reset states, and a safer Windows default.
3. Test connection for the API key.

# AGENTS.md

lazyclipboard is a keyboard-first clipboard organizer for macOS, Windows and Linux. Copy into a named group with a global shortcut, paste from a group through a floating panel, join a group with Paste All, optional AI reformat and summary with the user's own key.

## Sources of truth

- `DESIGN.md`: tokens, components, interaction rules, do-not-do list.
- `lazyclipboard.pen`: screens, flows, components. Open only through the Pencil MCP tools. Never `Read` or `Grep` it.
- `docs/PRD.md`: requirements and decision log (once written).

Use the names in `DESIGN.md`: Group, Item, Copy to group, Paste from group, Paste all.

## Stack

Tauri 2 with a Rust core. Vite, React, TypeScript, Tailwind 4, Radix, Zustand. SQLite for data, OS keychain for keys. pnpm workspace, Turborepo, Cargo workspace. AI providers: Anthropic, OpenAI, Gemini, called from Rust with the user's key.

## Layout and ownership

One issue owns one directory. Do not edit outside it; if you need a change elsewhere, open or comment on the owning issue.

| Path | Owns |
| --- | --- |
| `src-tauri/crates/core` | database, models, retention, telemetry counters |
| `src-tauri/crates/os` | shortcuts, capture, paste, panel windows, tray, keychain, per OS |
| `src-tauri/crates/ai` | provider trait and the three provider implementations |
| `apps/ui/src/windows/<name>` | one entry per window: panel, main, settings, onboarding |
| `packages/tokens` | CSS variables generated from `DESIGN.md` |
| `packages/components` | the component library from `04 Components` |
| `src-tauri/src`, `apps/ui/src/shared`, capability files | The Sprint 0 contracts issue only; later changes go through a comment on that issue |
| `docs` | PRD and ADRs |
| `.github` | workflows and CODEOWNERS; owner review required |

The Rust to TypeScript boundary is generated types (`tauri-specta`). Change the Rust command, regenerate, never hand-edit generated files.

## Rules

- One issue, one worktree, one branch, one pull request.
- Every value in the UI is a token. No literal colours, no off-grid spacing, text is 12px or larger except uppercase overlines.
- Every async action ships loading, error and missing-prerequisite states.
- Selection is never tint alone. Destructive controls are never icon-only.
- No secrets in the repository, ever. Keys live in the OS keychain; signing material lives in GitHub Actions secrets.
- Never render captured HTML in a webview. Previews are plain text.
- Native behaviour (global shortcut, capture, paste) cannot be unit tested; add the steps to the per-OS smoke checklist in the issue instead.
- Code carries no comments; names say what the code does.

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

One issue owns one directory, or one module path inside it named in the issue (`src-tauri/crates/os/src/<os>/<concern>`, `src-tauri/crates/core/src/<module>`, `apps/ui/src/windows/panel/<mode>`). Two open issues never share a path. Do not edit outside it; if you need a change elsewhere, open or comment on the owning issue.

| Path | Owns |
| --- | --- |
| `src-tauri/crates/core` | database, models, retention, telemetry counters |
| `src-tauri/crates/os` | shortcuts, capture, paste, panel windows, tray, keychain, per OS |
| `src-tauri/crates/ai` | provider trait and the three provider implementations |
| `apps/ui/src/windows/<name>` | one entry per window: panel, main, settings, onboarding |
| `apps/storybook` | Storybook config, preview, Vitest project; stories are owned by the component or window task that they sit beside |
| `apps/ui/e2e` | Playwright config and fixtures; each spec `<window>/<story>.spec.ts` is owned by the story's UI issue |
| `spikes/r0-*` | Sprint 0 R0 spike code per OS; deleted after the gate, proven code moves into the crates |
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
- Every UI change ships a story for each design state (default, loading, error, empty, locked) and the Storybook a11y test passes.
- Every UI story ships a Playwright spec for its keyboard flow, and the axe scan in it is clean.
- Selection is never tint alone. Destructive controls are never icon-only.
- No secrets in the repository, ever. Keys live in the OS keychain; signing material lives in GitHub Actions secrets.
- Never render captured HTML in a webview. Previews are plain text.
- Native behaviour (global shortcut, capture, paste) cannot be unit tested; add the steps to the per-OS smoke checklist in the issue instead.
- Code carries no comments; names say what the code does.

## GitHub

Values the `gh-*` skills read. Keep in sync with the repository and project settings.

- Repository: rubentanahara/lazyclipboard
- Project owner: rubentanahara
- Project number: 13
- Project node id: PVT_kwHOBFNXic4BlL6g
- Default branch: main
- Integration branch: dev
- Pull request base: dev
- Required check: check-pull-request-body
- Status options: Todo, In Progress, In Review, Blocked, Done
- Project fields: Status, Priority, Size, Discipline, Sprint
- Token secret: PROJECT_TOKEN

Branch flow: work branches start from `dev` and pull requests target `dev`. `main` receives only promotion pull requests from `dev` (merge commit). After a change lands on `main`, sync it back into `dev` with a merge-commit pull request.

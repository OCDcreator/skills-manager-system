# Skills Browser Foundation Design

## Summary

This phase builds the first usable vertical slice of `skills-manager-system`: a lightweight desktop app that reads the real `my-skills` repository, lets the user configure the repository path, scans the repository for skills, and displays the results in a searchable browser with a detail panel.

The goal is to establish the real application skeleton and the core repo-scanning contract without prematurely taking on Git sync, agent assignment, scene management, or project distribution. This keeps the first delivery small but production-directed.

## Product Goal

Deliver a minimal but real desktop workflow:

1. Launch a Tauri desktop app with a React frontend and Rust backend.
2. Save and load the configured skill repository path.
3. Scan the actual repository on disk.
4. Display discovered skills grouped by source (`custom` and `external`).
5. Support search, refresh, and skill detail preview.

This phase is intentionally read-only. It is a browser for the repository, not yet a manager for agent sync or scenario assignment.

## In Scope

- Tauri 2 application scaffold for Windows and macOS compatibility
- React + TypeScript + Tailwind frontend foundation
- Rust command layer plus core domain modules
- Settings page for the skill repository path
- Real repository scan of the configured path
- Search and source filtering in the skills list
- Skill detail panel with metadata and raw `SKILL.md` preview
- i18n wiring for English and Chinese strings used in this phase
- Validation through existing JS verify plus Rust checks/tests added for scanning logic

## Out of Scope

- Skill enable/disable state
- Agent detection and sync via symlink or copy
- Scene management
- Project-local assignment
- Git status, diff, commit, push, pull, or update script integration
- Marketplace, import flows, backups, or release management
- Rich metadata parsing beyond the minimal fields needed now

## Recommended Approach

Use a real vertical slice instead of a mock-first shell:

- The frontend talks to real Tauri commands from day one.
- The backend scans the real repository layout from day one.
- The UI is intentionally narrow: one browse surface and one settings surface.

This gives the project a stable backbone for later expansion. The second phase can add mutable actions on top of already-proven scanning and configuration boundaries, instead of replacing mocks or migrating temporary data structures.

## Architecture

### Frontend

The frontend should remain thin and orchestration-focused:

- `src/App.tsx` owns the top-level shell and view switching.
- `src/context/AppContext.tsx` owns app-level state for repository path, scan result, loading status, warnings, and refresh actions.
- `src/views/skills/SkillsView.tsx` assembles the browse page.
- `src/views/settings/SettingsView.tsx` assembles the repository-path settings page.
- `src/lib/tauri.ts` is the only direct frontend entry to Tauri commands.
- `src/lib/skills/filters.ts` contains skill-list filtering and grouping logic.

The view layer should not embed command details or file-system assumptions. It receives normalized data from the backend and renders it.

### Backend

Rust follows the project rule of thin commands and core-domain logic:

- `src-tauri/src/commands/settings.rs` exposes read/write settings commands.
- `src-tauri/src/commands/skills.rs` exposes scan and document-read commands.
- `src-tauri/src/core/settings.rs` handles persisted app settings.
- `src-tauri/src/core/skills/scan.rs` scans `custom/` and `external/`.
- `src-tauri/src/core/skills/metadata.rs` parses `SKILL.md` metadata.
- `src-tauri/src/core/skills/documents.rs` reads raw document content for details.

The command layer is only responsible for argument handling and error mapping. All repository knowledge lives in `core`.

## Repository Model

The configured skill repository points to the local `my-skills` checkout. The scanner assumes this repository shape:

- `custom/` contains first-party skill directories.
- `external/` contains third-party source repositories, where valid skill directories may exist at deeper levels.

The scanner returns a normalized `SkillSummary` shape with at least:

- `id`
- `name`
- `description`
- `sourceType` (`custom` or `external`)
- `relativePath`
- `directoryPath`
- `skillDocumentPath`

The detail command returns a `SkillDocument` shape with:

- `id`
- `name`
- `description`
- `sourceType`
- `relativePath`
- `content`

IDs must be stable, derived from source type plus repository-relative path, for example:

- `custom:searxng`
- `external:awesome-claude-skills/some-skill`

This avoids random identifiers and keeps selection state stable across refreshes.

## Scan Rules

### `custom/`

- Only inspect first-level child directories under `custom/`.
- A directory counts as a skill only if it contains `SKILL.md`.
- Nested skill discovery is intentionally ignored in `custom/` for this phase.
- If future repository structure introduces nested custom skills with their own `SKILL.md` files, revisit this rule instead of stretching the one-level scanner with ad hoc exceptions.

### `external/`

- Recursively scan for directories containing `SKILL.md`.
- Treat the directory containing `SKILL.md` as the skill root.
- Use the path relative to the repository root as the canonical path.
- External source directories that do not contain any leaf `SKILL.md` files are treated as reference sources, not skills, and therefore do not appear in the browse results.

### Ignored Directories

The recursive walk should skip common non-source directories when encountered:

- `.git`
- `node_modules`
- `dist`
- `target`
- `.tmp-skills`

### Metadata Parsing

The scanner should parse the YAML frontmatter of `SKILL.md` when present and read:

- `name`
- `description`

If parsing fails or the fields are absent:

- `name` falls back to the skill directory name
- `description` falls back to empty

Malformed metadata must not exclude the skill from results.

## UI Design

### App Shell

The first phase uses a simple application shell with two destinations:

- `Skills`
- `Settings`

The shell does not need complex routing yet. A lightweight local navigation state is enough as long as the structure remains easy to expand later.

### Skills View

The `Skills` view contains:

1. A header row with search input and refresh action
2. A source summary/filter area for `all`, `custom`, and `external`
3. A central list grouped by source
4. A detail panel for the selected skill

Each skill row shows:

- Name
- Description snippet truncated to at most 100 characters, with the full description reserved for the detail panel
- Source badge
- Repository-relative path

The list and source summaries should describe discovered skills, not raw external source directories. This avoids implying that every folder under `external/` is browseable as a skill.

The detail panel shows:

- Name
- Description
- Source type
- Relative path
- Raw `SKILL.md` content rendered as Markdown or preformatted text

### Settings View

The `Settings` view contains a single focused form:

- Repository path input
- Browse/select-directory action
- Save action
- Inline success or error feedback

After saving:

- The path is persisted
- App state refreshes the current repository path
- A scan is triggered immediately
- Returning to `Skills` should show the updated result set

## Data Flow

The intended flow is:

1. App starts.
2. Frontend loads the persisted repository path.
3. If the path exists, frontend requests a scan.
4. Backend returns normalized results plus warnings if needed.
5. Frontend stores the results in `AppContext`.
6. `SkillsView` applies local filtering and selection.
7. Selecting a skill loads document content on demand.

Path changes from `Settings` should reuse the same flow instead of inventing a separate refresh mechanism.

## Error Handling

The app should degrade gracefully:

- No configured path: show an empty state with a clear prompt to go to `Settings`.
- Configured path missing: show an error banner and no crash.
- Repository root readable but some subtrees fail: keep partial results and surface warnings.
- Metadata parse failure: keep the skill, use fallback values.
- Document read failure for a selected skill: keep the rest of the page responsive and show a panel-level error.

Errors should be phrased for a desktop end user, not exposed as raw Rust internals unless included as secondary detail.

## Module Boundaries

The code layout should align with the repo's modularity rules and avoid both overgrowth and fragmentation.

Recommended first-phase file set:

- `src/main.tsx`
- `src/App.tsx`
- `src/context/AppContext.tsx`
- `src/lib/tauri.ts`
- `src/lib/skills/filters.ts`
- `src/views/skills/SkillsView.tsx`
- `src/views/settings/SettingsView.tsx`
- `src/components/layout/AppShell.tsx`
- `src/components/skills/SkillList.tsx`
- `src/components/skills/SkillFilters.tsx`
- `src/components/skills/SkillDetailPanel.tsx`
- `src/components/settings/RepoPathForm.tsx`
- `src/i18n/en.json`
- `src/i18n/zh.json`
- `src-tauri/src/lib.rs`
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/commands/settings.rs`
- `src-tauri/src/commands/skills.rs`
- `src-tauri/src/core/mod.rs`
- `src-tauri/src/core/settings.rs`
- `src-tauri/src/core/skills/mod.rs`
- `src-tauri/src/core/skills/scan.rs`
- `src-tauri/src/core/skills/metadata.rs`
- `src-tauri/src/core/skills/documents.rs`

The exact scaffold can shift slightly during implementation, but the boundaries should remain:

- view orchestration in frontend views
- UI pieces in components
- Tauri API wrappers in frontend lib
- business logic in Rust core
- thin Rust commands

## Validation Strategy

Phase-one completion should be backed by focused validation:

- `npm run verify`
- `cargo check --manifest-path src-tauri/Cargo.toml`
- `cargo test --manifest-path src-tauri/Cargo.toml`

Rust tests should cover at least:

- `custom/` one-level scanning
- `external/` recursive scanning
- ignored-directory behavior
- metadata fallback behavior when frontmatter is missing or malformed
- stable ID generation from source type and relative path

Frontend does not need a new test framework in this phase. The repo currently has no frontend test runner, so validation should remain focused on architecture checks plus backend tests.

## Future Expansion Path

This design deliberately sets up the next phases without implementing them yet:

- enable/disable state can be added as persisted frontend/backend data attached to the scanned skill list
- Git workspace status can be added as a separate domain without changing scan contracts
- scene and project assignment can build on the stable skill IDs and repository-relative paths
- agent sync can later consume the same discovered skill model

By keeping the first phase read-only and repository-centered, the codebase gets a dependable base layer before mutable workflows are introduced.

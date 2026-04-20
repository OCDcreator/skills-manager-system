# Skill Enable State Design

## Summary

Phase 2 adds the first mutable workflow to `skills-manager-system`: persistent skill enable/disable state inside the app.

The existing phase-one browser remains the discovery backbone:

- the app still scans the real `my-skills` repository
- the app still uses stable skill IDs derived from source type plus repo-relative path
- the app still previews raw `SKILL.md` documents

What changes in this phase is that the browser can now remember whether a discovered skill is enabled or disabled for the current repository.

This state is intentionally app-local only. Phase 2 does **not** write into agent skill directories, project-local config directories, or the `my-skills` repository itself.

## Product Goal

Deliver the smallest useful mutation layer on top of the phase-one skill browser:

1. Let the user enable or disable any discovered skill from the UI.
2. Persist that choice locally across app restarts.
3. Scope the stored state to the configured repository path.
4. Keep disabled skills visible, filterable, and inspectable.
5. Preserve the current module boundaries: thin Tauri commands, Rust business logic in `core`, and view-layer orchestration in React.

## In Scope

- Local persistent skill enable/disable state
- Persistence scoped by configured repository path
- Stable skill-ID join against the existing scan results
- UI status badges and row-level toggle action
- UI status filtering for `all`, `enabled`, and `disabled`
- Keeping disabled skills visible in the list and selectable in the detail panel
- Tauri commands for reading current skill state and persisting one toggle action
- Rust unit tests for the new persistence rules
- Updates to module documentation required by the repo guard

## Out of Scope

- Agent detection or skill sync via symlink/copy
- Writing skill state into project directories such as `.claude/skills`, `.opencode/skills`, or `.agents/skills`
- Scene-level or project-level overrides
- Git status, diff, commit, push, pull, or update-script integration
- Reordering skill priority
- Bulk enable/disable workflows
- Editing skill metadata or writing back to `SKILL.md`
- Replacing the existing scan contract with a combined scan-plus-state backend payload

## Recommended Approach

Use a dedicated Rust `skill-state` module instead of expanding `settings.rs`.

### Why this is the right boundary

- `settings.rs` currently has one focused responsibility: persisted app settings, currently only `repo_path`.
- Enable/disable state is domain state for the skills module, not a generic app setting.
- A separate state store keeps phase-two logic close to the skills domain and leaves room for future scene/project layering without turning `settings.json` into a catch-all.

### Why not merge state into the scan response

The scan contract should remain the read-only source of truth for repository discovery:

- `scan.rs` answers “what skills exist in this repo?”
- `state.rs` answers “which stable IDs are disabled for this configured repo?”

The frontend can join those two sources cheaply and transparently. This preserves clean boundaries and keeps scan-specific tests focused on discovery rules rather than UI state.

## Architecture

### Frontend

The frontend keeps the current structure and adds only the minimum extra state:

- `src/context/AppContext.tsx` remains the orchestration boundary for app-level skill state
- `src/views/SkillsView.tsx` owns page-local search and filter state
- `src/components/skills/SkillFilters.tsx` renders search, source filters, and status filters
- `src/components/skills/SkillList.tsx` renders row-level enable/disable controls and visual status
- `src/components/skills/SkillDetailPanel.tsx` shows current enabled/disabled status alongside existing metadata and document content
- `src/lib/tauri.ts` defines the new Tauri DTO and command wrappers
- `src/lib/skills/filters.ts` extends its pure helper role to cover status filtering and status summary counts

No new frontend domain directory is needed in this phase. The existing browser files stay below the repo’s file-thickness thresholds and still represent stable responsibilities.

### Backend

Rust adds one new skills-domain module and keeps commands thin:

- `src-tauri/src/core/settings.rs` stays unchanged except as an existing dependency for repo path lookup
- `src-tauri/src/core/skills/state.rs` becomes the new persistence module for repo-scoped enable/disable state
- `src-tauri/src/core/skills/mod.rs` exports the new module
- `src-tauri/src/commands/skills.rs` adds `get_skill_state` and `set_skill_enabled`
- `src-tauri/src/lib.rs` registers the new commands

`scan.rs` and `documents.rs` remain focused on discovery and document loading. They should not absorb persistence concerns from phase 2.

## Persistence Model

### App Settings

`settings.json` remains responsible only for app settings:

```json
{
  "repoPath": "C:/Users/lt/Desktop/Write/custom-project/my-skills"
}
```

### Skill State Store

Phase 2 adds a separate `skill-state.json` under the same app config base directory.

Recommended shape:

```json
{
  "repos": {
    "c:/users/lt/desktop/write/custom-project/my-skills": {
      "repoPath": "C:/Users/lt/Desktop/Write/custom-project/my-skills",
      "disabledSkillIds": [
        "custom:searxng",
        "external:awesome-claude-skills/frontend-design"
      ]
    }
  }
}
```

### Repo Key Rules

The state store needs a repo key that is stable for the same physical repository:

1. Start from the configured repo path.
2. Prefer `canonicalize()` when possible.
3. Convert path separators to forward-slash form.
4. Lowercase the normalized key on Windows.

This gives the backend a stable bucket key without exposing platform-specific separators to the rest of the app.

### Why store disabled IDs instead of enabled IDs

The app should treat discovered skills as enabled by default.

Persisting only disabled IDs keeps the state minimal:

- zero stored IDs means “everything is enabled”
- newly discovered skills appear enabled automatically
- stale IDs can remain harmlessly in storage if a skill temporarily disappears

## Data Model

### Existing Stable Skill Identifier

Phase 1 already established the correct stable identifier:

- `custom:searxng`
- `external:awesome-claude-skills/frontend-design`

This comes from `sourceType + source-relative repo path`, as defined in the scan layer. Phase 2 should continue using that ID unchanged.

### New Skill State Snapshot

The new backend payload should stay small:

```ts
interface SkillStateSnapshot {
  disabledSkillIds: string[];
}
```

The frontend derives:

- `isEnabled = !disabledSkillIdSet.has(skill.id)`
- status counts for filter chips
- row styling and toggle labels

Phase 2 should **not** add `isEnabled` directly to the Rust `SkillSummary` DTO. The scan summary stays a discovery model; enabled/disabled remains a joined state in the frontend.

### Stale IDs

Stored IDs that do not appear in the latest scan result are allowed.

Rules:

- loading them must not error
- the UI ignores them because there is no matching current skill row
- they may remain persisted for future scans

This avoids destructive cleanup for temporary repo layout changes.

## UI Design

### Skill List Behavior

Each skill row gains:

- an enabled/disabled badge
- a toggle button
- subdued visual treatment when disabled

Disabled skills remain visible and selectable. This is the recommended default because it preserves discoverability and avoids the awkward question of how the user would re-enable a skill that disappeared from view.

### Status Filter

The browser gains a second filter dimension:

- `all`
- `enabled`
- `disabled`

This filter lives beside the existing search and source filters and operates on the joined frontend view model.

### Detail Panel

The detail panel keeps its current responsibility and adds one extra metadata row:

- `status: enabled|disabled`

The detail panel does not need a second toggle button in this phase. Row-level toggle plus detail visibility is enough for the smallest useful slice.

## Frontend Responsibilities

### `src/context/AppContext.tsx`

`AppContext` becomes the app-level orchestration point for:

- loading persisted repo path
- refreshing the scan result
- loading persisted disabled IDs for the current repo
- exposing one toggle action that persists a single state change
- keeping selection and document loading behavior unchanged

This is still within the intended scope of the context because the heavy I/O remains behind Tauri commands.

### `src/views/SkillsView.tsx`

`SkillsView` should continue to own only page-local view state:

- search string
- source filter
- status filter

It should derive filtered/grouped rows from `scanResult.skills` plus `disabledSkillIds`, but it should not talk directly to Tauri.

### `src/lib/skills/filters.ts`

This module remains the correct home for pure list helpers:

- source summaries
- status summaries
- combined search/source/status filtering

The file already exists for browser filtering, so phase 2 can extend it without creating a thin wrapper file.

## Backend Responsibilities

### `src-tauri/src/core/skills/state.rs`

This module should own:

- repo-scoped load logic
- repo-scoped update logic
- JSON serialization/deserialization
- repo-key normalization
- sorting and deduping disabled IDs before persistence

### `src-tauri/src/commands/skills.rs`

This file should remain thin:

- load configured repo path from `SettingsStore`
- delegate to `SkillStateStore`
- map errors to `String`

It should not implement repo-key normalization, JSON parsing, or skill-state mutation rules directly.

## Data Flow

Recommended refresh flow:

1. App starts.
2. Frontend loads persisted repo path.
3. If no repo path exists, show the existing unconfigured empty state.
4. If repo path exists, `AppContext.refreshSkills()` runs.
5. `scan_skills` returns discovered skills and warnings.
6. `get_skill_state` returns disabled IDs for the same configured repo.
7. Frontend joins scan results with disabled IDs and renders status-aware filters and rows.

Recommended toggle flow:

1. User clicks the row-level enable/disable action.
2. Frontend calls `set_skill_enabled(skillId, enabled)`.
3. Backend updates repo-scoped state and returns the full updated snapshot.
4. Frontend replaces its current `disabledSkillIds` with the returned snapshot.
5. Current list filters, selection, and detail panel react to the updated snapshot without requiring a rescan.

## Error Handling

### Refresh

If the scan fails:

- keep the current phase-one error behavior
- do not invent extra fallback state paths in this phase

If skill-state loading fails but scanning succeeds:

- keep the scan results visible
- fall back to an empty disabled-ID list in the current render pass
- show an error banner/message for the persistence failure

This preserves the browser as a usable read-only surface even when local state storage is unavailable.

### Toggle Failure

If a toggle write fails:

- keep the previous disabled-ID snapshot
- keep the current selected skill and document
- show an error message
- do not optimistically mutate then silently revert in this phase

This is the safest behavior for the first mutable slice.

## Module Boundaries

Expected source changes:

- Modify: `src/context/AppContext.tsx`
- Modify: `src/lib/tauri.ts`
- Modify: `src/lib/skills/filters.ts`
- Modify: `src/views/SkillsView.tsx`
- Modify: `src/components/skills/SkillFilters.tsx`
- Modify: `src/components/skills/SkillList.tsx`
- Modify: `src/components/skills/SkillDetailPanel.tsx`
- Modify: `src/i18n/en.json`
- Modify: `src/i18n/zh.json`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/src/commands/skills.rs`
- Modify: `src-tauri/src/core/skills/mod.rs`
- Create: `src-tauri/src/core/skills/state.rs`

Expected module-doc changes:

- Modify: `docs/modules/frontend/context/AppContext.md`
- Modify: `docs/modules/frontend/lib/tauri.md`
- Modify: `docs/modules/frontend/lib/skills/filters.md`
- Modify: `docs/modules/frontend/views/SkillsView.md`
- Modify: `docs/modules/frontend/components/skills/SkillFilters.md`
- Modify: `docs/modules/frontend/components/skills/SkillList.md`
- Modify: `docs/modules/frontend/components/skills/SkillDetailPanel.md`
- Modify: `docs/modules/tauri/lib.md`
- Modify: `docs/modules/tauri/commands/skills.md`
- Modify: `docs/modules/tauri/core/skills/mod.md`
- Create: `docs/modules/tauri/core/skills/state.md`

This preserves the repo’s module-documentation guard without introducing unnecessary extra source files.

## Validation Strategy

Phase-two completion should be validated with the repo’s existing gate plus focused Rust coverage:

- `npm run verify`
- `cargo test --manifest-path src-tauri/Cargo.toml`

Rust tests should cover at least:

- empty state when no skill-state file exists
- repo-scoped persistence isolation across two repo paths
- disabling a skill adds its stable ID
- re-enabling a skill removes its stable ID
- duplicate writes remain deduped and sorted
- stale/unknown IDs load without error

Frontend does not need a new test runner in phase 2. The existing repo gate already checks architecture, module-doc coverage, build success, `cargo check`, and `cargo test`.

## Future Expansion Path

This design keeps phase 2 small while setting up later phases cleanly:

- scene and project assignment can layer on top of the same stable skill IDs
- agent sync can later consume only enabled skills without changing the phase-two persistence contract
- repo-scoped global state can coexist with scene/project overrides rather than being rewritten

By keeping enable/disable state local, repo-scoped, and separate from scanning, phase 2 adds useful behavior without prematurely locking the app into a full assignment model.

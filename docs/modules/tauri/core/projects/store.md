# Project Config Store

> **Source**: `src-tauri/src/core/projects/store.rs`
> **Status**: [DRAFT]

## Overview

Persists project assignment configuration to `project-config.json` in the app config directory. Each project maps a project path to per-agent skill selection entries, with unsupported legacy agent keys retained separately for user-facing diagnostics.

## Public Surface

| Export | Purpose |
|---|---|
| `ProjectAssignment` | A single project's skill/agent mapping |
| `ProjectAgentAssignment` | Per-agent selected skills, selected scenes, and excluded skills |
| `ProjectAgentApplyStatus` / `ProjectApplyFreshness` | Transient per-agent apply freshness returned to the UI |
| `ProjectConfigSnapshot` | Full config state (projects map) |
| `ProjectConfigStore` | Persistence layer with CRUD operations |

## Core Logic

`ProjectConfigStore` provides atomic CRUD operations that load → mutate → save. Each method returns the full updated snapshot. The store now reuses shared path normalization from `project_paths.rs` so edit-mode persistence and read-only inspection stay aligned. Loads also normalize legacy map keys and embedded `projectPath` values, which keeps older Windows `C:\...` entries editable/removable after the stored format switches to portable `/` separators. If two legacy entries normalize to the same project path, load returns an explicit migration conflict instead of silently overwriting one assignment.

The current persisted project model is per-agent: `agents` is a map from supported agent key to `selectedSkillIds`, `selectedSceneIds`, and `excludedSkillIds`, while `unsupportedAgentKeys` preserves unknown keys. During load, legacy flat records with `skillIds` and `agentKeys` are migrated by copying the legacy skill list into every supported agent listed in `agentKeys`; unknown keys move into `unsupportedAgentKeys`. All project skill, scene, exclusion, and agent-key lists are trimmed, sorted, and deduped before save.

`applyStatuses` is a transient DTO field skipped during config serialization. `sync.rs` fills it after loading a snapshot so saved-project cards can show current/stale/never-applied state without writing status metadata into `project-config.json`.

For compatibility during the migration, `add_project(project_path, display_name, skill_ids, agent_keys)` and `update_project(..., skill_ids, agent_keys)` still accept the old flat command shape and convert it into per-agent entries. New store callers that already have per-agent state should use `add_project_with_agents` or `update_project_agents`.

## Interactions

- `sync.rs` — calls store for deploying project-local skills
- `project_paths.rs` — shared path normalization boundary
- `commands::projects` — thin wrappers around store methods

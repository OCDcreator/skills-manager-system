# Project Sync

> **Source**: `src-tauri/src/core/projects/sync.rs`
> **Status**: [DRAFT]

## Overview

Deploys skills into project-local agent directories (e.g., `.claude/skills`, `.opencode/skills`) based on layered project assignment configuration.

## Public Surface

| Export | Purpose |
|---|---|
| `apply_project_assignments` | Deploy skills to all configured projects |
| `attach_project_apply_statuses` | Annotate a project config snapshot with current/stale/never-applied per-agent status |
| `ProjectApplyResult` | Per-project apply outcome |

## Core Logic

For each project in the config, loads the agent inventory, selection context, and saved agent sync mode once, then resolves each project-agent target as a layer over that same agent's global direct skills and scenes. Project direct skills and scenes are added on top, and project exclusions are applied only to the final project-local write set. Global agent targets and other projects keep their own selections.

The apply path uses project-local catalog rules, so OpenCode targets `.opencode/skills` while its global sync path can remain `.config/opencode/skills`, and Cursor gets its own `.cursor/skills` manifest boundary. It maps persisted `AgentSyncMode` to the shared target reconciliation `SyncMode`, so project sync follows the configured copy/symlink behavior instead of forcing copy. The module still uses `agents::target_sync` with a project-specific ledger and portable path strings for stored target paths. Ledger entries now include a stable resolution hash plus `appliedAt` timestamp so config snapshots can report `current`, `stale`, or `neverApplied` without mutating target folders. Before applying a project, the sync path cleans any managed ledger entry whose stored target directory no longer matches the current project-local rule, which migrates old OpenCode/Cursor targets without leaving stale managed content behind. Legacy target comparison only treats `\` as a separator on Windows so macOS filenames containing backslashes are not rewritten.

Resolver diagnostics are surfaced through each agent apply result message. Missing project/global scene IDs, missing skill IDs, and globally disabled references append deterministic warning text and mark the agent result `Partial`, while unmanaged target conflicts still keep their existing partial status.

## Interactions

- `store.rs` — reads project configuration
- `agents::selection` — resolves layered global plus project skill sets
- `settings.rs` — provides the saved copy/symlink mode
- `agents::target_sync` — reuses `DesiredSkillEntry`, `apply_desired_entries`, and cleanup helpers
- `commands::projects` — thin wrapper for Tauri command

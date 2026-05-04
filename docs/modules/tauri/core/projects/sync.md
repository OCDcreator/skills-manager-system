# Project Sync

> **Source**: `src-tauri/src/core/projects/sync.rs`
> **Status**: [DRAFT]

## Overview

Deploys skills into project-local agent directories (e.g., `.claude/skills`, `.opencode/skills`) based on project assignment configuration.

## Public Surface

| Export | Purpose |
|---|---|
| `apply_project_assignments` | Deploy skills to all configured projects |
| `ProjectApplyResult` | Per-project apply outcome |

## Core Logic

For each project in the config, copies enabled skills from the my-skills repo into the project's local agent skill directories. Uses project-local catalog rules, so OpenCode targets `.opencode/skills` while its global sync path can remain `.config/opencode/skills`, and Cursor gets its own `.cursor/skills` manifest boundary. Uses the shared target reconciliation helpers from `agents::target_sync` with a project-specific ledger and portable path strings for stored target paths. Before applying a project, the sync path cleans any managed ledger entry whose stored target directory no longer matches the current project-local rule, which migrates old OpenCode/Cursor targets without leaving stale managed content behind. Legacy target comparison only treats `\` as a separator on Windows so macOS filenames containing backslashes are not rewritten.

## Interactions

- `store.rs` — reads project configuration
- `agents::target_sync` — reuses `DesiredSkillEntry`, `apply_desired_entries`, and cleanup helpers
- `commands::projects` — thin wrapper for Tauri command

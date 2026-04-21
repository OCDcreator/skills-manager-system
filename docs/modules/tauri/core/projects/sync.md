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

For each project in the config, copies enabled skills from the my-skills repo into the project's local agent skill directories. Uses the shared target reconciliation helpers from `agents::target_sync` with a project-specific ledger.

## Interactions

- `store.rs` — reads project configuration
- `agents::target_sync` — reuses `DesiredSkillEntry`, `apply_desired_entries`, and cleanup helpers
- `commands::projects` — thin wrapper for Tauri command

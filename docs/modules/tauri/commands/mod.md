# Commands Module Boundary

> **Source**: `src-tauri/src/commands/mod.rs`
> **Status**: [REVIEW]

## Overview

Declares the Rust command submodules exposed to the Tauri application layer.

## Import Relationships

```text
Upstream: src-tauri/src/lib.rs
Downstream: src-tauri/src/commands/agent_targets.rs, src-tauri/src/commands/agents.rs, src-tauri/src/commands/assistant.rs, src-tauri/src/commands/git.rs, src-tauri/src/commands/projects.rs, src-tauri/src/commands/scenes.rs, src-tauri/src/commands/settings.rs, src-tauri/src/commands/skills.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `agent_targets` | Explicit per-entry management commands for global agent skill directories. |
| `agents` | Agent inventory/config/apply command module. |
| `assistant` | Project assistant context status and ask command module. |
| `git` | Git status/diff/log/pull/push/commit/fetch/sync command module. |
| `projects` | Project assignment CRUD and apply command module. |
| `scenes` | Scene CRUD and apply command module. |
| `settings` | Settings command module. |
| `skills` | Skill browsing command module. |

## Core Logic

This is an aggregation module only; it has no runtime branching or data transformation.

## Data Flow

Not applicable.

## Interactions

Must include any new command module that is registered in `tauri::generate_handler!`.

## Configuration

None.

## Change Notes

Keep command modules thin; business rules belong under `src-tauri/src/core/`.

# Projects Commands

> **Source**: `src-tauri/src/commands/projects.rs`
> **Status**: [DRAFT]

## Overview

Thin Tauri command wrappers for project assignment operations. Maps core results to `Result<T, String>` for the frontend.

## Commands

| Command | Purpose |
|---|---|
| `get_project_config` | Load current project config snapshot with transient apply freshness when repo context is available |
| `add_project` | Register a new project with skills/agents |
| `add_project_with_agents` | Register a new project with per-agent skills/scenes/exclusions |
| `update_project` | Modify project skill/agent assignments |
| `update_project_with_agents` | Modify a project with per-agent skills/scenes/exclusions |
| `remove_project` | Remove a project assignment |
| `inspect_project_assignment_path` | Read-only path validation and per-agent marker/target inspection |
| `delete_project_target_skill` | Delete one unmanaged project-local target skill entry for an addressed agent |
| `apply_project_assignments` | Deploy skills to project-local agent dirs |

## Interactions

- `core::projects::store` — CRUD operations and per-agent project-layer persistence
- `core::projects::path_inspection` — preview inspection logic
- `core::projects::sync` — apply logic

## Current Note

The command layer stays thin: target deletion is delegated to `core::projects::path_inspection`, while managed target deselection remains a frontend draft operation followed by the normal project apply flow.

# Projects Commands

> **Source**: `src-tauri/src/commands/projects.rs`
> **Status**: [DRAFT]

## Overview

Thin Tauri command wrappers for project assignment operations. Maps core results to `Result<T, String>` for the frontend.

## Commands

| Command | Purpose |
|---|---|
| `get_project_config` | Load current project config snapshot |
| `add_project` | Register a new project with skills/agents |
| `update_project` | Modify project skill/agent assignments |
| `remove_project` | Remove a project assignment |
| `apply_project_assignments` | Deploy skills to project-local agent dirs |

## Interactions

- `core::projects::store` — CRUD operations
- `core::projects::sync` — apply logic

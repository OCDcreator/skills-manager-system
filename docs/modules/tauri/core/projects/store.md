# Project Config Store

> **Source**: `src-tauri/src/core/projects/store.rs`
> **Status**: [DRAFT]

## Overview

Persists project assignment configuration to `project-config.json` in the app config directory. Each project maps a project path to a set of skill IDs and agent keys.

## Public Surface

| Export | Purpose |
|---|---|
| `ProjectAssignment` | A single project's skill/agent mapping |
| `ProjectConfigSnapshot` | Full config state (projects map) |
| `ProjectConfigStore` | Persistence layer with CRUD operations |

## Core Logic

`ProjectConfigStore` provides atomic CRUD operations that load → mutate → save. Each method returns the full updated snapshot. The store now reuses shared path normalization from `project_paths.rs` so edit-mode persistence and read-only inspection stay aligned. Loads also normalize legacy map keys and embedded `projectPath` values, which keeps older Windows `C:\...` entries editable/removable after the stored format switches to portable `/` separators. If two legacy entries normalize to the same project path, load returns an explicit migration conflict instead of silently overwriting one assignment.

## Interactions

- `sync.rs` — calls store for deploying project-local skills
- `project_paths.rs` — shared path normalization boundary
- `commands::projects` — thin wrappers around store methods

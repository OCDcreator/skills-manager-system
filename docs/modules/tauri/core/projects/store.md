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

`ProjectConfigStore` provides atomic CRUD operations that load → mutate → save. Each method returns the full updated snapshot. The store validates project paths and handles deduplication.

## Interactions

- `sync.rs` — calls store for deploying project-local skills
- `commands::projects` — thin wrappers around store methods

# Projects View

> **Source**: `src/views/ProjectsView.tsx`
> **Status**: [DRAFT]

## Overview

Page for managing project-level skill assignments. Allows users to register project directories, select skills and agents, and deploy skills to project-local agent directories.

## State

- `config` — current `ProjectConfigSnapshot` from Rust backend
- `isLoading` — loading state for initial fetch
- Form state for adding new projects

## Data Flow

1. On mount, calls `getProjectConfig()` to load current config
2. User fills add-project form (path, name, skills, agents)
3. `addProject()` → backend → refreshed config
4. Per-project apply button → `applyProjectAssignments()`

## i18n Keys

All `projects.*` keys in `src/i18n/en.json` and `src/i18n/zh.json`.

# Projects API

> **Source**: `src/lib/projects.ts`
> **Status**: [DRAFT]

## Overview

Frontend API types and Tauri invoke wrappers for project assignment operations.

## Public Surface

| Export | Purpose |
|---|---|
| `ProjectAssignment` | TypeScript type for a project's skill/agent mapping |
| `ProjectConfigSnapshot` | Full config state type |
| `ProjectApplyResult` | Apply outcome type |
| `getProjectConfig` | Load project config |
| `addProject` | Register a project |
| `updateProject` | Modify project assignment |
| `removeProject` | Remove a project |
| `applyProjectAssignments` | Deploy skills |

## Interactions

- `src/views/ProjectsView.tsx` — primary consumer

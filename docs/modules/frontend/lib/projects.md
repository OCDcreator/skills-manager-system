# Projects API

> **Source**: `src/lib/projects.ts`
> **Status**: [REVIEW]

## Overview

Frontend API types and Tauri invoke wrappers for project-assignment operations.

## Public Surface

| Export | Purpose |
|---|---|
| `ProjectAssignment` / `ProjectConfigSnapshot` | Frontend types for stored project assignments. |
| `ProjectApplyResult` | Per-project apply result including display name and per-agent outcomes. |
| `ApplyProjectAssignmentsResponse` | Aggregate payload returned by `apply_project_assignments`. |
| `getProjectConfig` / `addProject` / `updateProject` / `removeProject` | CRUD wrappers. |
| `applyProjectAssignments` | Runs project-local skill deployment. |

## Interactions

Consumed primarily by `src/views/ProjectsView.tsx`, which now reads `response.results` instead of assuming the command returns a bare array.

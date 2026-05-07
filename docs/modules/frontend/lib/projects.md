# Projects API

> **Source**: `src/lib/projects.ts`
> **Status**: [REVIEW]

## Overview

Frontend API types and Tauri invoke wrappers for project-assignment operations.

## Public Surface

| Export | Purpose |
|---|---|
| `ProjectAssignment` / `ProjectAgentAssignment` / `ProjectConfigSnapshot` | Frontend types for stored per-agent project assignments. |
| `ProjectAgentApplyStatus` / `ProjectApplyFreshness` | Frontend apply-freshness DTOs for saved project cards. |
| `ProjectPathInspection` / `ProjectPathInspectionAgentResult` | Read-only inspection payload for draft paths, including existing target skill entries. |
| `ProjectApplyResult` | Per-project apply result including display name, apply freshness, and per-agent outcomes. |
| `ApplyProjectAssignmentsResponse` | Aggregate payload returned by `apply_project_assignments`. |
| `getProjectConfig` / `addProject` / `updateProject` / `removeProject` | Legacy-compatible CRUD wrappers. |
| `addProjectWithAgents` / `updateProjectWithAgents` | Per-agent project-layer CRUD wrappers that preserve project scenes and project-local exclusions. |
| `inspectProjectAssignmentPath` | Validates a draft path and reports per-agent marker/target existence plus read-only target skill inventory. |
| `deleteProjectTargetSkill` | Deletes one unmanaged project-local target skill entry for an addressed project path and agent. |
| `applyProjectAssignments` | Runs project-local skill deployment. |

## Interactions

Consumed primarily by `src/views/ProjectsView.tsx`, which combines CRUD calls, draft-path inspection, unmanaged target deletion, and apply-all feedback from the same wrapper module. The inspection DTO mirrors the agent target-inventory row shape so the Projects inspector can show already-present project-local skills before any apply action. New Projects UI saves through the per-agent wrappers; the flat wrappers remain for compatibility with older call sites and config shapes.

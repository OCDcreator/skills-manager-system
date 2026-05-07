# Project Path Inspection

> **Source**: `src-tauri/src/core/projects/path_inspection.rs`
> **Status**: [REVIEW]

## Overview

Project target inspection and scoped project-local target mutation module for the Projects workbench.

## Public Surface

| Export | Purpose |
|---|---|
| `ProjectPathInspectionAgentResult` | Per-agent marker/target existence result plus read-only target skill inventory. |
| `ProjectPathInspection` | Aggregate inspection payload for one draft path. |
| `inspect_project_assignment_path` | Validates a draft path and reports marker/target status and target skill entries for requested or default agents. |
| `delete_project_target_skill` | Deletes one unmanaged project-local target skill entry from the addressed agent's project skills directory. |

## Responsibilities

- reuses shared path normalization
- resolves project-local agent target rules from the catalog
- reuses agent target inventory scanning for existing project-local skill entries
- reuses agent target deletion logic for scoped project-local target cleanup
- reports unsupported agent keys without mutating stored configuration
- keeps target skill scan failures scoped to the affected agent and returns them as warnings
- emits marker and target paths in the shared portable path-string format

## Current Note

Inspection remains read-only, but this module also exposes the narrow delete primitive used by the Projects UI for unmanaged target entries. Managed entries are still removed through draft selection/exclusion changes and the later project apply flow rather than by calling this delete function.

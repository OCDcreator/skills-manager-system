# Project Path Inspection

> **Source**: `src-tauri/src/core/projects/path_inspection.rs`
> **Status**: [REVIEW]

## Overview

Read-only inspection module for the Projects workbench.

## Public Surface

| Export | Purpose |
|---|---|
| `ProjectPathInspectionAgentResult` | Per-agent marker/target existence result. |
| `ProjectPathInspection` | Aggregate inspection payload for one draft path. |
| `inspect_project_assignment_path` | Validates a draft path and reports marker/target status for requested or default agents. |

## Responsibilities

- reuses shared path normalization
- resolves project-local agent target rules from the catalog
- reports unsupported agent keys without mutating stored configuration
- emits marker and target paths in the shared portable path-string format

## Current Note

The current branch change for this module is formatting-only; runtime behavior is unchanged.

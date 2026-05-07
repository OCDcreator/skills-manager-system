# Project Sync Status Tests

> **Source**: `src-tauri/src/core/projects/sync_status_tests.rs`
> **Status**: [REVIEW]

## Overview

Focused tests for project apply freshness metadata.

## Responsibilities

- creates disposable project config, repo skills, and project-local agent targets
- verifies a freshly applied project-agent layer reports `current`
- mutates the same project-agent layer and verifies the ledger reports `stale`

## Interactions

Exercises `apply_project_assignments`, `attach_project_apply_statuses`, and `ProjectConfigStore` without expanding the broader project sync regression module.

## Current Note

The current branch change for this module is formatting-only; runtime behavior is unchanged.

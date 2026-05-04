# Project Sync Tests

> **Source**: `src-tauri/src/core/projects/sync_tests.rs`
> **Status**: [REVIEW]

## Overview

Focused tests for project-local assignment deployment.

## Responsibilities

- verifies OpenCode uses `.opencode/skills` for project-local assignment
- verifies Cursor and Claude Code use independent project-local target directories
- verifies legacy ledger target directories are cleaned when project-local rules change

## Interactions

Exercises `sync.rs`, `ProjectConfigStore`, shared target reconciliation, and catalog project-local rules without growing the production sync module past the architecture warning line.

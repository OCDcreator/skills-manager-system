# Project Sync Tests

> **Source**: `src-tauri/src/core/projects/sync_tests.rs`
> **Status**: [REVIEW]

## Overview

Focused tests for project-local assignment deployment.

## Responsibilities

- verifies project sync inherits the matching agent's global scene selection and adds project scenes
- verifies project exclusions remove inherited skills only from the project-local target
- verifies globally disabled skills remain a hard gate across inherited and project-local layers
- verifies missing project scene and skill references report diagnostics in the returned apply message and do not panic during apply
- verifies saved copy mode and includes a non-Windows apply-level directory symlink regression
- verifies OpenCode uses `.opencode/skills` for project-local assignment
- verifies Cursor and Claude Code use independent project-local target directories
- verifies legacy ledger target directories are cleaned when project-local rules change, including legacy entries that predate resolution hashes

## Interactions

Exercises `sync.rs`, `ProjectConfigStore`, `AgentConfigStore`, `SceneConfigStore`, shared target reconciliation, and catalog project-local rules without growing the production sync module past the architecture warning line.

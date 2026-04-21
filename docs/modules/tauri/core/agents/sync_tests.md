# Agent Sync Tests

> **Source**: `src-tauri/src/core/agents/sync_tests.rs`
> **Status**: [REVIEW]

## Overview

Holds focused Rust tests for the phase-three agent-sync workflow.

## Import Relationships

```text
Upstream: src-tauri/src/core/agents/mod.rs
Downstream: src-tauri/src/core/agents/sync.rs, src-tauri/src/core/agents/config.rs, src-tauri/src/core/agents/discovery.rs, src-tauri/src/core/agents/target_sync.rs, src-tauri/src/core/skills/state.rs
```

## Public Surface

This is a test-only module and exports no runtime API.

## Core Logic

Builds disposable repositories and target directories to verify copy/symlink apply behavior, disabled-skill exclusion, stale managed-entry cleanup, unmanaged-content preservation, and override-path precedence. Tests import `SyncMode` from `target_sync.rs` and use `SyncMode::Copy` for reproducibility unless explicitly checking symlink preference propagation.

## Data Flow

Test fixtures create repo/config/target state, invoke `apply_agent_sync`, and assert filesystem results.

## Interactions

Must stay aligned with the stable target-name mapping and manifest safety rules implemented by `target_sync.rs`.

## Configuration

Uses `tempfile` for disposable filesystem state.

## Change Notes

Keep new phase-three regression tests here instead of growing `sync.rs` past the repo's file-size limits. When the CLI feature split changes sync visibility, update this doc alongside any source-only test fixture adjustments so module-doc diff remains green.

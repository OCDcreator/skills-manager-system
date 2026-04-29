# Agent Target Sync Tests

> **Source**: `src-tauri/src/core/agents/target_sync_tests.rs`
> **Status**: [REVIEW]

## Overview

Holds focused Rust tests for low-level target reconciliation safety rules.

## Test Coverage

| Test | Purpose |
|---|---|
| `unmanaged_entries_are_not_deleted_when_managed_entries_are_removed` | Confirms target sync removes only app-managed entries and preserves manual content. |

## Interactions

This safety rule matters for both traditional sync outputs and any future agent-visible directories that also contain imported external mirrors.

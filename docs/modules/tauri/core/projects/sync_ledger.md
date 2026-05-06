# Project Sync Ledger

> **Source**: `src-tauri/src/core/projects/sync_ledger.rs`
> **Status**: [REVIEW]

## Overview

Owns project sync ledger persistence and cleanup helpers.

## Public Surface

This module is internal to `core::projects`. It provides:

- `ProjectSyncLedger` and `ProjectSyncLedgerEntry`
- ledger load/save helpers for `project-sync-ledger.json`
- stale assignment cleanup for removed project-agent mappings
- retarget cleanup when a project-local agent rule changes
- portable ledger keys and legacy target comparison

## Interactions

`sync.rs` records fresh ledger entries after applying project targets. `apply_status.rs` reads the same ledger shape to compare current project-layer hashes with the last applied state.

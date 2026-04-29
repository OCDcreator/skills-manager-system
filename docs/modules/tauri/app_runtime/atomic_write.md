# Atomic Text Write Helper

> **Source**: `src-tauri/src/app_runtime/atomic_write.rs`
> **Status**: [REVIEW]

## Overview

Provides cross-platform atomic text-file persistence for config snapshots.

## Public Surface

| Export | Purpose |
|---|---|
| `write_text_atomic` | Writes text through a temp file plus swap/rename flow. |

## Core Logic

The helper creates the parent directory, writes and `sync_all()`s a create-new temp file, swaps it into place, then syncs the parent directory on non-Windows platforms. Windows uses `ReplaceFileW` when the destination already exists so config stores can avoid partial overwrites on that platform too.

## Interactions

Re-exported from `app_runtime::mod` and used by config stores such as `core/external_sources/store.rs`. Callers are expected to pair it with the config lock when the file participates in multi-step mutable workflows.

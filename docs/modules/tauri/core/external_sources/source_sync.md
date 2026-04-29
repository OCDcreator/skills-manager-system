# External Source Sync

> **Source**: `src-tauri/src/core/external_sources/source_sync.rs`
> **Status**: [REVIEW]

## Overview

Owns source-record mutation, cached fetch, and import refresh logic for external sources.

## Public Surface

| Export | Purpose |
|---|---|
| `upsert_source` | Adds or updates a normalized source record and returns its stable id. |
| `set_source_failure` | Persists a fetch failure onto the source record without panicking the service layer. |
| `fetch_source` | Refreshes the cached repo, detection result, warnings, and import update flags. |
| `cache_repo_absolute_path` | Resolves the expected cache location for one source id. |

## Core Logic

The module normalizes GitHub URLs into stable `src_<hash>` ids, keeps cached repos under `config/external-sources/<id>/repo`, fetches upstream state through `git_repo.rs`, and recalculates per-import `updateAvailable` plus `variant_disappeared` warnings after every fetch. Source status is derived from warning severity, not from a separate state machine.

## Interactions

`service.rs` uses this as its write-side helper, while `source_snapshot.rs` handles read-side projection. Keep the warning and cache-path rules aligned with `detect.rs`, `git_repo.rs`, and persisted models.

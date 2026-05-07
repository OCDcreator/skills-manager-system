# External Source Service Tests

> **Source**: `src-tauri/src/core/external_sources/service_tests.rs`
> **Status**: [REVIEW]

## Overview

Holds focused orchestration tests for source-level listing and destructive removal.

## Test Coverage

| Test | Purpose |
|---|---|
| `list_external_sources_reads_variants_from_persisted_fetch_commit` | Confirms read-only source listing uses the persisted fetched commit to discover variants, metadata, and direct child directory/file previews without depending on worktree files or remote HEAD resolution. |
| `remove_external_source_preflight_blocks_partial_import_deletion` | Confirms service-level removal stops before deleting any mirror when one imported skill is still referenced. |
| `remove_external_source_keeps_snapshot_when_cache_removal_fails` | Confirms cache-removal failures leave the persisted source/import snapshot untouched. |

## Interactions

These tests protect the `list_external_sources()` and `remove_external_source()` contracts in `service.rs`: source listing stays a local read from cached git objects when a fetched commit is recorded, including the variant folder's direct child directory/file previews; removal preflights every imported mirror first, then performs destructive work only if the whole batch is safe, and never commits source/import deletion if cache cleanup fails.

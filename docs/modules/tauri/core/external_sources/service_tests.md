# External Source Service Tests

> **Source**: `src-tauri/src/core/external_sources/service_tests.rs`
> **Status**: [REVIEW]

## Overview

Holds focused orchestration tests for source-level destructive removal.

## Test Coverage

| Test | Purpose |
|---|---|
| `remove_external_source_preflight_blocks_partial_import_deletion` | Confirms service-level removal stops before deleting any mirror when one imported skill is still referenced. |
| `remove_external_source_keeps_snapshot_when_cache_removal_fails` | Confirms cache-removal failures leave the persisted source/import snapshot untouched. |

## Interactions

These tests protect the `remove_external_source()` contract in `service.rs`: preflight every imported mirror first, then perform destructive work only if the whole batch is safe, and never commit source/import deletion if cache cleanup fails.

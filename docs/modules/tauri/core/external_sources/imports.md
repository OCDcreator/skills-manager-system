# External Source Imports

> **Source**: `src-tauri/src/core/external_sources/imports.rs`
> **Status**: [REVIEW]

## Overview

Provides the public import/remove/preflight API for managed mirrors under `external/managed/github/`.

## Public Surface

| Export | Purpose |
|---|---|
| `ImportVariantInput` | Import request payload with source id, agent key, variant path, and pinned commit. |
| `ImportVariantResult` | Serializable result for import, update, or repair flows. |
| `ImportRemovalResult` | Serializable result for destructive mirror removal. |
| `import_variant_into_repo` | Imports or updates one managed mirror into the repo. |
| `remove_imported_variant_from_repo` | Removes one managed mirror after reference and integrity checks. |
| `preflight_remove_imported_variant` | Validates that one mirror can be safely removed before a larger operation continues. |

## Core Logic

This module now owns workflow ordering rather than every helper detail itself. The import path/id rules live in `import_paths.rs`, filesystem validation and rollback helpers live in `mirror_fs.rs`, and git blob export stays in `git_export.rs`. `imports.rs` coordinates those helpers into one transactional flow: resolve the cached repo, choose or reuse a stable mirror path, stage the exported variant, validate it, swap it into the repo, then persist the updated import record under the same config lock. Import validation accepts `.` as the special upstream variant path for generic root skill repositories; all other upstream variant paths still go through the normal repo-relative path canonicalizer.

Removal preflight and destructive removal share the same guardrails: block referenced skills through `reference_check.rs`, require a valid live managed mirror, and restore backups if persistence fails mid-operation.

## Interactions

This module owns the mirror lifecycle API. It does not discover variants, fetch upstream commits, or project source snapshots; those belong to `detect.rs`, `git_repo.rs`, `source_sync.rs`, and `source_snapshot.rs`.

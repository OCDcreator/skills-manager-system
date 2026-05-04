# External Source Service

> **Source**: `src-tauri/src/core/external_sources/service.rs`
> **Status**: [REVIEW]

## Overview

Provides the public desktop-facing orchestration layer for source records, source snapshots, and managed import operations.

## Public Surface

| Export | Purpose |
|---|---|
| `AddExternalSourceInput` | Backend add/upsert request carrying `repoUrl` with optional requested `branch` and `subpath`. |
| `ExternalVariantSnapshot` | Serializable detected variant shape plus parsed skill metadata returned to the frontend. |
| `ExternalSourceSnapshotItem` | Source record plus detected variants and imports. |
| `ExternalSourcesListResponse` | List payload returned by list/add/fetch/remove flows. |
| `list_external_sources` | Loads current snapshots and injects runtime integrity warnings when a repo root is available. |
| `add_external_source` | Legacy repo-url-only wrapper that upserts a source record and attempts an immediate fetch. |
| `add_external_source_with_input` | Upserts a source record from the structured input and attempts an immediate fetch. |
| `fetch_external_source` | Refreshes one source from GitHub into the cache. |
| `import_external_variant` | Imports one detected variant into the managed mirror tree. |
| `update_external_import` | Re-imports an existing managed mirror at the fetched upstream head. |
| `remove_external_source` | Removes a source, optionally cascading import deletion and cache cleanup, but only after all destructive preflight checks succeed. |
| `repair_external_import` | Rebuilds a managed mirror while preserving a backup for rollback. |

## Core Logic

`service.rs` now focuses on use-case orchestration. Snapshot assembly is delegated to `source_snapshot.rs`, while source add/fetch/failure persistence lives in `source_sync.rs`. The structured add input lets command callers pass optional branch/subpath data while the repo-url-only wrapper preserves existing Rust call sites. The service stitches those helpers together for the command layer: list current source snapshots, add and optionally fetch a source, fetch one source, import/update/repair a managed mirror, and remove a source with import preflight when destructive cleanup is requested.

For destructive source removal, the service now preflights every imported mirror first, then deletes the cache directory before committing the source/import snapshot mutation so a cache-removal failure cannot leave app state already removed.

Runtime integrity warnings are still computed late during listing so the UI can surface broken live mirrors without first mutating `external-sources.json`. Variant snapshots now also include parsed `name` and `description` metadata, but that enrichment still happens read-only during snapshot assembly rather than through persisted source records. Source records may carry an optional `subpath`; the service keeps that value in the structured add/fetch/list flow while leaving the visible UI form unchanged for now.

## Interactions

Commands should call this module, not the lower-level helpers directly. The service also feeds the skill scanner indirectly by keeping `ImportedExternalSkillRecord` state current and by preserving the managed mirror layout that `managed_scan.rs` later enriches.

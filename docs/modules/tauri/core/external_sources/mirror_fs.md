# External Source Mirror Filesystem Helpers

> **Source**: `src-tauri/src/core/external_sources/mirror_fs.rs`
> **Status**: [REVIEW]

## Overview

Owns low-level filesystem validation and rollback helpers for managed mirrors.

## Public Surface

| Export | Purpose |
|---|---|
| `validate_target_path_for_import` | Rejects unmanaged or mismatched targets before an import/update swap. |
| `validate_live_mirror_for_removal` | Confirms a live mirror still matches its persisted import record. |
| `validate_mirror_dir` | Verifies `SKILL.md`, manifest contents, and derived skill id after staging or swap. |
| `write_manifest` | Writes `.skills-manager-source.json` into a staged mirror. |
| `upsert_import_record` | Replaces or inserts an import record while preserving sorted order. |
| `create_operation_dir` / `create_sibling_path` | Builds temp, backup, and rollback paths. |
| `restore_previous_target` / `remove_empty_managed_parents` | Handles rollback and managed-tree cleanup. |
| `current_timestamp_string` | Produces the import timestamp stored in the snapshot. |

## Core Logic

The module keeps import/update/remove flows safe by treating manifests as the source of truth for live mirror ownership. It refuses unmanaged collisions, restores backups when swaps fail, and only prunes empty managed-parent directories back toward `external/`.

## Interactions

`imports.rs` owns workflow order; this module owns the filesystem invariants that workflow depends on. Any manifest-shape change must stay aligned with `models.rs`, `skills::identity`, and runtime integrity checks in `source_snapshot.rs`.

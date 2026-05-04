# External Source Import Tests

> **Source**: `src-tauri/src/core/external_sources/imports_tests.rs`
> **Status**: [REVIEW]

## Overview

Covers the managed-mirror import and removal lifecycle at the Rust domain boundary.

## Test Coverage

| Test | Purpose |
|---|---|
| `import_variant_writes_manifest_and_updates_record` | Confirms import writes the mirror manifest and persists the import record. |
| `import_generic_root_variant_writes_manifest_and_preserves_unmanaged_collision` | Confirms the generic root variant path `.` exports root skill assets, writes the managed manifest, and avoids overwriting an unmanaged existing directory. |
| `remove_import_blocks_when_skill_is_referenced` | Confirms destructive removal stops when scenes still reference the managed skill. |
| `remove_import_succeeds_without_source_snapshot_entry` | Confirms mirror removal can proceed even if the source record is already gone. |
| `remove_import_succeeds_when_source_repo_url_drifts` | Confirms removal depends on live manifest/import metadata, not on the current source URL text. |

## Interactions

These tests validate the split between `imports.rs`, `mirror_fs.rs`, and `reference_check.rs`. They are the main proof that managed mirrors can be removed safely even when source snapshot state has drifted.

# Agent Target Inventory Tests

> **Source**: `src-tauri/src/core/agents/target_inventory_tests.rs`
> **Status**: [REVIEW]

## Overview

Verifies agent target-directory inventory classification for managed and unmanaged entries.

## Test Coverage

| Test | Purpose |
|---|---|
| `target_inventory_classifies_managed_and_unmanaged_entries` | Confirms folder-name sync entries remain marked managed while manual folders stay unmanaged and keep metadata-derived display names. |
| `target_inventory_ignores_manifest_and_keeps_unmanaged_after_cleanup` | Confirms managed cleanup does not surface the manifest file and does not erase unrelated manual entries. |
| `target_inventory_reports_symlink_target_paths` | Confirms visible directory symlinks report both their `symlink` entry kind and the normalized target path shown in the UI. |

## Interactions

These tests cover the same managed-vs-unmanaged boundary that external GitHub mirrors rely on when they later appear in agent target inventories.

# Agent Target Management Tests

> **Source**: `src-tauri/src/core/agents/target_management_tests.rs`
> **Status**: [REVIEW]

## Overview

Exercises the explicit adopt/delete/import flows for entries already present in a global agent skills directory.

## Test Coverage

| Test | Purpose |
|---|---|
| `taking_over_unmanaged_entry_keeps_existing_contents_when_sync_would_conflict` | Confirms take-over preserves manual contents even when later sync logic would otherwise collide. |
| `deleting_requested_entry_preserves_other_target_entries` | Confirms delete only removes the named entry. |
| `importing_unmanaged_entry_copies_into_repo_and_only_deletes_when_requested` | Confirms unmanaged imports land under `custom/` and only delete the source entry when requested. |

## Interactions

These tests stay focused on unmanaged global target entries, but they complement the external-source import rules by proving that explicit management actions do not broaden into destructive cleanup.

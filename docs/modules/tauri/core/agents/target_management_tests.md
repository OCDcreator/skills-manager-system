# Agent Target Management Tests

> **Source**: `src-tauri/src/core/agents/target_management_tests.rs`
> **Status**: [REVIEW]

## Overview

Exercises the explicit adopt/delete/import flows for entries found in a global agent skills directory.

## Covered Behaviors

| Test | Purpose |
|---|---|
| `taking_over_unmanaged_entry_keeps_existing_contents_when_sync_would_conflict` | Confirms preserved take-over entries block later repo-backed sync overwrites and keep their original files intact. |
| `deleting_requested_entry_preserves_other_target_entries` | Confirms delete only removes the named target entry and leaves other managed or unmanaged entries alone. |
| `importing_unmanaged_entry_copies_into_repo_and_only_deletes_when_requested` | Confirms imports land under `custom/` and preserve the original target entry unless deletion is explicitly requested. |

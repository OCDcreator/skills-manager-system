# Agent Target Sync Tests

> **Source**: `src-tauri/src/core/agents/target_sync_tests.rs`
> **Status**: [REVIEW]

## Overview

Holds focused Rust tests for low-level target reconciliation safety rules.

## Test Coverage

| Test | Purpose |
|---|---|
| `managed_entry_names_prefer_skill_directory_name_without_source_prefix` | Confirms synced target entries use the source skill folder name instead of adding `custom` or long external-source prefixes. |
| `duplicate_skill_directory_names_are_disambiguated_by_relative_path` | Keeps duplicate selected folder names safe by falling back to sanitized relative paths. |
| `unmanaged_entries_are_not_deleted_when_managed_entries_are_removed` | Confirms target sync removes only app-managed entries and preserves manual content. |
| `symlink_mode_links_the_skill_directory_entry_instead_of_each_document` | Verifies symlink sync points the target skill entry at the whole source folder so assets beside `SKILL.md` remain available. |
| `remove_target_deletes_windows_directory_symlink_without_deleting_source` | Guards Windows directory symlink cleanup so explicit deletion removes the link rather than failing with access denied or deleting the source. |
| `managed_entry_name_sanitizes_relative_path_when_disambiguation_is_needed` | Documents the fallback name sanitizer for collision cases. |

## Interactions

These safety rules matter for both traditional sync outputs and any future agent-visible directories that also contain imported external mirrors.

# Platform Path Helpers

> **Source**: `src-tauri/src/core/platform_paths.rs`
> **Status**: [REVIEW]

## Overview

Shared Rust helpers for storing and returning user-facing paths in a portable string format.

## Public Surface

| Export | Purpose |
|---|---|
| `normalize_portable_absolute_path` | Trims, validates absoluteness, converts Windows separators to `/`, and removes non-root trailing separators. |
| `portable_path_string` | Converts a trusted `Path` into the same persisted/DTO path format. |

## Core Logic

The module keeps Windows and macOS path persistence stable by normalizing Windows `\` separators to `/` and coalescing `C:/foo/` and `C:\foo\` into one stored value. On non-Windows platforms, backslash is preserved as a valid filename character while trailing `/` separators are still trimmed. Root paths such as `/` and `C:/` are preserved.

## Interactions

Used by settings, agent path overrides, project path storage, project inspection, and project sync result DTOs. Keep this module focused on path-string normalization; filesystem mutation stays in the owning feature modules.

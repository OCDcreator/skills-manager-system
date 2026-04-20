# Settings Commands

> **Source**: `src-tauri/src/commands/settings.rs`
> **Status**: [REVIEW]

## Overview

Exposes Tauri commands for loading and saving the configured skill repository path.

## Import Relationships

```text
Upstream: src-tauri/src/lib.rs, src/lib/tauri.ts
Downstream: src-tauri/src/core/settings.rs, tauri::Manager
```

## Public Surface

| Export | Purpose |
|---|---|
| `get_repo_path` | Returns the persisted repository path, if configured. |
| `set_repo_path` | Trims and persists a new repository path or clears it when empty. |

## Core Logic

Both commands resolve the app config directory through Tauri, create a `SettingsStore`, delegate persistence to core settings logic, and map errors to strings for the frontend.

## Data Flow

Frontend invokes command -> command resolves config directory -> `SettingsStore` loads/saves `settings.json` -> command returns `Option<String>`.

## Interactions

Must stay aligned with `src/lib/tauri.ts` wrapper names and the `SettingsStore` JSON format.

## Configuration

Uses Tauri's app config directory as the storage base.

## Change Notes

Keep path normalization rules minimal here; deeper validation should live in `core/settings.rs` or a dedicated core module. The git commands module reuses the same repo-path loading pattern.

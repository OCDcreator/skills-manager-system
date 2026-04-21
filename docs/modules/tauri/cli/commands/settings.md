# CLI Settings Commands

> **Source**: `src-tauri/src/cli/commands/settings.rs`
> **Status**: [REVIEW]

## Overview

Implements `settings get-repo-path`, `settings set-repo-path`, `settings get-sync-mode`, and `settings set-sync-mode`.

## Import Relationships

```text
Upstream: src-tauri/src/cli/mod.rs
Downstream: src-tauri/src/app_runtime/*, src-tauri/src/core/settings.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `run` | Routes one parsed settings subcommand to its JSON response builder. |

## Core Logic

Repo-path output uses the effective runtime resolution order, so `--repo` wins over saved settings without mutating the underlying file. Mutation commands acquire the advisory config lock and write through `SettingsStore`, preserving the shared `settings.json` schema. Sync-mode mutation validates `copy` / `symlink` and maps unsupported values to `invalid_sync_mode`.

## Interactions

Must stay aligned with `SettingsStore`, `AppRuntimeContext.current_repo_path`, config-lock error mapping, and stable error code names emitted through `CliCommandError`.

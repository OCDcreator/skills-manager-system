# CLI Settings Commands

> **Source**: `src-tauri/src/cli/commands/settings.rs`
> **Status**: [REVIEW]

## Overview

Implements the read-only `settings get-repo-path` and `settings get-sync-mode` commands.

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

Repo-path output uses the effective runtime resolution order, so `--repo` wins over saved settings without mutating the underlying file. Sync-mode output reads the shared `settings.json` file and returns the persisted `copy` / `symlink` enum value in CLI JSON.

## Interactions

Must stay aligned with `SettingsStore`, `AppRuntimeContext.current_repo_path`, and the stable error code names emitted through `CliCommandError`.

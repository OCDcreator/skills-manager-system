# App Runtime Context

> **Source**: `src-tauri/src/app_runtime/context.rs`
> **Status**: [REVIEW]

## Overview

Resolves the shared application config directory, applies `--repo` / `--config-dir` overrides, normalizes emitted paths, and provides the config lock entrypoint for future mutation commands.

## Import Relationships

```text
Upstream: src-tauri/src/cli/mod.rs, src-tauri/src/cli/commands/*
Downstream: src-tauri/src/core/settings.rs, dirs, std::fs
```

## Public Surface

| Export | Purpose |
|---|---|
| `AppRuntimeOptions` | CLI-facing inputs for config-dir, repo, and output mode. |
| `AppRuntimeContext` | Shared runtime snapshot for command handlers. |
| `active_app_identifier` | Picks the production vs dev Tauri identifier. |
| `tauri_app_config_dir` | Recreates the Tauri-style app config path from the identifier. |
| `normalize_output_path` | Forces forward slashes for JSON-facing paths. |

## Core Logic

The context resolves the config root from `dirs::config_dir()` unless `--config-dir` is supplied, joins that root with the active app identifier, and reads persisted settings from the same files used by the desktop app. Repo-path lookup always prefers the CLI override over saved settings. The lock helper uses a config-root lock file so future mutation commands can coordinate writes without pulling Tauri runtime types into the CLI.

## Data Flow

CLI flags enter through `AppRuntimeOptions`, become an `AppRuntimeContext`, and then feed every command that needs config, repo, sync-mode, or lock information.

## Interactions

The production/dev identifiers must stay aligned with `src-tauri/tauri.conf.json` and `src-tauri/tauri.dev.conf.json`. Repo-path reads depend on `src-tauri/src/core/settings.rs`.

## Change Notes

Keep this layer environment-focused. Do not move scene/project/git business rules here; the runtime should only resolve execution context and shared filesystem concerns.

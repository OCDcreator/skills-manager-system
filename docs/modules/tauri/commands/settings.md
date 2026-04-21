# Settings Commands

> **Source**: `src-tauri/src/commands/settings.rs`
> **Status**: [REVIEW]

## Overview

Exposes Tauri commands for loading and saving both the configured repository path and the preferred global agent sync mode.

## Public Surface

| Export | Purpose |
|---|---|
| `get_repo_path` / `set_repo_path` | Load or update the saved my-skills repository path. |
| `get_agent_sync_mode` / `set_agent_sync_mode` | Load or update the saved `copy`/`symlink` preference. |

## Core Logic

Each command resolves the app config directory, delegates to `SettingsStore`, and returns only the requested field so the frontend can treat settings reads/writes as thin command calls.

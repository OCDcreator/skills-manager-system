# Settings Commands

> **Source**: `src-tauri/src/commands/settings.rs`
> **Status**: [REVIEW]

## Overview

Exposes Tauri commands for loading and saving the configured repository path, the preferred global agent sync mode, and the persisted global agent ordering keys.

## Public Surface

| Export | Purpose |
|---|---|
| `get_repo_path` / `set_repo_path` | Load or update the saved my-skills repository path. |
| `get_agent_sync_mode` / `set_agent_sync_mode` | Load or update the saved `copy`/`symlink` preference. |
| `get_agent_order` / `set_agent_order` | Load or update the saved global agent ordering array. |

## Core Logic

Each command resolves the app config directory, delegates to `SettingsStore`, and returns only the requested field so the frontend can treat settings reads/writes as thin command calls.

# Settings Store

> **Source**: `src-tauri/src/core/settings.rs`
> **Status**: [REVIEW]

## Overview

Owns persistence for app settings stored in `settings.json` under the Tauri app config directory.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentSyncMode` | Persisted enum for `copy` vs `symlink`. |
| `AppSettings` | Serializable payload with `repo_path` and `agent_sync_mode`. |
| `SettingsStore::load` | Reads settings or returns defaults when missing. |
| `SettingsStore::save_repo_path` | Updates only the repository path while preserving other settings. |
| `SettingsStore::save_agent_sync_mode` | Updates only the preferred sync mode. |

## Core Logic

Uses serde defaults so older `settings.json` files that only contain `repoPath` still load cleanly, defaulting `agentSyncMode` to `copy`.

# Settings Store

> **Source**: `src-tauri/src/core/settings.rs`
> **Status**: [REVIEW]

## Overview

Owns persistence for app settings stored in `settings.json` under the Tauri app config directory.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentSyncMode` | Persisted enum for `Copy` vs `Symlink`. Default is `Symlink`. |
| `AppSettings` | Serializable payload with `repo_path`, `agent_sync_mode`, persisted `agent_order`, and the last assistant terminal working directory. |
| `SettingsStore::load` | Reads settings or returns defaults when missing. |
| `SettingsStore::save_repo_path` | Updates only the repository path while preserving other settings. |
| `SettingsStore::save_agent_sync_mode` | Updates only the preferred sync mode. |
| `SettingsStore::save_agent_order` | Updates only the preferred global agent key order. |
| `SettingsStore::save_assistant_working_directory` | Updates only the last assistant-terminal working directory preference. |

## Core Logic

Uses serde defaults so older `settings.json` files that only contain `repoPath` still load cleanly, defaulting `agentSyncMode` to `Symlink` and `agentOrder` to an empty array. Saved repo paths and assistant working directories are serialized through `platform_paths` so Windows and macOS share the same slash-normalized, trailing-separator-trimmed format.

The `Symlink` default is reflected on the frontend side: `AgentsView` initializes its `syncMode` state to `"symlink"`, matching the Rust default. Older settings files that explicitly stored `"copy"` continue to load correctly via serde deserialization.

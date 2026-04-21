# Agent Commands

> **Source**: `src-tauri/src/commands/agents.rs`
> **Status**: [REVIEW]

## Overview

Provides thin Tauri commands for agent inventory, target configuration, and manual global apply.

## Import Relationships

```text
Upstream: src-tauri/src/lib.rs
Downstream: core::agents::{config, discovery, sync, target_sync}, core::settings
```

## Public Surface

| Export | Purpose |
|---|---|
| `get_agent_inventory` | Returns the current agent inventory snapshot. |
| `set_agent_enabled` / `set_agent_path_override` / `clear_agent_path_override` | Persist per-agent target settings and return refreshed inventory. |
| `apply_agent_sync` | Runs one manual global sync pass using an explicit mode override or the saved setting. |

## Core Logic

`apply_agent_sync` resolves sync mode from either the command argument or `settings.json`, then delegates orchestration to `core::agents::sync`. The command layer only translates persisted `AgentSyncMode` into `target_sync::SyncMode`.

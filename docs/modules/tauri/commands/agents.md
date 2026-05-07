# Agent Commands

> **Source**: `src-tauri/src/commands/agents.rs`
> **Status**: [REVIEW]

## Overview

Provides thin Tauri commands for agent inventory, per-agent configuration, and manual apply.

## Public Surface

| Export | Purpose |
|---|---|
| `get_agent_inventory` | Returns the current agent inventory snapshot. |
| `set_agent_enabled` / `set_agent_path_override` / `clear_agent_path_override` | Persist focused per-agent target settings and return refreshed inventory. |
| `set_agent_configuration` | Persists a full per-agent draft payload and returns refreshed inventory. |
| `apply_agent_sync` | Runs manual sync using an explicit mode override or the saved setting, optionally scoped to one agent. |

## Core Logic

`apply_agent_sync` resolves sync mode from either the command argument or `settings.json`, validates optional single-agent scope, and then delegates orchestration to `core::agents::sync`. The command layer only translates persisted `AgentSyncMode` into `target_sync::SyncMode`.

## Current Note

The current branch change for this module is formatting-only; runtime behavior is unchanged.

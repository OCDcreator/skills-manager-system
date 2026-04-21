# Agent Commands

> **Source**: `src-tauri/src/commands/agents.rs`
> **Status**: [REVIEW]

## Overview

Provides thin Tauri commands for agent inventory, target configuration, and manual global apply.

## Public Surface

| Export | Purpose |
|---|---|
| `get_agent_inventory` | Returns the current agent inventory snapshot. |
| `set_agent_enabled` / `set_agent_path_override` / `clear_agent_path_override` | Persist per-agent target settings and return refreshed inventory. |
| `apply_agent_sync` | Runs one manual global sync pass using an explicit mode override or the saved setting. |

## Core Logic

`apply_agent_sync` now resolves sync mode from either the command argument or `settings.json`, so manual apply and scene-apply flows share the same `copy`/`symlink` preference model.

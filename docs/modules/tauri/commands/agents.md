# Agent Commands

> **Source**: `src-tauri/src/commands/agents.rs`
> **Status**: [REVIEW]

## Overview

Provides thin Tauri commands for phase-three agent inventory, target configuration, and manual apply.

## Import Relationships

```text
Upstream: src-tauri/src/lib.rs, src/lib/tauri.ts
Downstream: src-tauri/src/core/agents/*, src-tauri/src/core/settings.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `get_agent_inventory` | Returns the current agent inventory snapshot. |
| `set_agent_enabled` | Persists one agent's enabled flag and returns refreshed inventory. |
| `set_agent_path_override` | Persists one agent path override and returns refreshed inventory. |
| `clear_agent_path_override` | Clears one agent path override and returns refreshed inventory. |
| `apply_agent_sync` | Runs one manual copy-only global sync pass. |

## Core Logic

Resolves the app config directory, loads the repo path when apply is requested, delegates agent operations into the `core/agents` domain, and maps errors into `String` for the frontend.

## Data Flow

Frontend invokes these commands through `src/lib/tauri.ts`; command functions pass normalized inputs into the agents core modules and return serialized DTOs.

## Interactions

Keep business rules in `src-tauri/src/core/agents/`; this module should remain command-layer plumbing only.

## Configuration

Uses the Tauri app config directory and the existing repo-path setting from `settings.json`.

## Change Notes

If more agent actions are added, keep the command layer thin and reuse `AgentSystemDirs::current()` rather than duplicating path logic.

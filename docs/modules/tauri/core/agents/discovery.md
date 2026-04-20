# Agent Discovery

> **Source**: `src-tauri/src/core/agents/discovery.rs`
> **Status**: [REVIEW]

## Overview

Builds the frontend-facing agent inventory snapshot by combining catalog metadata, persisted config, and runtime path detection.

## Import Relationships

```text
Upstream: src-tauri/src/commands/agents.rs, src-tauri/src/core/agents/sync.rs
Downstream: src-tauri/src/core/agents/catalog.rs, src-tauri/src/core/agents/config.rs, dirs
```

## Public Surface

| Export | Purpose |
|---|---|
| `AgentSystemDirs` | Runtime home/config directory roots used for candidate resolution. |
| `AgentPathMode` | Path-state enum exposed to the frontend. |
| `AgentInventoryItem` | One agent target snapshot. |
| `AgentInventorySnapshot` | Full inventory payload returned to the frontend. |
| `load_agent_inventory` | Loads config and returns the current inventory snapshot. |
| `build_agent_inventory` | Pure inventory builder for callers that already have config. |

## Core Logic

Resolves path candidates from catalog rules, handles `.config/...` dual-candidate lookup for OpenCode, determines detected/default/effective paths, and marks each agent as `override`, `detected`, or `missing`.

## Data Flow

Commands call `load_agent_inventory`; sync orchestration also uses the same inventory snapshot to decide which targets can currently apply.

## Interactions

Must stay aligned with the built-in rules in `catalog.rs`, the persisted overrides in `config.rs`, and the frontend DTOs in `src/lib/tauri.ts`.

## Configuration

Uses `dirs` to read the current machine's home/config roots when building production snapshots.

## Change Notes

Missing-path state is normal inventory output, not a command failure.

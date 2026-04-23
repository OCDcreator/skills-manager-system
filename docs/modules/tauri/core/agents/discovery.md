# Agent Discovery

> **Source**: `src-tauri/src/core/agents/discovery.rs`
> **Status**: [REVIEW]

## Overview

Builds the frontend-facing agent inventory snapshot by combining catalog metadata, persisted per-agent config, and runtime path detection.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentSystemDirs` | Runtime home/config directory roots used for candidate resolution. |
| `AgentPathMode` | Path-state enum exposed to the frontend. |
| `AgentInventoryItem` | One agent target snapshot including saved skill/scene/exclusion selections. |
| `AgentInventorySnapshot` | Full inventory payload returned to the frontend. |
| `load_agent_inventory` | Loads config and returns the current inventory snapshot. |
| `build_agent_inventory` | Pure inventory builder for callers that already have config. |

## Core Logic

Resolves path candidates from catalog rules, handles `.config/...` dual-candidate lookup, determines detected/default/effective paths, normalizes emitted path strings, and carries saved direct skills, scene IDs, and exclusion IDs into the inventory payload shown by the editor.

## Interactions

Must stay aligned with the built-in rules in `catalog.rs`, persisted config in `config.rs`, frontend DTOs in `src/lib/tauri.ts`, and sync orchestration in `sync.rs`.

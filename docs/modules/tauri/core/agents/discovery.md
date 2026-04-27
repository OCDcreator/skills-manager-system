# Agent Discovery

> **Source**: `src-tauri/src/core/agents/discovery.rs`
> **Status**: [REVIEW]

## Overview

Builds the frontend-facing agent inventory snapshot by combining catalog metadata, persisted per-agent config, runtime path detection, and read-only target skill inventory.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentSystemDirs` | Runtime home/config directory roots used for candidate resolution. |
| `AgentPathMode` | Path-state enum exposed to the frontend. |
| `AgentInventoryItem` | One agent target snapshot including saved skill/scene/exclusion selections, static catalog rules, and target-directory inventory. |
| `AgentInventorySnapshot` | Full inventory payload returned to the frontend. |
| `load_agent_inventory` | Loads config and returns the current inventory snapshot. |
| `build_agent_inventory` | Pure inventory builder for callers that already have config. |

## Core Logic

Resolves path candidates from catalog rules, handles `.config/...` dual-candidate lookup, determines detected/default/effective paths, normalizes emitted path strings, scans the effective target directory for managed/unmanaged skill entries, and carries saved direct skills, scene IDs, exclusion IDs, plus serialized `skills_dir_rule` / `detect_dir_rule` values into the inventory payload shown by the editor. Hidden home-directory tools such as Kimi Code CLI therefore detect directly from `~/.kimi` and surface `~/.kimi/skills` as both the default and detected sync target when present.

## Interactions

Must stay aligned with the built-in rules in `catalog.rs`, persisted config in `config.rs`, target inventory scanning in `target_inventory.rs`, frontend DTOs in `src/lib/tauri.ts`, and sync orchestration in `sync.rs`.

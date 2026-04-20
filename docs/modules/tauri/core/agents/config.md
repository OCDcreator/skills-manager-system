# Agent Config Store

> **Source**: `src-tauri/src/core/agents/config.rs`
> **Status**: [REVIEW]

## Overview

Persists phase-three agent target configuration separately from `settings.json`.

## Import Relationships

```text
Upstream: src-tauri/src/commands/agents.rs, src-tauri/src/core/agents/discovery.rs, src-tauri/src/core/agents/sync_tests.rs
Downstream: serde_json, std::fs
```

## Public Surface

| Export | Purpose |
|---|---|
| `AgentConfigEntry` | One agent's enabled flag and optional path override. |
| `AgentConfigSnapshot` | Full persisted config map keyed by agent key. |
| `AgentConfigStore` | Loads and updates `agent-config.json`. |

## Core Logic

Validates supported agent keys, reads/writes `agent-config.json`, toggles enabled flags, and stores absolute override paths without mixing repo-path settings into this file.

## Data Flow

Commands mutate this store, then rebuild inventory snapshots from the updated config.

## Interactions

Must stay aligned with the built-in keys from `catalog.rs` and the discovery layer that consumes these persisted overrides.

## Configuration

Writes `agent-config.json` under the app config directory.

## Change Notes

Keep this store limited to target enablement and override paths; do not fold repo-path settings or sync ledger state into it.

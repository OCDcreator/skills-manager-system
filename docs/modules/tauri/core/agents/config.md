# Agent Config Store

> **Source**: `src-tauri/src/core/agents/config.rs`
> **Status**: [REVIEW]

## Overview

Persists per-agent sync configuration separately from `settings.json`.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentConfigEntry` | One agent's enabled flag, optional path override, direct skill IDs, scene IDs, and exclusion IDs. |
| `AgentConfigSnapshot` | Full persisted config map keyed by agent key. |
| `AgentConfigStore` | Loads and updates `agent-config.json`. |

## Core Logic

Validates supported agent keys, reads/writes `agent-config.json`, toggles enabled flags, stores absolute override paths, normalizes ID arrays, and supports whole-entry saves for the agent editor. Path overrides go through `platform_paths` so manually entered Windows and macOS paths persist with `/` separators and no redundant trailing separator.

## Interactions

Commands mutate this store, discovery carries the persisted fields into inventory snapshots, and sync resolves each agent's desired skill set from these fields.

## Change Notes

Keep this store limited to per-agent sync configuration; do not fold repo-path settings or sync ledger state into it.

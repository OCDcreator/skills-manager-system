# Agent Sync Ledger Helpers

> **Source**: `src-tauri/src/core/agents/manifest.rs`
> **Status**: [REVIEW]

## Overview

Owns the app-local ledger that records the last target directory used by global agent sync.

## Import Relationships

```text
Upstream: src-tauri/src/core/agents/sync.rs
Downstream: serde_json, std::fs
```

## Public Surface

| Export | Purpose |
|---|---|
| `AgentSyncLedger` | App-local record of the last applied target directory per agent. |
| `AgentSyncLedgerEntry` | One ledger row for a supported agent. |
| `load_ledger` / `save_ledger` | Reads and writes `agent-sync-ledger.json`. |

## Core Logic

Tracks only global agent sync history. Target-local manifest reconciliation, copy/symlink deployment, and managed-entry cleanup now live in `target_sync.rs` so project-local sync can share those helpers without including unrelated ledger code.

## Data Flow

`sync.rs` loads this ledger before applying targets and saves it after each global sync pass.

## Interactions

Must stay aligned with target-path decisions assembled in `discovery.rs`.

## Configuration

Writes `agent-sync-ledger.json` under the app config directory.

## Change Notes

Keep this module ledger-focused; target reconciliation belongs in `target_sync.rs`.

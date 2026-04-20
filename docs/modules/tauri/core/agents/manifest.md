# Agent Sync Manifest Helpers

> **Source**: `src-tauri/src/core/agents/manifest.rs`
> **Status**: [REVIEW]

## Overview

Owns the low-level manifest, ledger, and copy-only target-reconciliation helpers used by agent sync.

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
| `DesiredSkillEntry` | Internal desired-output description built from scan results. |
| `TargetApplyStats` | Internal counts for one target apply pass. |
| `build_desired_skill_entries` | Maps enabled skills to stable managed target names. |
| `apply_desired_entries` | Reconciles one target directory using the managed manifest. |
| `cleanup_managed_entries` | Removes only app-managed entries from a target directory. |
| `load_ledger` / `save_ledger` | Reads and writes `agent-sync-ledger.json`. |

## Core Logic

Tracks a target-local manifest and an app-local ledger, copies directories recursively in copy mode, skips `.git`, removes stale managed entries, and protects unmanaged content from deletion or overwrite.

## Data Flow

`sync.rs` builds desired entries from enabled skills, then delegates target reconciliation and ledger persistence into this module.

## Interactions

Must stay aligned with stable skill IDs from `src-tauri/src/core/skills/scan.rs` and with the target-path decisions assembled in `discovery.rs`.

## Configuration

Writes `.skills-manager-system-manifest.json` inside managed target directories and `agent-sync-ledger.json` under the app config directory.

## Change Notes

Keep these helpers copy-only in phase three; symlink behavior belongs in a future expansion.

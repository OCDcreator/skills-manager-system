# Agent Sync Orchestration

> **Source**: `src-tauri/src/core/agents/sync.rs`
> **Status**: [REVIEW]

## Overview

Implements the high-level manual apply workflow for per-agent sync.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentApplyStatus` | Per-agent result status enum returned to the frontend. |
| `AgentApplyResult` | Per-agent manual apply summary. |
| `ApplyAgentSyncResponse` | Aggregate apply response including globally available skill count. |
| `apply_agent_sync` | Runs one manual apply pass across the supported agents with configurable copy/symlink mode. |
| `apply_agent_sync_for_agent` | Runs one manual apply pass scoped to a single agent. |

## Core Logic

Loads inventory, builds a skill-selection context from scan results plus global disabled IDs and saved scenes, resolves each agent's desired skills as `direct ∪ scenes - exclusions`, reconciles old/current target directories, removes managed entries for disabled targets, and returns per-agent result summaries without deleting unmanaged content.

## Interactions

Depends on discovery for target paths, scene config for reusable skill groups, skills scan/state for global hard-disable truth, `manifest.rs` for ledger persistence, and `target_sync.rs` for safe target reconciliation.

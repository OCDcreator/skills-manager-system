# Agent Sync Orchestration

> **Source**: `src-tauri/src/core/agents/sync.rs`
> **Status**: [REVIEW]

## Overview

Implements the high-level manual apply workflow for global agent sync.

## Import Relationships

```text
Upstream: src-tauri/src/commands/agents.rs, src-tauri/src/core/agents/sync_tests.rs
Downstream: src-tauri/src/core/agents/discovery.rs, src-tauri/src/core/agents/manifest.rs, src-tauri/src/core/skills/scan.rs, src-tauri/src/core/skills/state.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `AgentApplyStatus` | Per-agent result status enum returned to the frontend. |
| `AgentApplyResult` | Per-agent manual apply summary. |
| `ApplyAgentSyncResponse` | Aggregate apply response including enabled-skill count. |
| `apply_agent_sync` | Runs one manual copy-only apply pass across the supported agents. |

## Core Logic

Loads the current inventory snapshot, computes enabled skills from scan results minus disabled IDs, reconciles old/current target directories per agent, removes managed entries for disabled targets, and returns a per-agent result summary without deleting unmanaged content.

## Data Flow

Commands pass the config dir, repo path, and runtime system dirs into this module. The module returns frontend-ready apply summaries.

## Interactions

Depends on discovery for target paths, skills scan/state for enabled-skill truth, and manifest helpers for safe target reconciliation.

## Configuration

None directly beyond the config-dir files owned by sibling modules.

## Change Notes

Keep this file focused on orchestration; lower-level manifest/ledger mechanics belong in `manifest.rs`.

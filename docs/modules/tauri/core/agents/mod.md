# Agents Core Module Boundary

> **Source**: `src-tauri/src/core/agents/mod.rs`
> **Status**: [REVIEW]

## Overview

Declares the Rust backend subdomain for phase-three agent sync.

## Import Relationships

```text
Upstream: src-tauri/src/commands/agents.rs, src-tauri/src/core/agents/sync_tests.rs
Downstream: src-tauri/src/core/agents/catalog.rs, src-tauri/src/core/agents/config.rs, src-tauri/src/core/agents/discovery.rs, src-tauri/src/core/agents/manifest.rs, src-tauri/src/core/agents/selection.rs, src-tauri/src/core/agents/sync.rs, src-tauri/src/core/agents/target_inventory.rs, src-tauri/src/core/agents/target_sync.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `catalog` | Built-in supported-agent definitions. |
| `config` | Persistent enabled/override config. |
| `discovery` | Runtime target-path inventory assembly. |
| `manifest` | App-local global sync ledger helpers. |
| `selection` | Per-agent direct/scene/exclusion skill resolution. |
| `sync` | Manual apply orchestration and public sync DTOs. |
| `target_inventory` | Read-only scanner for existing target-directory skills. |
| `target_sync` | Shared target-directory reconciliation and copy/symlink deployment helpers. |

## Core Logic

This module is aggregation only. It exposes the agent domain to commands and keeps helper internals such as ledger handling and target reconciliation inside the domain boundary.

## Data Flow

Not applicable.

## Interactions

Add new agent-domain modules here before command or test code can import them.

## Configuration

None.

## Change Notes

Keep this file small and declarative; business logic belongs in the leaf modules.

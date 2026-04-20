# Agents Core Module Boundary

> **Source**: `src-tauri/src/core/agents/mod.rs`
> **Status**: [REVIEW]

## Overview

Declares the Rust backend subdomain for phase-three agent sync.

## Import Relationships

```text
Upstream: src-tauri/src/commands/agents.rs, src-tauri/src/core/agents/sync_tests.rs
Downstream: src-tauri/src/core/agents/catalog.rs, src-tauri/src/core/agents/config.rs, src-tauri/src/core/agents/discovery.rs, src-tauri/src/core/agents/manifest.rs, src-tauri/src/core/agents/sync.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `catalog` | Built-in supported-agent definitions. |
| `config` | Persistent enabled/override config. |
| `discovery` | Runtime target-path inventory assembly. |
| `sync` | Manual apply orchestration and public sync DTOs. |

## Core Logic

This module is aggregation only. It exposes the agent domain to commands and keeps helper internals such as manifest handling inside the domain boundary.

## Data Flow

Not applicable.

## Interactions

Add new agent-domain modules here before command or test code can import them.

## Configuration

None.

## Change Notes

Keep this file small and declarative; business logic belongs in the leaf modules.

# CLI Agent Commands

> **Source**: `src-tauri/src/cli/commands/agents.rs`
> **Status**: [REVIEW]

## Overview

Implements agent inventory inspection and agent configuration mutations.

## Import Relationships

```text
Upstream: src-tauri/src/cli/mod.rs
Downstream: src-tauri/src/app_runtime/*, src-tauri/src/core/agents/discovery.rs, src-tauri/src/core/agents/config.rs, src-tauri/src/cli/commands/agent_sync.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `run` | Dispatches the parsed agent subcommand. |

## Core Logic

The module resolves host directories through `AgentSystemDirs`, loads the shared agent config snapshot, and returns normalized inventory for `agents list`. Enable/disable/path mutations validate known agent keys, acquire the advisory config lock, and write through `AgentConfigStore`; `agents sync` delegates sync-specific behavior to `agent_sync.rs`.

## Interactions

Inventory shape must stay aligned with `src-tauri/src/core/agents/discovery.rs`, and mutation payloads must stay compatible with `AgentConfigStore` and the stable CLI JSON schema.

# CLI Agent Commands

> **Source**: `src-tauri/src/cli/commands/agents.rs`
> **Status**: [REVIEW]

## Overview

Implements the read-only `agents list` command for headless inventory inspection.

## Import Relationships

```text
Upstream: src-tauri/src/cli/mod.rs
Downstream: src-tauri/src/app_runtime/*, src-tauri/src/core/agents/discovery.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `run` | Dispatches the parsed agent subcommand. |

## Core Logic

The module resolves host directories through `AgentSystemDirs`, loads the shared agent config snapshot, and returns the normalized inventory payload through the shared CLI response builder.

## Interactions

Inventory shape must stay aligned with `src-tauri/src/core/agents/discovery.rs` and any future CLI mutation commands should continue to reuse the same runtime/meta helpers.

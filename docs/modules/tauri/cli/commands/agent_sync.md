# CLI Agent Sync Adapter

> **Source**: `src-tauri/src/cli/commands/agent_sync.rs`
> **Status**: [REVIEW]

## Overview

Handles the `agents sync` mutation path and keeps sync-specific error and partial-result mapping out of the inventory adapter.

## Import Relationships

```text
Upstream: src-tauri/src/cli/commands/agents.rs, cli command tests
Downstream: src-tauri/src/app_runtime/*, src-tauri/src/core/agents/sync.rs, src-tauri/src/core/settings.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `run` | Resolves repo, sync mode, host dirs, lock, and dispatches `agents sync`. |
| `sync_with_system_dirs` | Testable adapter entrypoint with injected system dirs. |
| `sync_failure_error` | Maps sync failures to stable CLI error codes. |

## Core Logic

Mutation is protected by the advisory config lock. Conflict-bearing sync results become `partial` responses with exit code `8`, while failed agent results become structured errors; Windows Developer Mode symlink failures map to `windows_symlink_privilege_required`.

## Interactions

Must stay aligned with `ApplyAgentSyncResponse`, `AgentApplyStatus`, and the approved CLI output schema.


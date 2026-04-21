# CLI Agent Command Tests

> **Source**: `src-tauri/src/cli/commands/agents_tests.rs`
> **Status**: [REVIEW]

## Overview

Validates agent mutation payloads, partial sync behavior, and Windows symlink error mapping.

## Import Relationships

```text
Upstream: cargo test --features cli
Downstream: src-tauri/src/cli/commands/agents.rs, src-tauri/src/cli/commands/agent_sync.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| tests | Focused coverage for `agents enable`, `agents set-path`, `agents sync`, and symlink privilege mapping. |

## Core Logic

Tests use temp config/repo/target directories and injected `AgentSystemDirs` so they do not depend on the developer machine's installed agents.

## Interactions

Keep assertions aligned with stable CLI status values and error codes.


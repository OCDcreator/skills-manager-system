# App Runtime Config Lock

> **Source**: `src-tauri/src/app_runtime/config_lock.rs`
> **Status**: [REVIEW]

## Overview

Owns the cross-platform advisory lock used to serialize future desktop and CLI config mutations.

## Import Relationships

```text
Upstream: src-tauri/src/app_runtime/context.rs, src-tauri/src/app_runtime/output_errors.rs
Downstream: fs2, std::fs
```

## Public Surface

| Export | Purpose |
|---|---|
| `ConfigLockErrorKind` | Classifies lock conflicts separately from lock I/O failures. |
| `ConfigLockError` | Carries the lock path and optional source error for stable CLI mapping. |
| `ConfigLockGuard` | Holds the exclusive advisory lock until dropped. |
| `acquire_config_lock` | Opens the config-root lock file and attempts a non-blocking exclusive lock. |

## Core Logic

The lock file lives at `.skills-manager-system.lock` under the active config directory. Acquisition creates the config root if needed, opens or creates the lock file, attempts an exclusive advisory lock, and writes the current process id after the lock is held. The file remains on disk after release so stale lock files do not block future writers.

## Interactions

`AppRuntimeContext` exposes this helper to command adapters through `acquire_config_lock` and `with_config_lock`. `CliCommandError::from_config_lock` maps conflict and filesystem failures into stable machine-readable error codes.

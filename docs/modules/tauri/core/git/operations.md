# Git Operations Core

> **Source**: `src-tauri/src/core/git/operations.rs`
> **Status**: [DRAFT]

## Overview

Provides git CLI operations for the Skills Manager repository: status, diff, log, pull/push/commit/fetch, and external sync script execution.

## Import Relationships

```text
Upstream: serde, anyhow, std::process
Downstream: commands::git
```

## Public Surface

| Export | Purpose |
|---|---|
| `git_status` | Parse porcelain v2 status into structured response |
| `git_diff` | Get staged or unstaged diff text |
| `git_log` | Recent commit log entries |
| `git_pull` / `git_push` / `git_commit` / `git_fetch` | Remote and local operations |
| `run_sync_script` | Run update.sh / update.bat |
| `GitStatusResponse`, `GitDiffResponse`, `GitLogResponse`, `GitOperationResult` | Serde response types |

## Core Logic

All operations invoke the system `git` binary via `std::process::Command` with `GIT_TERMINAL_PROMPT=0` and `LC_ALL=C`. The status parser handles porcelain v2 format lines (`1 `, `2 `, `u `, `? `) and branch metadata (`# branch.head`, `# branch.ab`).

## Data Flow

1. Command layer passes `repo_path` from app settings
2. Operations construct and execute git CLI commands
3. Output is parsed into typed Rust structs
4. Results are serialized via serde to the frontend

## Interactions

- `commands::git` — thin wrappers calling these functions
- `core::settings` — provides repo_path
- Git CLI must be available on the system PATH

## Configuration

None.

## Change Notes

- Porcelain v2 parsing depends on git output format stability
- Windows sync script uses `update.bat`, Unix uses `update.sh`
- `git_commit` stages all changes before committing (`git add -A`)

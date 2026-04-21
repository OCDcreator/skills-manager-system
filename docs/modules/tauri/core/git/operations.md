# Git Operations Core

> **Source**: `src-tauri/src/core/git/operations.rs`
> **Status**: [DRAFT]

## Overview

Provides git CLI operations for the Skills Manager repository: status, diff, log, pull/push/commit/fetch, and external sync script execution.

## Import Relationships

```text
Upstream: anyhow, std::process, core::git::types
Downstream: commands::git, core::git::operations_test
```

## Public Surface

| Export | Purpose |
|---|---|
| `git_status` | Parse porcelain v2 status into structured response |
| `git_diff` | Get staged or unstaged diff text |
| `git_log` | Recent commit log entries |
| `git_pull` / `git_push` / `git_commit` / `git_fetch` | Remote and local operations |
| `run_sync_script` | Run update.sh / update.bat |
| response DTOs | Imported from `core::git::types` |

## Core Logic

All operations invoke the system `git` binary via `std::process::Command` with `GIT_TERMINAL_PROMPT=0` and `LC_ALL=C`. The status parser handles porcelain v2 format lines (`1 `, `2 `, `u `, `? `) and branch metadata (`# branch.head`, `# branch.ab`). Operation helpers now retain stdout, stderr, and exit status alongside the existing display message so CLI adapters can expose structured error details. Tests live in `operations_test.rs` so this module remains focused on runtime behavior.

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

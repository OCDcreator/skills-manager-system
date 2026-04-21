# Git Commands

> **Source**: `src-tauri/src/commands/git.rs`
> **Status**: [DRAFT]

## Overview

Thin Tauri command layer exposing git operations to the frontend. Each command loads the repo path from settings, delegates behavior to `core::git::operations`, and returns DTOs from `core::git::types`.

## Import Relationships

```text
Upstream: core::git::operations, core::git::types, core::settings
Downstream: Frontend via Tauri invoke
```

## Public Surface

| Command | Purpose |
|---|---|
| `git_status` | Repository status |
| `git_diff` | Staged or unstaged diff |
| `git_log` | Commit history |
| `git_pull` / `git_push` / `git_commit` / `git_fetch` | Remote/local operations |
| `run_sync_script` | Execute update script |

## Core Logic

All commands follow the same pattern: load repo path from settings, call the corresponding `core::git::operations` function, map errors to `String`.

## Data Flow

Frontend invoke → command → settings (repo_path) → core git operation → serialized response.

## Interactions

- Registered in `lib.rs` via `generate_handler![]`
- `src/lib/git.ts` — matching TypeScript invoke wrappers

## Configuration

None.

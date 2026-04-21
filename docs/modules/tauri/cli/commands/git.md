# CLI Git Commands

> **Source**: `src-tauri/src/cli/commands/git.rs`
> **Status**: [REVIEW]

## Overview

Implements headless git status, diff, log, fetch, pull, push, commit, and external sync-script commands.

## Import Relationships

```text
Upstream: src-tauri/src/cli/mod.rs
Downstream: src-tauri/src/app_runtime/*, src-tauri/src/core/git/*
```

## Public Surface

| Export | Purpose |
|---|---|
| `run` | Dispatches one parsed git subcommand to the core git operation. |

## Core Logic

Read commands serialize typed core responses. Operation commands return success payloads when git succeeds and structured `git_command_failed` errors that include operation, stdout, stderr, and exit code when git fails.

## Interactions

Requires an effective repo path. Error details depend on `GitOperationResult` retaining stdout/stderr/status from `core/git/operations.rs`.


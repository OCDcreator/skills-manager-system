# Git Response Types

> **Source**: `src-tauri/src/core/git/types.rs`
> **Status**: [REVIEW]

## Overview

Defines serde response structs shared by git command wrappers and git operation helpers.

## Import Relationships

```text
Upstream: src-tauri/src/commands/git.rs, src-tauri/src/core/git/operations.rs
Downstream: serde
```

## Public Surface

| Export | Purpose |
|---|---|
| `GitStatusEntry` | One staged, unstaged, or untracked status row. |
| `GitStatusResponse` | Branch, remote, ahead/behind, and grouped status rows. |
| `GitDiffResponse` | Diff text plus stat summary. |
| `GitLogEntry` / `GitLogResponse` | Recent commit history payloads. |
| `GitOperationResult` | Success/message wrapper for mutating git operations. |

## Core Logic

No business logic lives here. Keeping DTOs separate lets `operations.rs` stay focused on command execution and parsing while command modules can import response types without depending on helper internals.

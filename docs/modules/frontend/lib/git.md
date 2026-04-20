# Git API Types

> **Source**: `src/lib/git.ts`
> **Status**: [DRAFT]

## Overview

TypeScript type definitions and Tauri invoke wrappers for the git sync backend.

## Public Surface

| Export | Purpose |
|---|---|
| `GitStatusEntry`, `GitStatusResponse` | Status types |
| `GitDiffResponse` | Diff output |
| `GitLogEntry`, `GitLogResponse` | Log types |
| `GitOperationResult` | Generic operation result |
| `gitStatus`, `gitDiff`, `gitLog`, etc. | Invoke wrappers |

## Export Semantics

Pure type + invoke module. No state, no side effects beyond Tauri IPC calls.

## Interactions

- `src/views/GitView.tsx` — primary consumer
- `src-tauri/src/commands/git.rs` — backend counterparts

# GitActions Component

> **Source**: `src/components/git/GitActions.tsx`
> **Status**: [DRAFT]

## Overview

Action bar for git operations: pull, push, commit (with inline message input), and sync external sources.

## Public Surface

| Export | Purpose |
|---|---|
| `GitActions` | Git action buttons component |

## Core Logic

Manages local state for each operation's loading state and last result. Commit has an inline text input with Enter-key support. Displays operation result as a success/error banner.

## Data Flow

Calls `lib/git` API functions directly. Emits `onOperationComplete` to trigger parent data refresh.

## Interactions

- `lib/git.ts` — `gitPull`, `gitPush`, `gitCommit`, `runSyncScript`
- `i18n` keys: `git.actions.*`, `git.operation.running`
- `lucide-react` icons: ArrowDownToLine, ArrowUpFromLine, GitCommitHorizontal, RefreshCw, Upload

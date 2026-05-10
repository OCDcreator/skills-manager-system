# Git View

> **Source**: `src/views/GitView.tsx`
> **Status**: [REVIEW]

## Overview

Page-level Git workbench for repository status, file selection, diff inspection, actions, operation feedback, and recent commit history.

## Import Relationships

`App.tsx` routes here; the view composes `GitStatusBar`, `GitFileList`, `GitDiffViewer`, `GitActions`, `GitOperationLog`, and `GitLogList` on top of `AppContext` plus `src/lib/git.ts`.

## Public Surface

| Export | Purpose |
|---|---|
| `GitView` | Main git sync page component |

## Core Logic

The view owns local status, log, diff, loading, selected-path, diff-mode, and operation-log state. It defers the first status/log refresh with `queueMicrotask()` after `repoPath` becomes available, treats fetch failures as operation-log entries, and keeps the unconfigured dashed panel when no repo path exists. File selection derives the initial diff mode from the status entry (`staged` when `x` carries a real index change, otherwise `unstaged`) and then asks `GitDiffViewer` to render that mode.

The page does not have a separate `900px-1279px` Git-specific drawer or rail. Its compact decision is simpler: the file list and diff viewer stay stacked all the way until the `xl` split activates, so both `<900px` and `900px-1279px` remain in the same single-column workbench flow. Only at `xl` does the page switch to the `280px + 1fr` two-column inspection layout.

## Data Flow

1. When `repoPath` exists, refresh `gitStatus()` and `gitLog(20)`.
2. Selecting a file stores `selectedPath`, derives a default diff mode, and fetches `gitDiff(...)`.
3. Refresh and action callbacks reload status/log; fetch errors prepend a failed log row.

## Interactions

- Keep Git API calls in `src/lib/git.ts`; this view is orchestration only.
- Keep scrollable panel behavior in `GitFileList` and `GitDiffViewer`, not here.
- Stay aligned with `git.*` and `tooltip.git.*` i18n keys.

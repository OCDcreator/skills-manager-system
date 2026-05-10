# GitDiffViewer Component

> **Source**: `src/components/git/GitDiffViewer.tsx`
> **Status**: [REVIEW]

## Overview

Displays the selected Git diff with staged/unstaged mode toggles, a copy action, stat metadata, and inline diff coloring.

## Public Surface

| Export | Purpose |
|---|---|
| `GitDiffViewer` | Diff display component |

## Core Logic

The component has three states: loading shell, no-selection placeholder, and full diff viewer. In the full state it keeps separate remembered scroll positions for `staged` and `unstaged` diffs, renders an optional `stat` line above the body, and colors diff lines by prefix (`@@`, `+`, `-`, `+++`, `---`). Copy feedback is local-only through `navigator.clipboard`, with a two-second checkmark state.

Like `GitFileList`, this component does not create a special `900px-1279px` layout. It stays width-safe and vertically scrollable inside GitView's compact stacked workbench below `xl`, then fills the right column when the wide split appears.

## Data Flow

`GitView` owns diff fetching and mode changes; this component only renders the current payload and emits `onModeChange`.

## Interactions

- `src/lib/git.ts` supplies `GitDiffResponse`.
- `git.diff.*` and `tooltip.git.diff.*` supply labels and button titles.

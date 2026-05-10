# GitFileList Component

> **Source**: `src/components/git/GitFileList.tsx`
> **Status**: [REVIEW]

## Overview

Displays staged, unstaged, and untracked files in the left inspection panel of the Git workbench.

## Public Surface

| Export | Purpose |
|---|---|
| `GitFileList` | File change list component |

## Core Logic

The component is props-driven and renders three reusable `Section` blocks with color-coded status markers plus file-path tooltips. It keeps the panel width-safe with `min-w-0` and truncates long paths inside each row so compact workbench widths do not force horizontal overflow. The scroll surface uses `skill-markdown-scroll`, remembers its position through `useRememberedScrollPosition("git:file-list")`, and adds a large-screen height cap so long file lists scroll inside the panel instead of stretching the page.

This component does not add a dedicated `900px-1279px` mode. It simply behaves well inside the page's stacked compact shell below `xl`, then remains the fixed-width left column once `GitView` enables the wide split.

## Data Flow

Receives grouped status entries plus the selected path from `GitView`, then emits `onSelect(entry)` for diff loading.

## Interactions

- `src/lib/git.ts` supplies `GitStatusEntry`.
- `git.files.*` and `tooltip.git.file.*` provide labels and row titles.

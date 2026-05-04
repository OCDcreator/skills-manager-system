# GitDiffViewer Component

> **Source**: `src/components/git/GitDiffViewer.tsx`
> **Status**: [DRAFT]

## Overview

Displays git diff content with syntax coloring, staged/unstaged mode toggle, and copy button. The diff panel is width-shrinkable, caps its large-screen height so long diffs scroll inside the panel instead of stretching the git page, reuses the skill reading pane scrollbar styling, and remembers separate scroll positions for staged and unstaged modes.

## Public Surface

| Export | Purpose |
|---|---|
| `GitDiffViewer` | Diff display component |

## Core Logic

Renders diff with line-level coloring: additions (emerald), deletions (rose), headers (sky), metadata (slate). Supports staged/unstaged mode toggle and clipboard copy via `navigator.clipboard`.

## Data Flow

Props-driven. Parent switches between staged/unstaged via `onModeChange`.

## Interactions

- `lib/git.ts` — `GitDiffResponse` type
- `i18n` keys: `git.diff.*`

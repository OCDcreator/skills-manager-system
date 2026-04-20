# GitFileList Component

> **Source**: `src/components/git/GitFileList.tsx`
> **Status**: [DRAFT]

## Overview

Displays repository file changes grouped into staged, unstaged, and untracked sections.

## Public Surface

| Export | Purpose |
|---|---|
| `GitFileList` | File change list component |

## Core Logic

Renders three sections (staged, unstaged, untracked) with color-coded status codes. Each file entry is clickable to trigger diff viewing. Uses `Section` sub-component for each group.

## Data Flow

Props-driven. Emits `onSelect` with the clicked `GitStatusEntry`.

## Interactions

- `lib/git.ts` — `GitStatusEntry` type
- `i18n` keys: `git.files.*`
- `lucide-react` icons: FileText

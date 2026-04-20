# GitStatusBar Component

> **Source**: `src/components/git/GitStatusBar.tsx`
> **Status**: [DRAFT]

## Overview

Displays repository branch, remote URL, ahead/behind counts, clean/dirty badge, and fetch/refresh buttons.

## Public Surface

| Export | Purpose |
|---|---|
| `GitStatusBar` | Status summary card component |

## Core Logic

Reads `GitStatusResponse` from props. Shows branch name, remote URL, ahead/behind indicators, and a clean/dirty status badge. Provides fetch and refresh action buttons.

## Data Flow

Props-driven, no internal state. Parent triggers `onRefresh` / `onFetch`.

## Interactions

- `lib/git.ts` — `GitStatusResponse` type
- `i18n` keys: `git.status.*`, `git.actions.fetch`, `git.operation.running`
- `lucide-react` icons: GitBranch, Globe, RefreshCw, ArrowDownToLine, ArrowUpFromLine

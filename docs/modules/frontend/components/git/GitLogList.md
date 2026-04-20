# GitLogList Component

> **Source**: `src/components/git/GitLogList.tsx`
> **Status**: [DRAFT]

## Overview

Displays recent commit history as a compact list with hash, message, author, and relative date.

## Public Surface

| Export | Purpose |
|---|---|
| `GitLogList` | Commit log list component |

## Core Logic

Renders a list of `GitLogEntry` items. Includes a `formatDate` helper for relative date display (today, yesterday, Xd ago).

## Data Flow

Props-driven. No internal state beyond rendering.

## Interactions

- `lib/git.ts` — `GitLogEntry` type
- `i18n` keys: `git.log.*`, `git.operation.running`
- `lucide-react` icons: GitCommit

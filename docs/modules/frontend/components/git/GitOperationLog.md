# Git Operation Log

> **Source**: `src/components/git/GitOperationLog.tsx`
> **Status**: [DRAFT]

## Overview

Displays a scrollable, reverse-chronological list of git operations performed during the current session (push, pull, commit, sync external). Each entry shows success/failure status, operation label, timestamp, and result message.

## Props

| Prop | Type | Purpose |
|---|---|---|
| entries | OperationLogEntry[] | Session operation history |

## Interactions

- Used by `GitView.tsx` to render the operation log panel
- Entries are added via `onLogEntry` callback passed to `GitActions`

# Git View

> **Source**: `src/views/GitView.tsx`
> **Status**: [DRAFT]

## Overview

Page-level view for git repository sync operations. Composes status bar, file list, diff viewer, action buttons, and commit log.

## Import Relationships

```text
Upstream: context/AppContext, lib/git, components/git/*
Downstream: App.tsx (routing)
```

## Public Surface

| Export | Purpose |
|---|---|
| `GitView` | Main git sync page component |

## Core Logic

Manages local state for git status, log, diff, and loading flags. Initial status/log refresh is deferred through a microtask once `repoPath` is available, fetch failures are recorded into the operation log, and the view still shows an unconfigured state when no repo path exists.

## Data Flow

1. On mount: fetch `gitStatus()` and `gitLog(20)`
2. User selects file → fetch `gitDiff(staged|unstaged)` for that file
3. User clicks action (pull/push/commit/sync) → call API, then refresh status + log

## Interactions

- `context/AppContext` — reads `repoPath`
- `lib/git` — all API calls
- `components/git/*` — all sub-components
- `i18n` keys: `git.*`

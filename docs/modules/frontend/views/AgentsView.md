# AgentsView

> **Source**: `src/views/AgentsView.tsx`
> **Status**: [REVIEW]

## Overview

Assembles the global agent-sync page and owns the page-local sync-mode preference UI.

## Import Relationships

```text
Upstream: src/App.tsx
Downstream: src/context/AppContext.tsx, src/lib/tauri.ts, src/components/agents/*
```

## Public Surface

| Export | Purpose |
|---|---|
| `AgentsView` | Page-level orchestration for global agent sync and mode selection. |

## Core Logic

The view reads inventory and apply state from `AppContext`, loads the persisted sync mode from Tauri settings, lets the user switch between `copy` and `symlink`, and passes the selected mode into `applyAgentSync`.

## Data Flow

Context state provides repo/skill/agent counts; `src/lib/tauri.ts` provides the sync-mode setting read/write path; child components only receive normalized props and callbacks.

## Interactions

Must stay aligned with `src/components/agents/AgentSyncSummary.tsx`, `src/components/agents/AgentTargetCard.tsx`, and the settings/agent command wrappers in `src/lib/tauri.ts`.

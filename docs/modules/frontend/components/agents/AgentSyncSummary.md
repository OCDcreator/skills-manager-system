# AgentSyncSummary

> **Source**: `src/components/agents/AgentSyncSummary.tsx`
> **Status**: [REVIEW]

## Overview

Renders the top summary card for global agent sync, including repo counts, sync-mode selection, and the manual apply action.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentSyncSummary` | Summary UI for repo/skill/target counts, sync mode, and apply action. |

## Core Logic

Displays the configured repository path, enabled-skill count, enabled-target count, a `copy/symlink` mode selector, and context-aware helper text that explains when apply is blocked or which mode manual/scene sync will use.

## Data Flow

Receives precomputed counts, the active sync mode, and callbacks from `AgentsView`; it does not own any filesystem or agent business logic.

## Interactions

Depends on `agents.*` i18n keys and should stay aligned with the `applyAgentSync` and sync-mode setting behavior exposed by `src/views/AgentsView.tsx`.

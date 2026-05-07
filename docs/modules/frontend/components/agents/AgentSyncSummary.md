# AgentSyncSummary

> **Source**: `src/components/agents/AgentSyncSummary.tsx`
> **Status**: [REVIEW]

## Overview

Renders the top summary card for global agent sync, including repo counts, sync-mode selection, manual apply action, and the fixed-slot unsaved-draft controls.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentSyncSummary` | Summary UI for repo/skill/target counts, sync mode, and apply action. |

## Core Logic

Displays the configured repository path, enabled-skill count, enabled-target count, a `copy/symlink` mode selector, and a bottom status slot that always keeps the card height stable. When drafts are clean the slot shows the normal apply-mode helper text; when drafts are dirty it swaps in the unsaved summary plus discard/save-all actions without requiring `AgentsView` to inject a separate banner above the targets list.

## Data Flow

Receives precomputed counts, dirty-draft state, and action callbacks from `AgentsView`; it does not own any filesystem or agent business logic.

## Interactions

Depends on `agents.*` i18n keys and should stay aligned with the `applyAgentSync` and sync-mode setting behavior exposed by `src/views/AgentsView.tsx`.

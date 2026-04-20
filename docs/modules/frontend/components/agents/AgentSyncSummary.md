# AgentSyncSummary

> **Source**: `src/components/agents/AgentSyncSummary.tsx`
> **Status**: [REVIEW]

## Overview

Renders the top summary card for phase-three agent sync, including repo summary, enabled counts, and the manual apply action.

## Import Relationships

```text
Upstream: src/views/AgentsView.tsx
Downstream: src/i18n/index.ts
```

## Public Surface

| Export | Purpose |
|---|---|
| `AgentSyncSummary` | Summary UI for repo/skill/target counts and the apply button. |

## Core Logic

Displays the configured repository path, enabled-skill count, enabled-target count, and copy-only/manual-apply messaging. The component also decides which short disable hint to show when apply is unavailable.

## Data Flow

Receives precomputed counts and the `onApply` callback from `AgentsView`.

## Interactions

Depends on `agents.*` i18n keys and should stay aligned with the apply behavior exposed by `AppContext`.

## Configuration

Styling is local Tailwind-only UI state.

## Change Notes

Do not move inventory scanning or target-path logic into this component.

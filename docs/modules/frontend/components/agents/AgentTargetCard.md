# AgentTargetCard

> **Source**: `src/components/agents/AgentTargetCard.tsx`
> **Status**: [REVIEW]

## Overview

Renders one supported agent target, including enable/disable state, path status, default/detected/effective path display, and override editing controls.

## Import Relationships

```text
Upstream: src/views/AgentsView.tsx
Downstream: src/lib/tauri.ts, src/i18n/index.ts
```

## Public Surface

| Export | Purpose |
|---|---|
| `AgentTargetCard` | Per-agent UI for target state and path override editing. |

## Core Logic

Maintains a nullable local override draft so the saved backend value remains the source of truth until the user edits. Successful save/reset actions clear the local draft and let the refreshed inventory snapshot repopulate the visible value without an effect-driven mirror.

## Data Flow

`AgentInventoryItem` data flows in from `AgentsView`; user actions flow back out through the callback props.

## Interactions

Must stay aligned with the `AgentInventoryItem` shape in `src/lib/tauri.ts` and the agent mutation actions in `src/context/AppContext.tsx`.

## Configuration

The override input uses the resolved default path as its placeholder.

## Change Notes

Keep validation and persistence rules in Rust/`AppContext`; this component should stay as form UI and local input state only.

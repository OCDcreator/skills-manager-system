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

Maintains a local override input string, syncs that input when the persisted override changes, renders status badges from the backend inventory snapshot, and forwards toggle/save/reset actions to the callbacks supplied by the page.

## Data Flow

`AgentInventoryItem` data flows in from `AgentsView`; user actions flow back out through the callback props.

## Interactions

Must stay aligned with the `AgentInventoryItem` shape in `src/lib/tauri.ts` and the agent mutation actions in `src/context/AppContext.tsx`.

## Configuration

The override input uses the resolved default path as its placeholder.

## Change Notes

Keep validation and persistence rules in Rust/`AppContext`; this component should stay as form UI and local input state only.

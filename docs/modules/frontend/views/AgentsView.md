# AgentsView

> **Source**: `src/views/AgentsView.tsx`
> **Status**: [REVIEW]

## Overview

Assembles the phase-three agent-sync page from summary, target-card, and apply-result components.

## Import Relationships

```text
Upstream: src/App.tsx
Downstream: src/context/AppContext.tsx, src/components/agents/*
```

## Public Surface

| Export | Purpose |
|---|---|
| `AgentsView` | Page-level orchestration for global agent sync. |

## Core Logic

The view reads agent inventory, repo path, enabled-skill state, and apply actions from `AppContext`, derives enabled skill/target counts, and passes those normalized values into dedicated agent UI components.

## Data Flow

Context state flows into the view, which computes small page-level summaries and passes callbacks into the agent components.

## Interactions

Must stay aligned with `AppView` routing in `src/App.tsx`, agent DTOs in `src/lib/tauri.ts`, and the action surface exposed by `src/context/AppContext.tsx`.

## Configuration

None.

## Change Notes

Keep business rules and filesystem behavior out of this file; it should remain page composition only.

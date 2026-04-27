# AgentOrderModal

> **Source**: `src/components/agents/AgentOrderModal.tsx`
> **Status**: [REVIEW]

## Overview

Provides the global agent-ordering dialog used from the Agent Sync floating navigation rail.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentOrderModal` | Modal for drag-reordering the persisted global agent list while preserving enabled-first grouping. |

## Core Logic

Builds a local draft order from the persisted order plus any newly discovered agent keys, resolves enabled and disabled sections through the shared `agent-order` helper, and updates the draft with native HTML5 drag-and-drop before delegating save back to the page owner.

## Interactions

Must stay aligned with `src/lib/agent-order.ts`, `AgentBrandIcon`, and the `agents.orderModal.*` i18n keys.

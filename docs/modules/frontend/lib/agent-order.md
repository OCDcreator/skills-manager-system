# Agent Order Helper

> **Source**: `src/lib/agent-order.ts`
> **Status**: [REVIEW]

## Overview

Defines pure helper functions for merging persisted agent-order keys with the current inventory, resolving enabled-first sorted agent lists, and applying drag-reorder mutations.

## Public Surface

| Export | Purpose |
|---|---|
| `resolveSortedAgentInventory` | Sorts any agent list so enabled entries appear first, then applies persisted relative order within each enabled/disabled group. |
| `applyDraggedAgentOrder` | Moves one dragged key before a target key inside the persisted order array. |
| `mergeAgentOrderWithInventory` | Preserves stored keys that still exist and appends newly discovered agent keys. |

## Core Logic

Uses a persisted-key index map for stable relative ordering but lets enabled state win the top-level grouping so disabled agents never jump above enabled agents.

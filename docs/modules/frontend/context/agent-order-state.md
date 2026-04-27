# Agent Order State Hook

> **Source**: `src/context/agent-order-state.ts`
> **Status**: [REVIEW]

## Overview

Provides the `AppContext`-local state and actions for loading, persisting, and resolving the global agent order.

## Public Surface

| Export | Purpose |
|---|---|
| `useAgentOrderState` | Loads persisted agent ordering, exposes a save action, and derives sorted agent inventory. |

## Core Logic

Wraps the Tauri `get_agent_order` / `set_agent_order` commands, forwards failures into the shared app error surface, and derives `sortedAgentInventory` by combining persisted keys with the current inventory through `src/lib/agent-order.ts`.

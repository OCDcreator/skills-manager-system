# Agent Side Nav

> **Source**: `src/lib/agent-side-nav.ts`
> **Status**: [REVIEW]

## Overview

Owns the Agent Sync floating side-rail preference and inventory shaping helpers.

## Public Surface

| Export | Purpose |
|---|---|
| `readAgentSideNavEnabledOnlyPreference` | Restores the persisted `enabled only` rail preference from `localStorage`. |
| `writeAgentSideNavEnabledOnlyPreference` | Saves the rail preference as best-effort browser storage. |
| `applyAgentEnabledOverrides` | Rehydrates inventory rows with draft-enabled overrides before control rendering. |
| `resolveAgentSideNavInventory` | Applies the persisted order plus optional enabled-only filtering for the floating rail. |

## Core Logic

Keeps the storage key local to the side-nav domain instead of spreading it across the view, treats persistence as best-effort only, and centralizes the enabled-only filtering so `AgentsView` can feed the rail and order modal from the same draft-aware inventory.

## Interactions

Must stay aligned with `src/lib/agent-order.ts`, `src/lib/tauri.ts`, and `src/views/AgentsView.tsx`.

# AgentsView

> **Source**: `src/views/AgentsView.tsx`
> **Status**: [REVIEW]

## Overview

Assembles the per-agent sync editor, including draft state, scene loading, save-and-sync flows, persisted global agent ordering, explicit global-target skill management, sync-mode selection, guarded navigation registration, and right-side floating icon-rail anchors.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentsView` | Page-level orchestration for per-agent configuration, sync mode selection, and explicit target-entry management. |

## Core Logic

The view reads inventory, persisted `agentOrder`, sorted inventory, and sync actions from `AppContext`, loads scene config plus persisted sync mode, materializes editable drafts per agent, preserves dirty drafts across inventory-only refreshes, computes dirty-state and preview data, saves one or all agents through `set_agent_configuration`, persists global order changes through `set_agent_order`, auto-runs sync after save, and delegates confirmation-backed take-over / delete / import flows to `useAgentTargetActions`.

## Data Flow

Context state provides repo/skill/agent state plus the global sorted agent inventory; `src/lib/scenes.ts` loads reusable scene definitions; `src/lib/agent-selection.ts` handles draft/preview normalization; and child components only receive normalized props and callbacks. The page also coordinates inventory refreshes after target-management actions so the right-hand global-skill card and left-hand draft card stay in sync without dropping unsaved edits, and hosts `AgentOrderModal` so the floating nav action can reorder agents globally.

## Interactions

Must stay aligned with the agent draft helper module, `src/components/agents/*`, `src/lib/agent-target-actions.ts`, floating directory anchors, the Tauri command wrappers in `src/lib/tauri.ts`, and the i18n keys used for confirmations and success notices.

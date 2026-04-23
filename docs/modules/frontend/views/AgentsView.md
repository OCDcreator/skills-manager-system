# AgentsView

> **Source**: `src/views/AgentsView.tsx`
> **Status**: [REVIEW]

## Overview

Assembles the per-agent sync editor, including draft state, scene loading, save-and-sync flows, sync-mode selection, and guarded navigation registration.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentsView` | Page-level orchestration for per-agent configuration and sync mode selection. |

## Core Logic

The view reads inventory and sync actions from `AppContext`, loads scene config plus persisted sync mode, materializes editable drafts per agent, computes dirty-state and preview data, saves one or all agents through `set_agent_configuration`, and auto-runs sync after save.

## Data Flow

Context state provides repo/skill/agent state; `src/lib/scenes.ts` loads reusable scene definitions; `src/lib/agent-selection.ts` handles draft/preview normalization; and child components only receive normalized props and callbacks.

## Interactions

Must stay aligned with the agent draft helper module, `src/components/agents/*`, and the settings/agent command wrappers in `src/lib/tauri.ts`.

# AgentsView

> **Source**: `src/views/AgentsView.tsx`
> **Status**: [REVIEW]

## Overview

Assembles the Agent Sync page, keeps global apply state in the summary, and routes agent-scoped configuration, target management, and external-variant import/update/repair data into the per-agent workbench section.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentsView` | Page-level orchestration for per-agent configuration, sync mode selection, target management, and agent-facing external imports. |

## Core Logic

The view still owns sync mode, scene loading, editable drafts, save/apply flows, order persistence, and target-management action wiring. It passes aggregate dirty-draft state plus save/discard handlers into `AgentSyncSummary` instead of conditionally inserting a standalone unsaved banner in the page flow. It also passes single-entry target actions, batch target actions, external-source snapshots, and import/update/repair callbacks into `AgentTargetsSection`, so each rendered agent block can keep configuration, target-directory state, and external variants together. Source creation, fetch, and deletion remain on the dedicated `ExternalSourcesView`.

The view also owns the floating side-rail visibility preference. It persists a local `show enabled only` toggle in `localStorage`, rehydrates it on load, and derives a control-specific agent inventory that reflects unsaved draft enabled-state changes. That derived list feeds both the fixed `AgentFloatingNav` rail and the order modal's enabled/disabled badges, so the rail can hide disabled agents without waiting for a save.

## Interactions

Must stay aligned with `src/lib/agent-selection.ts`, `src/lib/tauri.ts`, `src/components/agents/*`, and the top-level navigation guard. Keep dirty-draft save/discard behavior isolated to agent config state, not to external-source actions.

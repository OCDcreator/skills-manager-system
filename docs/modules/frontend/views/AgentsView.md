# AgentsView

> **Source**: `src/views/AgentsView.tsx`
> **Status**: [REVIEW]

## Overview

Assembles the per-agent sync editor, explicit target management, and the new agent-scoped external-variant import/update/repair surface, while routing unsaved-draft status into the fixed summary area so draft toggles do not insert a separate top-of-page banner.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentsView` | Page-level orchestration for per-agent configuration, sync mode selection, target management, and agent-facing external imports. |

## Core Logic

The view still owns sync mode, scene loading, editable drafts, save/apply flows, order persistence, and target-management action wiring. It now passes aggregate dirty-draft state plus save/discard handlers into `AgentSyncSummary` instead of conditionally inserting a standalone unsaved banner in the page flow. It continues to pass single-entry and batch target actions into `AgentTargetsSection`, while the sidecar components keep local checkbox selection state. After the external GitHub source work it also reads `externalSources` from context and renders one `AgentExternalVariantPanel` per sorted agent below the target-management section. That panel is intentionally agent-scoped: `AgentsView` passes repo-path awareness and import/update/repair callbacks, but source creation, fetch, and deletion remain on the dedicated `ExternalSourcesView`.

The view also owns the floating side-rail visibility preference. It persists a local `show enabled only` toggle in `localStorage`, rehydrates it on load, and derives a control-specific agent inventory that reflects unsaved draft enabled-state changes. That derived list feeds both the fixed `AgentFloatingNav` rail and the order modal's enabled/disabled badges, so the rail can hide disabled agents without waiting for a save.

## Interactions

Must stay aligned with `src/lib/agent-selection.ts`, `src/lib/tauri.ts`, `src/components/agents/*`, and the top-level navigation guard. Keep dirty-draft save/discard behavior isolated to agent config state, not to external-source actions.

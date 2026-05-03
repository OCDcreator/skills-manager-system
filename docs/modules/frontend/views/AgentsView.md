# AgentsView

> **Source**: `src/views/AgentsView.tsx`
> **Status**: [REVIEW]

## Overview

Assembles the per-agent sync editor, explicit target management, and the new agent-scoped external-variant import/update/repair surface.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentsView` | Page-level orchestration for per-agent configuration, sync mode selection, target management, and agent-facing external imports. |

## Core Logic

The view still owns sync mode, scene loading, editable drafts, save/apply flows, order persistence, and target-management action wiring. It passes single-entry and batch target actions into `AgentTargetsSection`, while the sidecar components keep local checkbox selection state. After the external GitHub source work it also reads `externalSources` from context and renders one `AgentExternalVariantPanel` per sorted agent below the target-management section. That panel is intentionally agent-scoped: `AgentsView` passes repo-path awareness and import/update/repair callbacks, but source creation, fetch, and deletion remain on the dedicated `ExternalSourcesView`.

## Interactions

Must stay aligned with `src/lib/agent-selection.ts`, `src/lib/tauri.ts`, `src/components/agents/*`, and the top-level navigation guard. Keep dirty-draft save/discard behavior isolated to agent config state, not to external-source actions.

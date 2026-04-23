# AgentTargetCard

> **Source**: `src/components/agents/AgentTargetCard.tsx`
> **Status**: [REVIEW]

## Overview

Provides the per-agent card UI for sync enablement, path override drafting, direct skill selection, scene selection, and effective preview rendering.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentTargetCard` | One per-agent draft card with save-and-sync action. |

## Core Logic

Renders path badges, default/detected/effective path rows, a draft sync toggle, override input, child selectors, effective preview, and the card-level save button. Persistence and sync execution stay in `AppContext` / `AgentsView`.

## Interactions

Must stay aligned with `AgentInventoryItem` and `AgentConfigurationInput` in `src/lib/tauri.ts`, the draft helpers in `src/lib/agent-selection.ts`, and the child selector components under `src/components/agents/`.

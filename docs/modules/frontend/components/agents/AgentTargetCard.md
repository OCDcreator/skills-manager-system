# AgentTargetCard

> **Source**: `src/components/agents/AgentTargetCard.tsx`
> **Status**: [REVIEW]

## Overview

Provides the primary per-agent configuration card UI for sync enablement, path override drafting, direct skill selection, scene selection, and effective preview rendering.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentTargetCard` | One per-agent draft card with save-and-sync action. |

## Core Logic

Renders a brand badge, path badges, default/detected/effective path rows, a draft sync toggle, override input, the direct-skill / scene / preview selectors, and the card-level save button. The separate global-skill management card now lives beside this component in `AgentsView`, while persistence and sync execution stay in `AppContext` / `AgentsView`.

On wide screens it now caps the direct-skill / scene / preview row itself with a more conservative viewport-responsive `clamp(20rem, calc(100vh - 38rem), 30rem)` height budget, so the first agent card can stay inside the first page more reliably while large skill catalogs still scroll internally. Each inner panel remains an independent flex column inside that capped row, which removes the unwanted top blank space that appeared when different panels were forced to share one measured header height, while still leaving the row overflow visible so the direct-skill external-group popover can escape without being clipped by neighboring panels.

## Interactions

Must stay aligned with `AgentInventoryItem` and `AgentConfigurationInput` in `src/lib/tauri.ts`, the draft helpers in `src/lib/agent-selection.ts`, `AgentBrandIcon`, `AgentGlobalSkillList`, and the child selector components under `src/components/agents/`.

# AgentExternalSourceSection

> **Source**: `src/components/agents/AgentExternalSourceSection.tsx`
> **Status**: [REVIEW]

## Overview

Renders one agent-scoped external source section, including both detected upstream candidates and imported managed mirrors.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentExternalSourceSection` | Per-source renderer for the agent external-variants sidecar. |

## Core Logic

This component owns the dense row-level UI that made `AgentExternalVariantPanel` drift past the repo's warning line. It receives already-filtered imports and variants for one concrete agent, renders source metadata and counts, and keeps import/update/repair button truth scoped to that source. Candidate import actions always target the current agent key, while imported mirror rows keep their existing warning translation, pinned/checked commit display, and repair-versus-update busy-state distinction.

## Interactions

Used only by `AgentExternalVariantPanel`. It must stay aligned with `ExternalSourceRecord`, `ImportedExternalSkillRecord`, `ExternalVariantSnapshot`, and the shared presentation helpers in `src/lib/external-sources.ts`.

# Agent Scene Selector

> **Source**: `src/components/agents/AgentSceneSelector.tsx`
> **Status**: [REVIEW]

## Overview

Shows reusable scene assignments for one agent draft.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentSceneSelector` | Scene checkbox list for one agent draft. |

## Core Logic

Uses the shared draft toggle helper, shows scene descriptions inline, keeps the card stable when no scenes exist yet, and reuses the skill reading pane scrollbar styling for its internal list.

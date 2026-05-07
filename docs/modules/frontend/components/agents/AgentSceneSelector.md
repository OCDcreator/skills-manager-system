# Agent Scene Selector

> **Source**: `src/components/agents/AgentSceneSelector.tsx`
> **Status**: [REVIEW]

## Overview

Shows reusable scene assignments for one agent draft inside the capped three-panel agent row.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentSceneSelector` | Scene checkbox list for one agent draft. |

## Core Logic

Uses the shared draft toggle helper, shows scene descriptions inline, keeps the card stable when no scenes exist yet, and reuses the skill reading pane scrollbar styling for its internal list. The scene list now remembers its vertical position per agent draft and fills the remaining height of the capped `xl` row without borrowing extra top spacing from the neighboring cards.

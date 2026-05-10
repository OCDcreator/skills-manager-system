# AgentCompactTabs

> **Source**: `src/components/agents/AgentCompactTabs.tsx`
> **Status**: [REVIEW]

## Overview

Provides the compact-width agent switcher for the Agent Sync page.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentCompactTabs` | Renders a horizontally scrollable tab strip for choosing the currently focused agent below the wide workbench breakpoint. |

## Core Logic

Receives the ordered agent list plus the current `activeAgentKey` from `AgentsView` and renders a pill-style button for each agent. The active pill gets the highlighted sky treatment while inactive pills keep the neutral slate styling.

The component is intentionally scoped to the compact band only. It is hidden at `>=1280px`, where `AgentFloatingNav` and the full multi-agent list return. Below that breakpoint it becomes the primary navigation control that drives the single-agent visibility contract inside `AgentTargetsSection`.

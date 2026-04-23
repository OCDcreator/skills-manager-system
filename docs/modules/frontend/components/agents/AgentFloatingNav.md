# AgentFloatingNav

> **Source**: `src/components/agents/AgentFloatingNav.tsx`
> **Status**: [REVIEW]

## Overview

Renders the fixed right-side icon rail for the Agent Sync view, keeping top/bottom jumps plus per-agent anchors visible while the page scrolls.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentFloatingNav` | Fixed viewport icon-rail navigation for top/bottom jumps and individual agent cards. |

## Core Logic

Builds a compact vertical anchor list from top/bottom jump targets plus the current agent inventory. The visual treatment uses a transparent viewport rail, right-aligned circular nodes, SVG brand badges, a subtle guide line, reserved row height to prevent scaled-node overlap, fixed-distance left labels, and a magnetic neighborhood interaction where the hovered item grows and moves left most while adjacent items grow and pull left with tapering strength.

## Data Flow

Receives agent keys and display names from `AgentsView`; all section IDs are owned by the same view so anchors remain local to the Agent Sync page.

## Interactions

Keep aligned with `src/views/AgentsView.tsx` anchor IDs, `AgentBrandIcon`, and `agents.sideNav.*` i18n keys.

# AgentFloatingNav

> **Source**: `src/components/agents/AgentFloatingNav.tsx`
> **Status**: [REVIEW]

## Overview

Renders the fixed right-side icon rail for the Agent Sync view, keeping a global ordering action, top/bottom jumps, and per-agent anchors visible while the page scrolls without covering the shared header or the bottom-right assistant launcher.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentFloatingNav` | Fixed viewport icon-rail navigation for the ordering modal, top/bottom jumps, and individual agent cards. |

## Core Logic

Builds a compact vertical anchor list from one action node plus top/bottom jump targets and the current ordered agent inventory. The visual treatment uses a transparent viewport rail, right-aligned circular nodes, SVG brand badges, a subtle guide line, reserved row height to prevent scaled-node overlap, fixed-distance left labels, and a magnetic neighborhood interaction where the hovered item grows and moves left most while adjacent items grow and pull left with tapering strength.

Each anchor keeps a narrow right-aligned hit area (`w-14`) instead of stretching across the whole rail so hover/focus only activates once the pointer is actually near the icon. The transparent rail wrapper stays non-interactive, each list row opts out of pointer events, and only the real button/link hit targets opt back in; labels still expand leftward via absolutely positioned overflow without blocking the page behind them. Agent-specific bubble colors are keyed locally so new brand entries such as Kimi Code CLI can join the rail without changing the shared icon renderer. The rail no longer centers itself against raw `100vh`; it measures the app header, main content bottom edge, and assistant launcher, then applies `top`/`bottom` bounds plus internal scrolling so the icons stay inside the real free space.

## Data Flow

Receives ordered agent keys and display names plus an `onOpenOrderModal` callback from `AgentsView`; all section IDs are owned by the same view so anchors remain local to the Agent Sync page while the first rail node opens the ordering modal instead of scrolling. Layout bounds are derived from `AppShell` and `ProjectAssistantLauncher` via their `data-*` anchors.

## Interactions

Keep aligned with `src/views/AgentsView.tsx` anchor IDs, `AgentBrandIcon`, and `agents.sideNav.*` i18n keys.

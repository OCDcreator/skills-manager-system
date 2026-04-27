# AgentBrandIcon

> **Source**: `src/components/agents/AgentBrandIcon.tsx`
> **Status**: [REVIEW]

## Overview

Renders compact brand marks for Agent Sync surfaces, using public brand-style vectors plus local image assets for larger official marks.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentBrandIcon` | Shared SVG renderer keyed by agent id for nav rails and future agent badges. |

## Core Logic

Maintains a small keyed icon registry and falls back to a neutral glyph when no agent-specific mark exists. The component renders either inline SVG paths or local asset URLs so the UI does not depend on runtime network image fetches. Brand entries that need multi-color fidelity, including Gemini CLI and Kimi Code CLI, should prefer repo-local asset files over simplified inline monochrome paths.

## Data Flow

Receives an `agentKey` from callers such as `AgentFloatingNav`, then resolves the matching SVG definition without touching app state or network APIs.

## Interactions

Keep aligned with the agent catalog keys in `src-tauri/src/core/agents/catalog.rs` and any future UI that reuses agent branding.

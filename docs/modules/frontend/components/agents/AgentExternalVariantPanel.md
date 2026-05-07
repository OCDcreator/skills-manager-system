# AgentExternalVariantPanel

> **Source**: `src/components/agents/AgentExternalVariantPanel.tsx`
> **Status**: [REVIEW]

## Overview

Renders the agent-scoped external-source sidecar inside `AgentsView`, showing only candidates and managed imports relevant to one agent.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentExternalVariantPanel` | Read/write panel for per-agent import, update, and repair actions. |

## Core Logic

The component now stays intentionally thin: it filters the global `externalSources` snapshot down to records that either already own imports for the current `agent.key` or expose candidates whose suggested target agents include that key, then delegates per-source rendering to `AgentExternalSourceSection`. Its import callback still accepts the broader external variant key type, but the panel-level filtering ensures the section only renders work that belongs to the concrete agent.

## Interactions

This panel does not own source caching, fetch, or reference checks. Those stay in `src/views/ExternalSourcesView.tsx` and Rust `core/external_sources/*`. It must stay aligned with `ExternalSourceSnapshotItem`, `AgentExternalSourceSection`, and import warning semantics from `src/lib/tauri.ts`.

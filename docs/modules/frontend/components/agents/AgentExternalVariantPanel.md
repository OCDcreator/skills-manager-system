# AgentExternalVariantPanel

> **Source**: `src/components/agents/AgentExternalVariantPanel.tsx`
> **Status**: [REVIEW]

## Overview

Renders the agent-scoped external-source sidecar inside `AgentsView`, showing only variants and managed imports relevant to one agent.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentExternalVariantPanel` | Read/write panel for per-agent import, update, and repair actions. |

## Core Logic

The component filters the global `externalSources` snapshot down to records that either expose variants for the current `agent.key` or already own imports for that agent. Its import callback accepts the broader external variant key type, but this panel only renders entries matching the concrete agent key. It presents a left column for detected upstream variants and a right column for imported mirrors, translates warning codes like `variant_disappeared` and `integrity_mismatch`, and keeps busy-state UX scoped to either source-level imports or import-level update/repair actions.

## Interactions

This panel does not own source caching, fetch, or reference checks. Those stay in `src/views/ExternalSourcesView.tsx` and Rust `core/external_sources/*`. It must stay aligned with `ExternalSourceSnapshotItem` and import warning semantics from `src/lib/tauri.ts`.

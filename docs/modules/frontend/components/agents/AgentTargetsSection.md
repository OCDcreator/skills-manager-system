# AgentTargetsSection

> **Source**: `src/components/agents/AgentTargetsSection.tsx`
> **Status**: [REVIEW]

## Overview

Renders the ordered list of per-agent workbench blocks in the Agent Sync page.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentTargetsSection` | Maps ordered agents into a configuration card, target-directory sidecar, and external-variant panel. |

## Core Logic

Receives sorted agents, the compact `activeAgentKey`, current drafts, scene config, target-management callbacks, and external-source import/update/repair callbacks from `AgentsView`. It derives each row preview through `resolveAgentSelectionPreview`, forwards the live managed target inventory so the preview can distinguish already-synced skills from pending sync items, and keeps the agent-scoped `AgentExternalVariantPanel` directly below the same agent's configuration and target-directory controls.

The section now has a breakpoint-aware visibility contract: below `1280px`, only the active compact agent block renders as `block`, while inactive agents stay hidden to protect vertical information density on smaller screens. At `>=1280px`, every agent row is forced back to `block` and the existing `min-[1380px]` target-sidecar split remains intact for the wide workbench layout.

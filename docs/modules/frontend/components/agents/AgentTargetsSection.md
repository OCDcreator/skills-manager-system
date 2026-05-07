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

Receives sorted agents, current drafts, scene config, target-management callbacks, and external-source import/update/repair callbacks from `AgentsView`. It derives each row preview through `resolveAgentSelectionPreview`, forwards the live managed target inventory so the preview can distinguish already-synced skills from pending sync items, and keeps the agent-scoped `AgentExternalVariantPanel` directly below the same agent's configuration and target-directory controls. This preserves the existing wide-screen sidecar layout for target management while avoiding a separate lower page region that repeats all agents for variants.

# AgentTargetsSection

> **Source**: `src/components/agents/AgentTargetsSection.tsx`
> **Status**: [REVIEW]

## Overview

Renders the ordered list of per-agent configuration rows in the Agent Sync page.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentTargetsSection` | Maps ordered agents into `AgentTargetCard` plus `AgentGlobalSkillList` pairs. |

## Core Logic

Receives sorted agents, current drafts, scene config, and target-management callbacks from `AgentsView`, derives each row preview through `resolveAgentSelectionPreview`, and preserves the existing wide-screen sidecar layout while moving the bulky row mapping out of the page view file.

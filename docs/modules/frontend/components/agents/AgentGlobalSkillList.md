# AgentGlobalSkillList

> **Source**: `src/components/agents/AgentGlobalSkillList.tsx`
> **Status**: [REVIEW]

## Overview

Renders the read-only inventory of skills already present in an agent's effective global skills directory.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentGlobalSkillList` | Displays app-managed and existing unmanaged target skill entries for one agent. |

## Core Logic

Counts managed versus unmanaged entries, reports target scan errors, and lists target directory entries with path, manifest ownership, and `SKILL.md` availability. It intentionally does not delete or adopt unmanaged entries; sync safety remains owned by the backend manifest rules.

## Data Flow

Receives `AgentInventoryItem.targetSkillEntries` and `targetSkillScanError` from `AgentsView` through `AgentTargetCard`.

## Interactions

Keep aligned with `AgentTargetSkillEntry` in `src/lib/tauri.ts`, target inventory scanning in Rust, and `agents.globalSkills.*` i18n keys.

# AgentGlobalSkillList

> **Source**: `src/components/agents/AgentGlobalSkillList.tsx`
> **Status**: [REVIEW]

## Overview

Renders the sidecar inventory and explicit management actions for skills already present in an agent's effective global skills directory.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentGlobalSkillList` | Displays app-managed and existing unmanaged target skill entries for one agent and exposes explicit take-over / delete / import actions. |

## Core Logic

Counts managed versus unmanaged entries, reports target scan errors, highlights taken-over entries separately from synced ones, and lists per-entry actions. The sidecar shell is a full-height flex column and the scrollable list uses `flex-1` instead of a fixed max-height so it fills the available card height. The component never mutates files directly: it forwards explicit button clicks back to `AgentsView`, which owns confirmation dialogs and Tauri command calls.

## Data Flow

Receives `AgentInventoryItem.targetSkillEntries`, the current in-flight action key, and action callbacks directly from `AgentsView`.

## Interactions

Keep aligned with `AgentTargetSkillEntry` in `src/lib/tauri.ts`, target inventory scanning in Rust, and `agents.globalSkills.*` i18n keys.

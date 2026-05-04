# Agent Skill Selector

> **Source**: `src/components/agents/AgentSkillSelector.tsx`
> **Status**: [REVIEW]

## Overview

Lets one agent draft choose direct skills with search, a selected-only toggle, and global-hard-disable feedback.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentSkillSelector` | Searchable direct-skill checkbox list for one agent draft. |

## Core Logic

Filters skills by search text and an `已选中` / selected-only toggle, keeps already-selected globally disabled skills removable, clears matching exclusions when a direct skill is re-selected, and reuses the skill reading pane scrollbar styling for its internal list. The search row keeps the new selected-only button directly beside the text field so direct-pick triage stays compact. The result list restores its last scroll position per agent draft and shows a small empty-state message when the current filter combination matches nothing.

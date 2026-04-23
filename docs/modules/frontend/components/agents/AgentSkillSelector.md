# Agent Skill Selector

> **Source**: `src/components/agents/AgentSkillSelector.tsx`
> **Status**: [REVIEW]

## Overview

Lets one agent draft choose direct skills with search and global-hard-disable feedback.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentSkillSelector` | Searchable direct-skill checkbox list for one agent draft. |

## Core Logic

Filters skills by search text, keeps already-selected globally disabled skills removable, clears matching exclusions when a direct skill is re-selected, and reuses the skill reading pane scrollbar styling for its internal list.

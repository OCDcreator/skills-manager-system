# Agent Skill Selector

> **Source**: `src/components/agents/AgentSkillSelector.tsx`
> **Status**: [REVIEW]

## Overview

Lets one agent draft choose direct skills with search, path-bucket pills, a selected-only toggle, and global-hard-disable feedback.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentSkillSelector` | Searchable direct-skill checkbox list for one agent draft. |

## Core Logic

Filters skills by search text, top-level path buckets derived from `relativePath`, and an `已选中` / selected-only toggle. It keeps already-selected globally disabled skills removable, clears matching exclusions when a direct skill is re-selected, and reuses the skill reading pane scrollbar styling for its internal list.

The path pills follow the same repository-structure language as the Skills page and Projects workbench, so the user can quickly focus on `custom` or `external` skills without typing. The result list restores its last scroll position per agent draft and shows a small empty-state message when the current filter combination matches nothing.

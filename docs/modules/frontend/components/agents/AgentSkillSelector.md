# Agent Skill Selector

> **Source**: `src/components/agents/AgentSkillSelector.tsx`
> **Status**: [REVIEW]

## Overview

Lets one agent draft choose direct skills with search, path-bucket pills, a selected-only toggle, and global-hard-disable feedback inside the capped three-panel agent row.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentSkillSelector` | Searchable direct-skill checkbox list for one agent draft. |

## Core Logic

Filters skills by search text, top-level path buckets derived from `relativePath`, and an `已选中` / selected-only toggle. It keeps already-selected globally disabled skills removable, clears matching exclusions when a direct skill is re-selected, and reuses the skill reading pane scrollbar styling for its internal list.

When the `external` path pill is active, the selector also opens the same external subgroup popover model used by Scenes and Project Assignment. That subgroup list is built from `buildExternalGroupSummaries()`, so direct-skill filtering can narrow to `external/<source>` collections without flattening those child paths into the main pill row. The result list restores its last scroll position per agent draft and, on `xl` screens, fills the remaining height of the capped three-panel row while keeping its own header block independent from the scene and preview cards.

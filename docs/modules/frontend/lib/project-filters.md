# Project Filters Helper

> **Source**: `src/lib/project-filters.ts`
> **Status**: [REVIEW]

## Overview

Pure filtering helpers for the Projects workbench editor. Includes single-agent and multi-agent skill filtering.

## Public Surface

| Export | Purpose |
|---|---|
| `ProjectAgentStatusFilter` | Agent status filter union: `all`, `enabled`, `disabled`. |
| `ProjectSkillSelectionFilter` | Skill selected-state filter union: `all`, `selected`, `unselected`. |
| `filterProjectSkills` | Applies free-text, repository-path, external-group, and selected-state filters to skill rows (single-agent variant). |
| `filterProjectSkillsForMultiAgent` | Multi-agent variant that checks three-state selection (`all`/`some`/`none`) across multiple agents to determine whether a skill matches the `selectionFilter`. |
| `filterProjectAgents` | Applies free-text and enabled-state filters to agent rows. |
| `sortProjectAgentsForEditor` | Keeps already-configured agents near the top while editing an existing project assignment. |

## Multi-Agent Filtering

`filterProjectSkillsForMultiAgent` accepts an `agents` record and `agentKeys` array instead of a flat `selectedSkillIds` list. It uses an internal `getSkillFilterState` helper that returns `"all"`, `"some"`, or `"none"` for each skill across the selected agents:
- `selectionFilter === "selected"`: passes if state is `"all"` or `"some"` (at least one agent has the skill)
- `selectionFilter === "unselected"`: passes if state is `"none"` or `"some"` (at least one agent lacks the skill)
- `selectionFilter === "all"`: passes regardless of state

## Current Note

This module is intentionally separate from `project-draft.ts`: it keeps view-facing list filters out of the draft mutation helper and gives the Projects view a stable place for editor filtering rules.

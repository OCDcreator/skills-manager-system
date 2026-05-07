# Project Filters Helper

> **Source**: `src/lib/project-filters.ts`
> **Status**: [REVIEW]

## Overview

Pure filtering helpers for the Projects workbench editor.

## Public Surface

| Export | Purpose |
|---|---|
| `ProjectAgentStatusFilter` | Agent status filter union: `all`, `enabled`, `disabled`. |
| `ProjectSkillSelectionFilter` | Skill selected-state filter union: `all`, `selected`, `unselected`. |
| `filterProjectSkills` | Applies free-text, repository-path, external-group, and selected-state filters to skill rows. |
| `filterProjectAgents` | Applies free-text and enabled-state filters to agent rows. |
| `sortProjectAgentsForEditor` | Keeps already-configured agents near the top while editing an existing project assignment. |

## Current Note

This module is intentionally separate from `project-draft.ts`: it keeps view-facing list filters out of the draft mutation helper and gives the Projects view a stable place for editor filtering rules.

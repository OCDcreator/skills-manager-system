# Scene Skill Filters

> **Source**: `src/lib/scene-skill-filters.ts`
> **Status**: [REVIEW]

## Overview

Pure helper module for the scene skill chooser's filtering and external-source grouping.

## Public Surface

| Export | Purpose |
|---|---|
| `SceneSkillStatusFilter` / `SceneSkillSourceFilter` / `ExternalGroupFilter` | Filter key types used by `SceneSkillChooser`. |
| `getOrderedEnabledSkillSummaries` | Rehydrates ordered enabled scene rows back to full `SkillSummary` records. |
| `filterSceneSkills` | Applies search, status, source, and external subgroup filters to one enabled/disabled row set. |
| `buildSceneSkillSourceCounts` | Counts all/custom/external skills for source pills. |
| `buildExternalGroupSummaries` | Builds the external subgroup popover from managed import metadata or `external/<source>` path segments. |
| `getExternalGroupKey` | Stable grouping key for one external skill. |

## Data Flow

The module keeps all filtering pure and frontend-only. Scene membership remains owned by `SceneEntry`, source information comes from `SkillSummary.sourceType`, and external subgroup labels prefer `managedSource.repoUrl + agentKey` when available before falling back to repository-relative path structure.

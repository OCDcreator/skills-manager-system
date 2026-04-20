# Skill Filters

> **Source**: `src/lib/skills/filters.ts`
> **Status**: [REVIEW]

## Overview

Provides pure frontend helpers for skill source summaries, search/source filtering, grouping, and description truncation.

## Import Relationships

```text
Upstream: src/views/SkillsView.tsx, src/components/skills/SkillList.tsx, src/components/skills/SkillFilters.tsx
Downstream: src/lib/tauri.ts
```

## Public Surface

| Export | Purpose |
|---|---|
| `SourceFilter` | Union of supported source filter keys. |
| `SourceSummary` | Count payload used by filter controls. |
| `buildSourceSummaries` | Counts all, custom, and external skills. |
| `filterSkills` | Applies source and text filtering. |
| `groupSkills` | Splits skills into custom and external arrays. |
| `truncateDescription` | Shortens long descriptions with an ellipsis. |

## Core Logic

Filtering trims and lowercases the search term, then matches against skill name, description, or relative path after applying the selected source filter.

## Data Flow

`SkillSummary[]` values from Tauri are transformed into view-ready arrays and counts.

## Interactions

Source keys must stay in sync with backend `source_type` values, i18n `skills.source.*` keys, and UI filter controls.

## Configuration

`truncateDescription` defaults to 100 characters.

## Change Notes

If a new source type is introduced, update the union, summary counts, grouping, i18n keys, and the Rust scanner together.

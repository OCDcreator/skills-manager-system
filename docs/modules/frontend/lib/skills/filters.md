# Skill Filters

> **Source**: `src/lib/skills/filters.ts`
> **Status**: [REVIEW]

## Overview

Provides pure frontend helpers for skill source summaries, status summaries, path-bucket summaries, search/source/status filtering, grouping, and description truncation.

## Import Relationships

```text
Upstream: src/views/SkillsView.tsx, src/components/skills/SkillList.tsx, src/components/skills/SkillFilters.tsx
Downstream: src/lib/tauri.ts
```

## Public Surface

| Export | Purpose |
|---|---|
| `SourceFilter` | Union of supported source filter keys. |
| `VisibleSource` | Source keys that map to rendered skill sections. |
| `SkillStatusFilter` | Union of supported status filter keys. |
| `SourceSummary` | Count payload used by filter controls. |
| `StatusSummary` | Count payload used by status filter controls. |
| `SkillPathFilter` / `SkillPathSummary` | Top-level relative-path filter key plus count payload for path pills. |
| `buildSourceSummaries` | Counts all, custom, and external skills. |
| `buildStatusSummaries` | Counts all, enabled, and disabled skills. |
| `buildSkillPathSummaries` | Counts top-level path buckets such as `custom` and `external`. |
| `matchesSkillPathFilter` | Checks whether a skill belongs to the active top-level path bucket. |
| `filterSkills` | Applies source, status, and text filtering. |
| `groupSkills` | Splits skills into custom and external arrays. |
| `resolveVisibleSources` | Expands the current source filter into the source sections that should render. |
| `truncateDescription` | Shortens long descriptions with an ellipsis. |

## Core Logic

Filtering trims and lowercases the search term, applies source and status filters, then matches against skill name, description, or relative path. Path summaries are derived from the first segment of each `relativePath`, which keeps filters tied to repository structure instead of duplicating separate hard-coded source enums in every chooser.

## Data Flow

`SkillSummary[]` values from Tauri plus the disabled-ID set from `AppContext` are transformed into view-ready arrays and counts. The agent direct-skill chooser and the Projects workbench reuse the same path-bucket helpers so `custom` / `external` filtering behaves consistently across pages.

## Interactions

Source keys and status keys must stay in sync with backend DTOs, i18n `skills.source.*` and `skills.status.*` keys, and UI filter controls.

## Configuration

`truncateDescription` defaults to 100 characters.

## Change Notes

If a new source type is introduced, update the union, summary counts, grouping, i18n keys, and the Rust scanner together.

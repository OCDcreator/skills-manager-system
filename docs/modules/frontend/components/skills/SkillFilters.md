# SkillFilters

> **Source**: `src/components/skills/SkillFilters.tsx`
> **Status**: [REVIEW]

## Overview

Renders search, refresh, source-filter, and status-filter controls for the skill browser.

## Import Relationships

```text
Upstream: src/views/SkillsView.tsx
Downstream: src/lib/skills/filters.ts, src/i18n/index.ts
```

## Public Surface

| Export | Purpose |
|---|---|
| `SkillFilters` | Controlled filter toolbar driven by parent state plus source and status summaries. |

## Core Logic

The component receives current search text, selected source/status filters, source/status counts, refresh state, and first-scan loading state. It renders a search input, refresh button, and one button per filter summary. During the first scan it labels counts as scanning instead of showing the empty initial snapshot as zero. Source and status filters share one compact wrapping row, with each group keeping its label inline with its filter pills instead of stacking the groups vertically.

## Data Flow

Input changes and button clicks call callbacks owned by `SkillsView`; this component does not own filtering logic.

## Interactions

Filter keys must match `SourceFilter`, `SkillStatusFilter`, and the related `skills.source.*` / `skills.status.*` i18n keys.

## Configuration

None.

## Change Notes

Keep count calculation in `src/lib/skills/filters.ts`; this component should only render provided summaries.

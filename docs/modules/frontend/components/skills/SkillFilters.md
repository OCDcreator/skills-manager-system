# SkillFilters

> **Source**: `src/components/skills/SkillFilters.tsx`
> **Status**: [REVIEW]

## Overview

Renders search, refresh, and source-filter controls for the skill browser.

## Import Relationships

```text
Upstream: src/views/SkillsView.tsx
Downstream: src/lib/skills/filters.ts, src/i18n/index.ts
```

## Public Surface

| Export | Purpose |
|---|---|
| `SkillFilters` | Controlled filter toolbar driven by parent state and source summaries. |

## Core Logic

The component receives current search text, selected source filter, source counts, and refresh state. It renders a search input, refresh button, and one button per source summary.

## Data Flow

Input changes and button clicks call callbacks owned by `SkillsView`; this component does not own filtering logic.

## Interactions

Filter keys must match `SourceFilter` and `skills.source.*` i18n keys.

## Configuration

None.

## Change Notes

Keep count calculation in `src/lib/skills/filters.ts`; this component should only render provided summaries.

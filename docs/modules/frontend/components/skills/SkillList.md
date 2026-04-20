# SkillList

> **Source**: `src/components/skills/SkillList.tsx`
> **Status**: [REVIEW]

## Overview

Displays a selectable list of skill summaries for one source group.

## Import Relationships

```text
Upstream: src/views/SkillsView.tsx
Downstream: src/lib/skills/filters.ts, src/lib/tauri.ts, src/i18n/index.ts
```

## Public Surface

| Export | Purpose |
|---|---|
| `SkillList` | List component for a titled skill group. |

## Core Logic

The component renders the group title, count, an empty state, and one button per skill. Selected skills receive highlighted styling, and descriptions are truncated before display.

## Data Flow

`SkillsView` passes grouped skills and selection state. Selecting an item calls the parent `onSelect` callback with the clicked `SkillSummary`.

## Interactions

Uses `truncateDescription` from `src/lib/skills/filters.ts` and source/no-description i18n keys.

## Configuration

Description truncation uses the helper default limit unless that helper changes.

## Change Notes

Do not fetch skill documents here; selection side effects belong to `AppContext.selectSkill`.

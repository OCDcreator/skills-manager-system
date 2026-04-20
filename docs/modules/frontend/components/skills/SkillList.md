# SkillList

> **Source**: `src/components/skills/SkillList.tsx`
> **Status**: [REVIEW]

## Overview

Displays a selectable list of skill summaries for one source group, including enable/disable state.

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

The component renders the group title, count, an empty state, and one row per skill. Selected skills receive highlighted styling, disabled skills receive subdued styling plus a status badge, and each row exposes a toggle button while descriptions are truncated before display.

## Data Flow

`SkillsView` passes grouped skills, the disabled-ID set, selection state, and the row-level toggle callback. Selecting an item calls the parent `onSelect` callback with the clicked `SkillSummary`, while toggles call the parent persistence action.

## Interactions

Uses `truncateDescription` from `src/lib/skills/filters.ts` plus source, status, toggle, and no-description i18n keys.

## Configuration

Description truncation uses the helper default limit unless that helper changes.

## Change Notes

Do not fetch skill documents or persist state here; selection and mutation side effects belong to `AppContext`.

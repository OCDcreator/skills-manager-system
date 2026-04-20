# SkillsView

> **Source**: `src/views/SkillsView.tsx`
> **Status**: [REVIEW]

## Overview

Composes the skill browser page, including unconfigured state, filters, warnings/errors, grouped skill lists, and the selected skill detail panel.

## Import Relationships

```text
Upstream: src/App.tsx
Downstream: src/components/skills/*, src/context/AppContext.tsx, src/lib/skills/filters.ts, src/i18n/index.ts
```

## Public Surface

| Export | Purpose |
|---|---|
| `SkillsView` | Page-level view for browsing and selecting skills. |

## Core Logic

The view reads app state from context, owns local search/source filter state, memoizes filtered/grouped skills and source summaries, renders setup prompts when no repo path exists, and passes selection/refresh callbacks to child components.

## Data Flow

Context supplies scan results and selected document state. Local filter state transforms scan results before they are displayed in custom and external skill lists.

## Interactions

Must stay aligned with `SourceFilter`, filter helpers, context action names, and `skills.*` i18n keys.

## Configuration

Initial source filter is `all`; search text starts empty.

## Change Notes

Do not move skill scanning or document fetching into this view; those flows belong to `AppContext` and Tauri commands.

# SkillsView

> **Source**: `src/views/SkillsView.tsx`
> **Status**: [REVIEW]

## Overview

Composes the skill browser page, including unconfigured state, source/status filters, warnings/errors, grouped skill lists, and the selected skill detail panel.

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

The view reads app state from context, owns local search/source/status filter state, memoizes the disabled-ID set plus filtered/grouped skills and filter summaries, renders setup prompts when no repo path exists, and passes selection, refresh, and row-toggle callbacks to child components.

## Data Flow

Context supplies scan results, disabled IDs, and selected document state. Local filter state transforms the joined scan-plus-status view model before it is displayed in custom and external skill lists.

## Interactions

Must stay aligned with `SourceFilter`, `SkillStatusFilter`, filter helpers, context action names, and `skills.*` i18n keys.

## Configuration

Initial source and status filters are `all`; search text starts empty.

## Change Notes

Do not move skill scanning or document fetching into this view; those flows belong to `AppContext` and Tauri commands. The three-column shell also needs `minmax(0, …)` / `min-w-0` guards so long detail content cannot steal width from the skill lists, and the detail rail should use a bounded responsive width (for example `clamp(...)`) so it grows with the window without becoming too narrow or too wide.

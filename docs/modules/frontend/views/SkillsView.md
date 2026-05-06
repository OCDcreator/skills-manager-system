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

The view reads app state from context, owns local search/source/status filter state, memoizes the disabled-ID set plus filtered/grouped skills, visible source sections, and filter summaries, renders setup prompts when no repo path exists, and passes selection, refresh, row-toggle, and source-window bulk enablement callbacks to child components. During the first saved-repo scan it treats the empty scan snapshot as a loading state instead of a real no-match result, forwarding that state to the filter toolbar, source lists, and detail placeholder. Its responsive layout keeps the detail panel docked on the right for wide windows, shows both source sections side by side only for the `all` source filter, and lets a single selected source section expand to the full list rail width.

The selected-skill detail rail is now split into two paths:

- no selected skill: render a lightweight inline placeholder rail immediately
- selected skill: lazy-load `SkillDetailPanel` inside `Suspense`

That keeps markdown rendering and syntax highlighting out of the initial bundle until the user actually opens a skill.

## Data Flow

Context supplies scan results, disabled IDs, and selected document state. Local filter state transforms the joined scan-plus-status view model before it is displayed in the currently visible source sections. Source-window bulk actions call the existing `setSkillEnabled` action sequentially for each requested skill so persistent disabled-state writes do not race each other. The selected-skill branch then decides whether to keep the placeholder rail or mount the lazy detail panel.

## Interactions

Must stay aligned with `SourceFilter`, `SkillStatusFilter`, filter helpers, context action names, and `skills.*` i18n keys.

## Configuration

Initial source and status filters are `all`; search text starts empty.

## Change Notes

Do not move skill scanning or document fetching into this view; those flows belong to `AppContext` and Tauri commands. The responsive shell needs `minmax(0, …)` / `min-w-0` guards so long detail content cannot steal width from the skill lists, and the detail rail should use a bounded `clamp(...)` width so it grows with the window without becoming too narrow or too wide.

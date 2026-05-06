# Scene Skill Chooser

> **Source**: `src/components/scenes/SceneSkillChooser.tsx`
> **Status**: [REVIEW]

## Overview

Dense skill-selection workbench for one scene toolkit. It owns only local filtering and drag UI state while `SceneCard` owns persistence callbacks.

## Public Surface

| Export | Purpose |
|---|---|
| `SceneSkillChooser` | Renders searchable, filterable scene skill membership controls. |

## Rendering Logic

The chooser derives enabled rows through `getOrderedEnabledSkillSummaries` so drag ordering matches scene persistence while retaining full skill metadata, and derives disabled rows through `isSceneSkillEnabled`. Local filters narrow by free-text search, scene membership status, source type (`custom` / `external`), and an external subgroup popover. External subgroup computation lives in `src/lib/scene-skill-filters.ts` and prefers managed GitHub import metadata when present, otherwise the first two `relativePath` segments such as `external/<source>`.

## Interactions

Enabled rows expose a real HTML drag payload via `dataTransfer` so desktop webviews can drop reliably. The external subgroup menu is anchored to the active filter button and floats above it so it reads as a control overlay instead of list content. The scroll area uses the shared `skill-markdown-scroll` skin and remembered scroll position.

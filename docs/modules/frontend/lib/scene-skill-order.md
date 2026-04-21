# Scene Skill Order

> **Source**: `src/lib/scene-skill-order.ts`
> **Status**: [REVIEW]

## Overview

Small frontend helper for deriving ordered enabled skills and computing the next `skillOrder` array after a drag-drop reorder.

## Public Surface

| Export | Purpose |
|---|---|
| `getOrderedEnabledSceneSkills` | Filters disabled skills and sorts enabled skills by saved priority. |
| `reorderSceneSkillOrder` | Reorders one enabled skill onto another and returns the next `skillOrder` payload. |

## Interactions

Shared by `src/views/ScenesView.tsx` and `src/components/scenes/SceneCard.tsx` so drag-order logic stays out of page/component markup.

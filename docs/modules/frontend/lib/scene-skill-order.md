# Scene Skill Order

> **Source**: `src/lib/scene-skill-order.ts`
> **Status**: [REVIEW]

## Overview

Small frontend helper for deriving mode-aware enabled skills and computing the next `skillOrder` array after a drag-drop reorder.

## Public Surface

| Export | Purpose |
|---|---|
| `isSceneSkillEnabled` / `getSceneEnabledSkillCount` | Resolve selection state for legacy and explicit scene modes. |
| `getOrderedEnabledSceneSkills` | Filters disabled skills and sorts enabled skills by saved priority. |
| `reorderSceneSkillOrder` | Reorders one enabled skill onto another and returns the next `skillOrder` payload. |

## Interactions

Shared by `src/views/ScenesView.tsx`, `src/components/scenes/SceneCard.tsx`, and `src/lib/agent-selection.ts` so scene selection semantics stay consistent across toggles, counts, previews, and drag-order logic.

# Scene Card

> **Source**: `src/components/scenes/SceneCard.tsx`
> **Status**: [REVIEW]

## Overview

Card component for one scene, including edit controls, skill enable/disable toggles, drag-and-drop priority ordering, and agent selection.

## Props

| Prop | Purpose |
|---|---|
| `scene` | Current scene data. |
| `skills` / `agents` | Available skill + agent catalogs for configuration. |
| `isActive` / `isEditing` / `isApplying` / `isConfiguring` | UI mode flags supplied by `ScenesView`. |
| `onToggleSkill` / `onToggleAgent` | Persist skill/agent membership changes. |
| `onReorderSkill` | Reorders enabled skills by dragging one skill onto another. |
| other edit/apply callbacks | Scene CRUD and apply actions. |

## Rendering Logic

Enabled-skill counts and row membership are derived through the shared scene selection helper, then ordered through `getOrderedEnabledSceneSkills`. That keeps explicit-empty new scenes rendering as zero selected skills while legacy scenes still read as all-except-disabled. Disabled skills remain listed below without drag affordances.

## Interactions

Used by `src/views/ScenesView.tsx`; relies on `scenes.card.*` i18n keys and should keep business logic out of the card itself.

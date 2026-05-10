# Scene Card

> **Source**: `src/components/scenes/SceneCard.tsx`
> **Status**: [REVIEW]

## Overview

Card component for one reusable scene toolkit, including edit controls, summary counts, the scene skill chooser, and a neutral note that the toolkit is consumed from Agent Sync or Projects.

## Props

| Prop | Purpose |
|---|---|
| `scene` | Current scene data. |
| `skills` | Available skill catalog for configuration. |
| `isEditing` / `isConfiguring` | UI mode flags supplied by `ScenesView`. |
| `onToggleSkill` | Persists skill membership changes emitted by `SceneSkillChooser`. |
| `onReorderSkill` | Reorders enabled skills by dragging one skill onto another. |
| other edit callbacks | Scene CRUD actions. |

## Rendering Logic

Enabled-skill counts use the shared scene selection helper so explicit-empty new scenes render as zero selected skills while legacy scenes still read as all-except-disabled. The bulky search, filter, scroll, and drag behavior lives in `SceneSkillChooser`, keeping this card focused on shell controls and scene edit actions. The footer no longer exposes direct apply or target-Agent controls.

When the skill chooser is open, the card now marks that surface with `data-scene-active-config=\"true\"` so compact-layout checks can distinguish the actively configured scene from summary-only cards without adding new view state.

## Interactions

Used by `src/views/ScenesView.tsx`; relies on `SceneSkillChooser`, `scenes.card.*` i18n keys, and should keep business logic out of the card itself.

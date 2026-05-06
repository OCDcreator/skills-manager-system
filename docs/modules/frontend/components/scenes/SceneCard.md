# Scene Card

> **Source**: `src/components/scenes/SceneCard.tsx`
> **Status**: [REVIEW]

## Overview

Card component for one reusable scene toolkit, including edit controls, skill enable/disable toggles, drag-and-drop priority ordering, agent membership selection, and a neutral note that the toolkit is consumed from Agents or Projects.

## Props

| Prop | Purpose |
|---|---|
| `scene` | Current scene data. |
| `skills` / `agents` | Available skill + agent catalogs for configuration. |
| `isEditing` / `isConfiguring` | UI mode flags supplied by `ScenesView`. |
| `onToggleSkill` / `onToggleAgent` | Persist skill/agent membership changes. |
| `onReorderSkill` | Reorders enabled skills by dragging one skill onto another. |
| other edit callbacks | Scene CRUD actions. |

## Rendering Logic

Enabled-skill counts and row membership are derived through the shared scene selection helper, then ordered through `getOrderedEnabledSceneSkills`. That keeps explicit-empty new scenes rendering as zero selected skills while legacy scenes still read as all-except-disabled. Disabled skills remain listed below without drag affordances, each configured sublist remembers its own scroll offset per scene, and the footer no longer exposes a direct apply button.

## Interactions

Used by `src/views/ScenesView.tsx`; relies on `scenes.card.*` i18n keys and should keep business logic out of the card itself.

# Scene Card

> **Source**: `src/components/scenes/SceneCard.tsx`
> **Status**: [DRAFT]

## Overview

Card component for displaying and configuring a single scene, including skill enable/disable checkboxes and agent target selection.

## Props

| Prop | Type | Purpose |
|---|---|---|
| `scene` | `SceneEntry` | Scene data |
| `skills` | `{id, name}[]` | Available skills for selection |
| `agents` | `{key, displayName}[]` | Available agents for selection |
| `isActive` | `boolean` | Whether this is the active scene |
| `isEditing` | `boolean` | Name/description edit mode |
| `isApplying` | `boolean` | Apply in-progress |
| `isConfiguring` | `boolean` | Skill/agent panel open |
| `t` | i18n function | Translation |
| `onStartEdit` | `() => void` | Begin name edit |
| `onCancelEdit` | `() => void` | Cancel name edit |
| `onSaveEdit` | `() => void` | Save name/description |
| `onDelete` | `() => void` | Delete scene |
| `onApply` | `() => void` | Apply scene |
| `onDuplicate` | `() => void` | Duplicate scene |
| `onToggleConfigure` | `() => void` | Toggle config panel |
| `onToggleSkill` | `(id) => void` | Toggle skill enabled |
| `onToggleAgent` | `(key) => void` | Toggle agent enabled |
| `onEditNameChange` | `(v) => void` | Update edit name |
| `onEditDescChange` | `(v) => void` | Update edit description |

## Interactions

- Used by `ScenesView.tsx` to render each scene

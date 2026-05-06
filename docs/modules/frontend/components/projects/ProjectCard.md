# Project Card

> **Source**: `src/components/projects/ProjectCard.tsx`
> **Status**: [DRAFT]

## Overview

Saved-project management card showing per-agent layer counts, edit/delete actions, and session-scoped apply feedback.

## Props

| Prop | Type | Purpose |
|---|---|---|
| `project` | `ProjectAssignment` | Project data |
| `sessionApplyMessage` | `string \| null` | Latest session-only apply result for this project |
| `onEdit` | `() => void` | Reopen the saved project in the workbench |
| `onDelete` | `() => void` | Delete handler |
| `t` | i18n function | Translation |

## Interactions

- Used by `SavedProjectsSection.tsx` to render each saved project below the main workbench
- Reads both the new per-agent assignment shape and legacy flat compatibility fields while saved configs migrate

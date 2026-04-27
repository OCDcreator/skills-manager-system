# Project Card

> **Source**: `src/components/projects/ProjectCard.tsx`
> **Status**: [DRAFT]

## Overview

Saved-project management card showing counts, edit/delete actions, and session-scoped apply feedback.

## Props

| Prop | Type | Purpose |
|---|---|---|
| `project` | `ProjectAssignment` | Project data |
| `sessionApplyMessage` | `string \| null` | Latest session-only apply result for this project |
| `onEdit` | `() => void` | Reopen the saved project in the workbench |
| `onDelete` | `() => void` | Delete handler |
| `t` | i18n function | Translation |

## Interactions

- Used by `ProjectsView.tsx` to render each saved project below the main workbench

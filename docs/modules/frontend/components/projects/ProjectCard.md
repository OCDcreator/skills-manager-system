# Project Card

> **Source**: `src/components/projects/ProjectCard.tsx`
> **Status**: [DRAFT]

## Overview

Card component displaying a single project assignment with skill/agent counts, apply, and delete actions.

## Props

| Prop | Type | Purpose |
|---|---|---|
| `project` | `ProjectAssignment` | Project data |
| `isApplying` | `boolean` | Apply in-progress state |
| `onApply` | `() => void` | Apply handler |
| `onDelete` | `() => void` | Delete handler |
| `t` | i18n function | Translation |

## Interactions

- Used by `ProjectsView.tsx` to render each project

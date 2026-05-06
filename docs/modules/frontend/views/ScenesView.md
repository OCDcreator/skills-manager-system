# Scenes View

> **Source**: `src/views/ScenesView.tsx`
> **Status**: [REVIEW]

## Overview

Page-level scene management view for creating, editing, configuring, and drag-reordering reusable scene toolkit definitions.

## Public Surface

| Export | Purpose |
|---|---|
| `ScenesView` | Scene management page component. |

## Core Logic

Owns scene CRUD form state, loads `SceneConfigSnapshot` through a deferred initial refresh, updates scene skills and enabled agents through `src/lib/scenes.ts`, and computes drag-drop reorder payloads via `src/lib/scene-skill-order.ts`. Scene editing is configuration-only; applying toolkits to filesystem targets belongs to Agents or Projects.

## Data Flow

- `useAppContext()` provides repo path, scanned skills, sorted agent inventory, and refresh callbacks.
- `SceneCard` renders each scene and emits edit/configure/reorder actions.
- `src/lib/scenes.ts` carries every Tauri command invocation.

## Interactions

Skill toggles branch on `scene.skillSelectionMode`: new scenes persist explicit selected IDs, while legacy scenes keep their all-except-disabled behavior. The normal scene UI no longer invokes direct scene apply or rewrites global agent configuration.

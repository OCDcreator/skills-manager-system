# Scenes View

> **Source**: `src/views/ScenesView.tsx`
> **Status**: [REVIEW]

## Overview

Page-level scene management view for creating, editing, configuring, applying, and drag-reordering scene skill priorities.

## Public Surface

| Export | Purpose |
|---|---|
| `ScenesView` | Scene management page component. |

## Core Logic

Owns scene CRUD form state, loads `SceneConfigSnapshot` through a deferred initial refresh, updates disabled skills and enabled agents through `src/lib/scenes.ts`, computes drag-drop reorder payloads via `src/lib/scene-skill-order.ts`, and refreshes both scene config and global app context after a scene apply.

## Data Flow

- `useAppContext()` provides repo path, scanned skills, sorted agent inventory, and refresh callbacks.
- `SceneCard` renders each scene and emits edit/configure/reorder actions.
- `src/lib/scenes.ts` carries every Tauri command invocation.

## Interactions

Applying a scene now refreshes both skill state and agent inventory so the rest of the app reflects the new active scene immediately.

# Scenes API Types

> **Source**: `src/lib/scenes.ts`
> **Status**: [DRAFT]

## Overview

TypeScript types and Tauri invoke wrappers for the scene management backend.

## Public Surface

| Export | Purpose |
|---|---|
| `SceneEntry`, `SceneConfigSnapshot` | Scene config types |
| `ApplySceneResult` | Apply result type |
| `getSceneConfig`, `createScene`, etc. | Invoke wrappers |

## Interactions

- `src/views/ScenesView.tsx` — primary consumer
- `src-tauri/src/commands/scenes.rs` — backend counterparts

# Scenes API Types

> **Source**: `src/lib/scenes.ts`
> **Status**: [DRAFT]

## Overview

TypeScript types and Tauri invoke wrappers for the scene management backend.

## Public Surface

| Export | Purpose |
|---|---|
| `SceneEntry`, `SceneConfigSnapshot` | Scene config types (`skillSelectionMode`, `selectedSkillIds`, and `skillOrder` on `SceneEntry`) |
| `ApplySceneResult` | Apply result type |
| `getSceneConfig`, `createScene`, `setSceneSkillOrder`, etc. | Invoke wrappers |

## Interactions

- `src/views/ScenesView.tsx` — primary consumer
- `src-tauri/src/commands/scenes.rs` — backend counterparts

`setSceneSkills` now sends generic scene skill IDs; the Rust scene store decides whether that payload is a selected-skill list or a disabled-skill list based on the persisted scene mode.

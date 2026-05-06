# Scenes API Types

> **Source**: `src/lib/scenes.ts`
> **Status**: [DRAFT]

## Overview

TypeScript types and Tauri invoke wrappers for the scene management backend.

## Public Surface

| Export | Purpose |
|---|---|
| `SceneEntry`, `SceneConfigSnapshot` | Scene config types (`skillSelectionMode`, `selectedSkillIds`, and `skillOrder` on `SceneEntry`) |
| `getSceneConfig`, `createScene`, `setSceneSkillOrder`, etc. | Invoke wrappers |

## Interactions

- `src/views/ScenesView.tsx` — primary consumer
- `src-tauri/src/commands/scenes.rs` — backend counterparts

`setSceneSkills` sends generic scene skill IDs; the Rust scene store decides whether that payload is a selected-skill list or a disabled-skill list based on the persisted scene mode. There is no frontend direct-apply wrapper because scenes are reusable toolkit definitions consumed by Agents or Projects.

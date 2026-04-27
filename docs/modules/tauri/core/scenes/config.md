# Scene Config

> **Source**: `src-tauri/src/core/scenes/config.rs`
> **Status**: [DRAFT]

## Overview

Persists scene configuration (scene list + active scene) to `scene-config.json` in the app config directory. Each scene now carries a backward-compatible skill-selection mode so legacy scenes can keep their "all except disabled" behavior while new scenes start with an explicit empty selection.

## Public Surface

| Export | Purpose |
|---|---|
| `SceneEntry` | A single scene definition |
| `SceneSkillSelectionMode` | Distinguishes legacy all-except-disabled scenes from explicit selected-skill scenes. |
| `SceneConfigSnapshot` | Full config state (scenes map + active ID) |
| `SceneConfigStore` | Persistence layer with CRUD operations |

## Core Logic

`SceneConfigStore` provides atomic CRUD operations that load → mutate → save. Each method returns the full updated snapshot. The store validates scene IDs, keeps active-scene cleanup on deletion, and normalizes skill ID lists before persisting them. New scenes default to `OnlySelected` with zero selected skills; `set_scene_skills` updates either the legacy disabled list or the explicit selected list based on the scene's stored mode, and trims `skill_order` to still-enabled entries.

## Interactions

- `manager.rs` — calls store for applying scenes
- `commands::scenes` — thin wrappers around store methods

## Change Notes

Keeps persistence path handling scoped to `PathBuf`; no runtime path references are required in this module.

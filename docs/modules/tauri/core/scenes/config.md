# Scene Config

> **Source**: `src-tauri/src/core/scenes/config.rs`
> **Status**: [DRAFT]

## Overview

Persists scene configuration (scene list + active scene) to `scene-config.json` in the app config directory. Each scene stores its disabled skill IDs and enabled agent keys.

## Public Surface

| Export | Purpose |
|---|---|
| `SceneEntry` | A single scene definition |
| `SceneConfigSnapshot` | Full config state (scenes map + active ID) |
| `SceneConfigStore` | Persistence layer with CRUD operations |

## Core Logic

`SceneConfigStore` provides atomic CRUD operations that load → mutate → save. Each method returns the full updated snapshot. The store validates scene IDs and handles active scene cleanup on deletion.

## Interactions

- `manager.rs` — calls store for applying scenes
- `commands::scenes` — thin wrappers around store methods

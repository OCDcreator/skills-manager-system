# Scene Commands

> **Source**: `src-tauri/src/commands/scenes.rs`
> **Status**: [DRAFT]

## Overview

Thin Tauri command layer for scene CRUD and application.

## Public Surface

| Command | Purpose |
|---|---|
| `get_scene_config` | Load scene configuration |
| `create_scene` | Create a new scene |
| `update_scene` | Update name/description |
| `delete_scene` | Delete a scene |
| `set_active_scene` | Set active scene ID |
| `set_scene_skills` | Set disabled skill IDs for a scene |
| `set_scene_agents` | Set enabled agent keys for a scene |
| `set_scene_skill_order` | Set skill priority order for a scene |
| `apply_scene` | Apply scene (update skills + agents + set active) |

## Interactions

- `core::scenes::config` — all CRUD operations
- `core::scenes::manager` — apply logic
- `src/lib/scenes.ts` — matching TypeScript wrappers

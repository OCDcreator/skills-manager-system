# Scene Commands

> **Source**: `src-tauri/src/commands/scenes.rs`
> **Status**: [DRAFT]

## Overview

Thin Tauri command layer for scene CRUD plus compatibility handling for the removed direct-apply behavior.

## Public Surface

| Command | Purpose |
|---|---|
| `get_scene_config` | Load scene configuration |
| `create_scene` | Create a new scene |
| `update_scene` | Update name/description |
| `delete_scene` | Delete a scene |
| `set_active_scene` | Set active scene ID |
| `set_scene_skills` | Update scene skill IDs using the scene's stored selection mode |
| `set_scene_agents` | Set enabled agent keys for a scene |
| `set_scene_skill_order` | Set skill priority order for a scene |
| `apply_scene` | Compatibility command that returns the toolkit-only blocked error |

## Interactions

- `core::scenes::config` — all CRUD operations
- `core::scenes::manager` — compatibility error message and result type
- `src/lib/scenes.ts` — matching TypeScript wrappers for scene CRUD/configuration commands

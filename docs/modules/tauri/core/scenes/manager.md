# Scene Manager

> **Source**: `src-tauri/src/core/scenes/manager.rs`
> **Status**: [DRAFT]

## Overview

Applies a scene by updating both skill enable/disable state and agent enabled state to match the scene configuration.

## Public Surface

| Export | Purpose |
|---|---|
| `apply_scene` | Apply a scene, updating skills + agents + active scene ID |
| `ApplySceneResult` | Result summary |

## Core Logic

1. Load scene from config
2. Diff and update skill disabled IDs via `SkillStateStore`
3. Diff and update agent enabled flags via `AgentConfigStore`
4. Set the scene as active

## Interactions

- `core/scenes/config` — scene definitions
- `core/skills/state` — skill enable/disable persistence
- `core/agents/config` + `core/agents/discovery` — agent state

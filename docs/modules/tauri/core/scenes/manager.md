# Scene Manager

> **Source**: `src-tauri/src/core/scenes/manager.rs`
> **Status**: [REVIEW]

## Overview

Applies a scene by updating repo-scoped skill state, global agent enabled flags, and the managed contents inside agent skill directories.

## Public Surface

| Export | Purpose |
|---|---|
| `apply_scene` | Applies one scene and then reruns global agent sync using the saved sync mode. |
| `ApplySceneResult` | Small summary returned to the frontend. |

## Core Logic

1. Load the target scene.
2. Diff and persist disabled skill IDs through `SkillStateStore`.
3. Diff and persist enabled agent flags through `AgentConfigStore`.
4. Load the preferred `AgentSyncMode` from settings, map it to `target_sync::SyncMode`, and rerun `apply_agent_sync`.
5. Mark the scene as active only after sync succeeds.

## Tests

Includes copy-mode and symlink-mode regression tests proving that scene apply now updates managed agent targets instead of only mutating config state.

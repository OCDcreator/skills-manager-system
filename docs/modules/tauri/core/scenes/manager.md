# Scene Manager

> **Source**: `src-tauri/src/core/scenes/manager.rs`
> **Status**: [REVIEW]

## Overview

Applies a scene by updating target-agent enablement, assigning that scene to the selected agents, and then rerunning agent sync.

## Public Surface

| Export | Purpose |
|---|---|
| `apply_scene` | Applies one scene and then reruns agent sync using the saved sync mode. |
| `ApplySceneResult` | Small summary returned to the frontend. |

## Core Logic

Loads the target scene, computes its currently unselected skill count from the live repo scan, persists enabled agent flags through `AgentConfigStore`, replaces enabled agents' selected-scene list with the applied scene ID while preserving direct skill and exclusion lists, maps the saved `AgentSyncMode` to `target_sync::SyncMode`, reruns `apply_agent_sync`, and marks the scene active only after sync succeeds.

## Tests

Includes regressions for explicit-empty new scenes, legacy/selected scene sync behavior, folder-name target entries, and symlink/copy mode application after the per-agent sync redesign.

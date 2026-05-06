# Scene Manager

> **Source**: `src-tauri/src/core/scenes/manager.rs`
> **Status**: [REVIEW]

## Overview

Describes reusable scene toolkit definitions without mutating agent configuration or filesystem targets. The old direct scene apply symbol remains compatibility-only and returns an explicit blocked error.

## Public Surface

| Export | Purpose |
|---|---|
| `describe_scene_toolkit` | Reads one scene and reports skill/agent counts without writing config or targets. |
| `SceneToolkitSummary` | Non-mutating summary of scene ID, name, enabled/disabled skill counts, and agent count. |
| `apply_scene` | Compatibility symbol that always errors with "Scenes are reusable toolkits. Apply them from Agents or Projects." |
| `ApplySceneResult` | Compatibility alias for callers that still type the blocked command result. |

## Core Logic

Loads the target scene, computes its enabled and disabled skill counts from the live repo scan, and returns a summary. It does not call `AgentConfigStore`, agent discovery, target sync, settings, or `set_active_scene`.

## Tests

Includes regressions that the summary helper reports counts without mutation and that compatibility apply returns the blocked error without rewriting agent selection or target folders.

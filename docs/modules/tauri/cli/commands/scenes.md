# CLI Scene Commands

> **Source**: `src-tauri/src/cli/commands/scenes.rs`
> **Status**: [REVIEW]

## Overview

Implements headless scene listing, scene config mutations, active-scene updates, and compatibility handling for the removed direct scene apply path.

## Import Relationships

```text
Upstream: src-tauri/src/cli/mod.rs
Downstream: src-tauri/src/app_runtime/*, src-tauri/src/core/scenes/*
```

## Public Surface

| Export | Purpose |
|---|---|
| `run` | Dispatches one parsed scene subcommand. |
| `apply_with_system_dirs` | Test-only compatibility adapter with injected host dirs; returns the blocked direct-apply error. |

## Core Logic

Config mutations acquire the advisory config lock before touching shared state. The adapter delegates scene business rules to `SceneConfigStore`, including the mode-aware interpretation of `set-skills` payloads for explicit-empty new scenes versus legacy scenes. `scenes apply` no longer syncs targets or switches active global state; it returns `scene_apply_blocked` and tells callers to apply toolkits through Agents or Projects.

## Interactions

Scene-not-found, duplicate-scene, and blocked direct-apply errors are mapped to stable CLI error bodies. Blocked apply does not require an effective repo path because it performs no mutation.

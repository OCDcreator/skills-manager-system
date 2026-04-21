# CLI Scene Commands

> **Source**: `src-tauri/src/cli/commands/scenes.rs`
> **Status**: [REVIEW]

## Overview

Implements headless scene listing, scene config mutations, active-scene updates, and scene application.

## Import Relationships

```text
Upstream: src-tauri/src/cli/mod.rs
Downstream: src-tauri/src/app_runtime/*, src-tauri/src/core/scenes/*
```

## Public Surface

| Export | Purpose |
|---|---|
| `run` | Dispatches one parsed scene subcommand. |
| `apply_with_system_dirs` | Testable apply adapter with injected host dirs. |

## Core Logic

Config mutations and apply operations acquire the advisory config lock before touching shared state. The adapter delegates all scene business rules to `SceneConfigStore` and `apply_scene`.

## Interactions

Scene-not-found and duplicate-scene errors are mapped to stable CLI error bodies; apply requires an effective repo path and current agent system dirs.


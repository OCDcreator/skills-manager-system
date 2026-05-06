# CLI Scene Command Tests

> **Source**: `src-tauri/src/cli/commands/scenes_tests.rs`
> **Status**: [REVIEW]

## Overview

Covers scene config mutations and blocked direct-apply behavior through the CLI adapter layer.

## Import Relationships

```text
Upstream: cargo test --features cli
Downstream: src-tauri/src/cli/commands/scenes.rs, src-tauri/src/core/scenes/*
```

## Public Surface

| Export | Purpose |
|---|---|
| tests | Exercise create/update/delete, active scene, skill/agent/order updates, and blocked direct apply. |

## Core Logic

Tests construct temporary repos with SKILL.md files and injected agent dirs, then assert scene mutations round-trip and `scenes apply` returns `scene_apply_blocked` without rewriting agent config or target folders. The suite also covers that blocked apply does not require a configured repo path.

## Interactions

Keep in sync with `SceneConfigSnapshot` serialization and project-wide advisory lock expectations.

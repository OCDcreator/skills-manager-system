# Skill State Store

> **Source**: `src-tauri/src/core/skills/state.rs`
> **Status**: [REVIEW]

## Overview

Persists repo-scoped skill enable/disable state as app-local JSON keyed by a normalized repository path.

## Import Relationships

```text
Upstream: src-tauri/src/commands/skills.rs
Downstream: serde_json, std::fs
```

## Public Surface

| Export | Purpose |
|---|---|
| `SkillStateSnapshot` | Serializable disabled-ID snapshot returned to the frontend. |
| `SkillStateStore` | Loads and updates repo-scoped persisted skill state. |
| `build_repo_state_key` | Produces a normalized repo bucket key. |
| `normalize_repo_path` | Converts repo paths to forward-slash normalized strings. |

## Core Logic

The store keeps a JSON map of repository buckets and persists only disabled skill IDs, treating all discovered skills as enabled by default.

## Data Flow

Tauri commands resolve the configured repo path, then delegate load and mutation operations into this store.

## Interactions

Must stay aligned with the stable skill IDs produced by `src-tauri/src/core/skills/scan.rs`.

## Configuration

Writes `skill-state.json` under the app config directory.

## Change Notes

Keep scene/project overrides out of this module until those features exist.

# Agent Target Sync Helpers

> **Source**: `src-tauri/src/core/agents/target_sync.rs`
> **Status**: [REVIEW]

## Overview

Owns target-directory reconciliation shared by global agent sync and project-local assignment sync.

## Import Relationships

```text
Upstream: src-tauri/src/core/agents/sync.rs, src-tauri/src/core/projects/sync.rs
Downstream: serde_json, std::fs, src-tauri/src/core/skills/scan.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `SyncMode` | Copy vs symlink deployment mode shared by desktop and CLI-facing sync adapters. |
| `DesiredSkillEntry` | Desired managed target entry derived from a scanned skill. |
| `TargetApplyStats` | Counts writes, removals, and unmanaged conflicts for one target. |
| `build_desired_skill_entries` | Converts selected skills into stable managed target names. |
| `apply_desired_entries` | Reconciles a target directory using `.skills-manager-system-manifest.json`. |
| `cleanup_managed_entries` | Removes only entries previously managed by this app. |
| `load_managed_entry_snapshots` | Exposes target-manifest metadata for read-only target inventory scanning. |
| `is_manifest_file_name` | Lets inventory scanners hide the app manifest from visible target skill lists. |

## Core Logic

This module loads a target-local manifest, removes stale managed entries, protects unmanaged conflicts, and deploys selected skills by copy or symlink. It skips nested `.git` directories so external source metadata is not copied into agent targets.

## Interactions

Global agent sync calls it with discovered global agent directories. Project sync calls it with project-relative agent directories. Ledger ownership stays outside this module so each caller can track its own lifecycle.

## Change Notes

`SyncMode` is now public because the shared feature-split library exposes sync orchestration to both desktop and CLI builds. Keep new sync variants serialized in `snake_case` to match settings and CLI payload expectations.

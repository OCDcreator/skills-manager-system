# External Source Commands

> **Source**: `src-tauri/src/commands/external_sources.rs`
> **Status**: [REVIEW]

## Overview

Hosts the thin Tauri command layer for external GitHub source management.

## Public Surface

| Export | Purpose |
|---|---|
| `list_external_sources` | Returns current source snapshots, optionally enriched with runtime repo integrity warnings. |
| `add_external_source` | Adds or updates a source record, attempts an immediate fetch, then returns the refreshed list. |
| `fetch_external_source` | Refreshes one cached source and returns the refreshed list. |
| `import_external_variant` | Imports one detected variant into the configured repo. |
| `update_external_import` | Re-imports one managed mirror at the latest fetched upstream commit. |
| `remove_external_source` | Removes a source record, optionally cascading import deletion. |
| `repair_external_import` | Rebuilds a managed mirror from cached upstream data. |

## Core Logic

The module resolves `app_config_dir`, loads the configured repo path from `SettingsStore`, enforces repo-path presence only for operations that need a live repo root, and delegates all business rules to `core::external_sources::service`.

## Interactions

Keep this layer thin. Detection, cached git sync, manifest validation, reference checks, and snapshot persistence belong under `src-tauri/src/core/external_sources/`.

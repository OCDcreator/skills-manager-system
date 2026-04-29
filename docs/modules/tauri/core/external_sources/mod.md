# External Sources Core Module Boundary

> **Source**: `src-tauri/src/core/external_sources/mod.rs`
> **Status**: [REVIEW]

## Overview

Declares the external GitHub sources domain and re-exports the subset used by commands and sibling modules.

## Public Surface

| Export | Purpose |
|---|---|
| `detect` | Cached-repo variant detection and layout classification. |
| `git_repo` | URL normalization, cached repo sync, and git reads. |
| `git_tree` | Private git-tree listing helpers used by detection. |
| `imports` | Managed mirror import, removal, and repair lifecycle. |
| `models` | Persisted snapshot, import record, manifest, and warning models. |
| `reference_check` | Cross-domain blocking-reference lookup before destructive removal. |
| `service` | Orchestration layer returned to Tauri commands. |
| `store` | `external-sources.json` persistence. |
| Re-exports | Thin convenience re-exports for commands and sibling domains. |

## Core Logic

This file is still declarative, but the split now makes the internal layering explicit: `service` is the desktop-facing orchestration boundary, `imports` is the managed-mirror lifecycle boundary, and private helpers like `git_tree`, `import_paths`, `mirror_fs`, `source_snapshot`, and `source_sync` keep those larger modules from regressing into monoliths.

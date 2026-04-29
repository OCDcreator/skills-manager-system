# External Source Snapshots

> **Source**: `src-tauri/src/core/external_sources/source_snapshot.rs`
> **Status**: [REVIEW]

## Overview

Builds runtime snapshot data for one persisted external source record.

## Public Surface

| Export | Purpose |
|---|---|
| `load_variants_for_source` | Converts cached-repo detection results into frontend snapshot variants. |
| `runtime_integrity_warning` | Produces late integrity warnings for broken live mirrors inside the repo. |

## Core Logic

This helper keeps snapshot assembly read-only. It loads detected variants only when a cached repo path exists and computes integrity warnings from live repo files instead of persisted snapshot state, so the UI can flag missing `SKILL.md`, unreadable manifests, or mismatched mirror metadata without rewriting stored records first.

## Interactions

Used by `service.rs`. Its manifest validation rules must match `mirror_fs.rs`, `skills::identity`, and the managed-source warning UX in `SkillDetailPanel` and `AgentExternalVariantPanel`.

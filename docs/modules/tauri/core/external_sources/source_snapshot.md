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

This helper keeps snapshot assembly read-only. It loads detected variants only when a cached repo path exists, enriches those variants with parsed `SKILL.md` name/description metadata, and computes integrity warnings from live repo files instead of persisted snapshot state, so the UI can explain what an upstream repository is for while still flagging missing `SKILL.md`, unreadable manifests, or mismatched mirror metadata without rewriting stored records first.

When `lastFetchedCommit` is available on the source record, variant detection and metadata are read from git objects through `git_tree.rs` instead of the worktree. The startup/listing path must not resolve remote `HEAD` or otherwise touch the network; if no persisted commit is available, the helper falls back to reading on-disk files.

## Interactions

Used by `service.rs`. Its manifest validation rules must match `mirror_fs.rs`, `skills::identity`, and the managed-source warning UX in `SkillDetailPanel` and `AgentExternalVariantPanel`.

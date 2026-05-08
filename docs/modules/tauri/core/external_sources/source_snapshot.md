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

This helper keeps snapshot assembly read-only. It loads detected variants only when a cached repo path exists, enriches those variants with parsed `SKILL.md` name/description metadata, forwards detection metadata used by the manual-import UI, derives direct child directory and file previews from either git objects or the fallback worktree, and attaches per-variant content fingerprints so the UI can reveal variants that are byte-for-byte equivalent. It also computes integrity warnings from live repo files instead of persisted snapshot state, so the UI can explain what an upstream repository is for while still flagging missing `SKILL.md`, unreadable manifests, or mismatched mirror metadata without rewriting stored records first.

When `lastFetchedCommit` is available on the source record, variant detection, metadata, previews, and fingerprints are read from git objects instead of the worktree. The stored source `subpath` is passed back into detection so listing/fetch projections agree about which subtree is importable. The startup/listing path must not resolve remote `HEAD` or otherwise touch the network; if no persisted commit is available, the helper falls back to reading on-disk files under the same selected subpath.

## Interactions

Used by `service.rs`. Its manifest validation rules must match `mirror_fs.rs`, `skills::identity`, and the managed-source warning UX in `SkillDetailPanel` and `AgentExternalVariantPanel`.

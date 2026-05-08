# External Source Variant Fingerprints

> **Source**: `src-tauri/src/core/external_sources/variant_fingerprint.rs`
> **Status**: [REVIEW]

## Overview

Builds stable per-variant content fingerprints for external-source snapshots.

## Public Surface

| Export | Purpose |
|---|---|
| `load_variant_fingerprint` | Returns a content fingerprint for a variant from the pinned git commit or, when no commit is available, from the cached worktree. |

## Core Logic

When a fetched commit is available, the module reads only the git tree's relative file paths and blob object IDs so snapshot reads stay pinned to the same upstream commit used for variant detection without spawning one `git cat-file` process per file. The worktree fallback is only for sources without a persisted commit; it recursively hashes files under the variant directory, normalizes separators, ignores `.git` internals and `.skills-manager-source.json`, and serializes sorted relative-path plus file-hash pairs before hashing the whole set.

These fingerprints are comparison hints for the UI. They are intentionally separate from the import/update fingerprint stored on managed mirrors, and they do not change import identity, target selection, or update detection, which remain owned by the import and source-sync modules.

## Interactions

Used by `source_snapshot.rs` while building `ExternalVariantSnapshot.contentFingerprint`. It shares hash semantics with `git_repo.rs` and `git_export.rs` so an imported mirror and a listed variant can be compared using the same file-set boundary.

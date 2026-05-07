# External Source Sync

> **Source**: `src-tauri/src/core/external_sources/source_sync.rs`
> **Status**: [REVIEW]

## Overview

Owns source-record mutation, cached fetch, and import refresh logic for external sources.

## Public Surface

| Export | Purpose |
|---|---|
| `SourceUpsertInput` | Internal add/update boundary carrying `repoUrl` plus optional requested `branch` and repo-relative `subpath`. |
| `upsert_source` | Adds or updates a normalized source record and returns its stable id. |
| `set_source_failure` | Persists a fetch failure onto the source record without panicking the service layer. |
| `fetch_source` | Refreshes the cached repo, detection result, warnings, and import update flags. |
| `cache_repo_absolute_path` | Resolves the expected cache location for one source id. |

## Core Logic

The module normalizes GitHub URLs into stable `src_<hash>` ids, so duplicate/upsert identity remains based on the canonical repo URL only. Upsert stores trimmed optional branch values and validates optional subpaths through the shared repo-relative path canonicalizer; traversal, rooted, or drive-qualified paths are rejected before the snapshot is written.

Fetch keeps cached repos under `config/external-sources/<id>/repo`, still resolves and stores `defaultBranch` for diagnostics, then reads `lastFetchedCommit` from the requested branch when one is stored or from the default branch otherwise. Detection runs against the fetched commit and receives the stored repo-relative `subpath`, so both supported bundle rules and the generic `SKILL.md` fallback scan the selected subtree rather than always scanning the repository root. After every fetch, the module recalculates per-import `updateAvailable` plus `variant_disappeared` warnings.

Refresh matching now treats `upstream_variant_path` as the durable upstream identity. That means a generic fallback candidate can be manually imported into any supported local target bucket, and later fetches will still recognize it as present as long as the upstream path still exists. If no variants are detected, fetch records a `no_importable_skills` warning that names either the configured subpath or the repository root, so an empty source does not look silently healthy. Source status is derived from warning severity, not from a separate state machine.

## Interactions

`service.rs` uses this as its write-side helper, while `source_snapshot.rs` handles read-side projection. Keep the warning and cache-path rules aligned with `detect.rs`, `git_repo.rs`, and persisted models.

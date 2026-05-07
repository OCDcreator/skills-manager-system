# External Source Sync Tests

> **Source**: `src-tauri/src/core/external_sources/source_sync_tests.rs`
> **Status**: [REVIEW]

## Overview

Focused unit tests for external source upsert normalization and branch-aware fetch orchestration.

## Public Surface

| Export | Purpose |
|---|---|
| `upsert_source_updates_branch_and_subpath_for_existing_repo_url_identity` | Confirms canonical repo URL identity updates the existing record and persists normalized branch/subpath values. |
| `upsert_source_rejects_traversing_subpath` | Confirms invalid repo-relative subpaths are rejected before writing `external-sources.json`. |
| `fetch_source_reads_requested_branch_commit_and_stores_default_branch` | Confirms fetch reads the requested branch commit, persists the discovered default branch, and records an empty-source warning when nothing importable is detected. |
| `fetch_source_passes_configured_subpath_to_detection` | Confirms fetch detection receives the persisted repo-relative subpath and names it in the empty-source warning. |
| `fetch_source_keeps_manual_target_mapping_when_generic_candidate_still_exists` | Confirms refresh does not mark a manually targeted generic candidate as disappeared when the upstream variant path still exists. |

## Core Logic

The tests use the real store and config lock for upsert behavior. Fetch orchestration uses a small recording runtime from `source_sync.rs`, which avoids network Git operations while proving the branch passed to `read_head_commit`, the subpath passed to detection, the commit stored as `lastFetchedCommit`, the warning/status produced for a source that fetches successfully but exposes no importable variants, and the refresh semantics for generic fallback candidates that were imported into a manually chosen agent bucket.

## Interactions

Keep these tests aligned with `source_sync.rs`, `models.rs`, and the shared repo-relative path canonicalizer in `core::skills::identity`.

# External Source Git Export

> **Source**: `src-tauri/src/core/external_sources/git_export.rs`
> **Status**: [REVIEW]

## Overview

Exports one upstream variant tree from git object data into a staging directory.

## Public Surface

| Export | Purpose |
|---|---|
| `export_variant_from_git` | Materializes one variant at a pinned git ref and returns its content fingerprint. |

## Core Logic

The helper reads blob entries directly from git instead of the worktree, recreates the variant tree under a destination directory, skips `.skills-manager-source.json` when computing the returned fingerprint, and hashes the exported files in sorted relative-path order. That keeps import/update flows stable across line-ending or worktree drift.

## Interactions

Used by `imports.rs` during staging. Its fingerprint contract must stay aligned with update detection in `git_repo.rs` and the pinned-variant metadata stored in `ImportedExternalSkillRecord`.

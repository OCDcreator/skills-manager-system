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

The helper reads blob entries directly from Git instead of the worktree, using `git_command.rs` for repository-scoped process creation. It recreates the variant tree under a destination directory, skips `.skills-manager-source.json` when computing the returned fingerprint, and hashes the exported files in sorted relative-path order. Variant path `.` is treated as the repository root so generic root skill repositories can import `SKILL.md` plus sibling assets without checking out the cache worktree. That keeps import/update flows stable across line-ending or worktree drift, while spawn failures report the attempted Git candidates without altering nonzero Git stderr behavior.

## Interactions

Used by `imports.rs` during staging. Its fingerprint contract must stay aligned with update detection in `git_repo.rs` and the pinned-variant metadata stored in `ImportedExternalSkillRecord`.

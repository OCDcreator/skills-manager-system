# External Source Git Repository Helpers

> **Source**: `src-tauri/src/core/external_sources/git_repo.rs`
> **Status**: [REVIEW]

## Overview

Owns GitHub URL normalization, cached-repo bootstrap/sync, and git-object reads used by the external-source pipeline.

## Public Surface

| Export | Purpose |
|---|---|
| `normalize_github_repo_url` | Normalizes supported SSH/HTTPS GitHub remotes into a stable `github.com/owner/repo` form. |
| `ensure_cached_repo` | Creates or refreshes the cached clone for one source id. |
| `read_default_branch` | Resolves the source repository's default branch. |
| `read_head_commit` | Reads the current commit for a named branch. |
| `fingerprint_variant_at_ref` | Builds a stable content fingerprint for one variant path at a git ref. |

## Core Logic

The module validates source ids, keeps cached repos under the external-source cache root, and uses low-level git commands like `ls-tree` and `show` to inspect variant contents without checking them out into the user's repo. Variant fingerprinting accepts the special `.` root variant used by generic root skill repositories and hashes repo-root blob paths directly. Repository-scoped Git process setup now goes through `git_command.rs`, which centralizes noninteractive Git environment defaults, shared command resolution, and spawn-error mapping while leaving cached-repo behavior here.

## Interactions

Used by `service.rs` for fetch/update checks and by `imports.rs` for staged exports. URL normalization must stay consistent with source-id generation and mirror-path slug generation.

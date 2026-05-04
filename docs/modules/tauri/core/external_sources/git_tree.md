# External Source Git Tree Helpers

> **Source**: `src-tauri/src/core/external_sources/git_tree.rs`
> **Status**: [REVIEW]

## Overview

Provides git-object-backed tree listing helpers for external-source detection.

## Public Surface

| Export | Purpose |
|---|---|
| `list_direct_child_skill_dirs_at_ref` | Lists direct child skill directories under one root at a git ref. |
| `list_recursive_skill_dirs_at_ref` | Lists all nested skill directories under one scan root at a git ref. |
| `read_text_file_at_ref` | Reads one UTF-8 text file from a git ref without checking out the worktree. |
| `skill_dir_exists_at_ref` | Checks whether a scan root itself contains `SKILL.md` at a git ref. |

## Core Logic

The module reads `git ls-tree -r --full-tree -z` output for one prefix through `git_command.rs`, filters for `SKILL.md` blobs, and returns canonical repo-relative parent directories. Direct-child mode only accepts `<root>/<skill>/SKILL.md`, while recursive mode keeps deeper matches so `detect.rs` can flag unsupported nested layouts. The special root scan path `.` is accepted for generic repository detection and treats direct children as repo-root children. `read_text_file_at_ref()` and `skill_dir_exists_at_ref()` use `git show` against the same fetched object graph so other read-only consumers can extract metadata from upstream `SKILL.md` files without relying on a populated cache worktree. Spawn failures include the resolver's attempted Git candidates while nonzero Git stderr remains the operation error.

## Interactions

Used by `detect.rs` and `source_snapshot.rs`. Its path parsing rules must stay aligned with `core/skills/identity.rs`, the generated-layout rule table, and any future import or warning rules that rely on git-tree paths rather than a populated worktree.

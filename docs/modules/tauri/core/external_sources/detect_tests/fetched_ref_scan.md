# Detect Fetched Ref Scan Tests

> **Source**: `src-tauri/src/core/external_sources/detect_tests/fetched_ref_scan.rs`
> **Status**: [REVIEW]

## Overview

Exercises git-ref-backed detection against cached repositories whose worktrees are intentionally empty.

## Public Surface

This file is test-only and exports no production API.

## Core Logic

The test builds a bare remote plus cached repo, pushes a generated layout, then confirms detection reads the fetched commit through git objects rather than depending on a checked-out worktree. This guards the read-only cached-repo contract that the listing flow relies on.

## Interactions

Loaded by `detect_tests.rs`. Keep it aligned with `detect.rs`, `git_repo.rs`, and `git_tree.rs`, because those modules together define how cached external repositories are scanned after fetch.

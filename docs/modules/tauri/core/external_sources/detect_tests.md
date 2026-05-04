# External Source Detection Tests

> **Source**: `src-tauri/src/core/external_sources/detect_tests.rs`
> **Status**: [REVIEW]

## Overview

Regression tests for external-source detection behavior.

## Public Surface

This file is test-only and exports no production API.

## Core Logic

The tests cover three behavior groups:

- legacy generated bundle roots under `dist/agents/...`
- additional aligned agent roots under `.cursor`, `.gemini`, `.github`, and `.kiro` plus their `dist/*` equivalents
- warnings for unsupported nested or unknown agent layouts
- git-ref-backed detection for fetched cache repos whose worktrees are empty, plus support for root-level hidden agent roots such as `.agents/skills`
- generic skill repository fallback detection for repo-root `SKILL.md`, direct child skills under a configured subpath, neutral `skill_repository` variant keys, and generated-bundle precedence over generic variants

## Interactions

Loaded by `detect.rs` through a test-only path module declaration. The scenarios intentionally overlap with `git_repo.rs` cache-fetch behavior and `git_tree.rs` path parsing because those modules together define whether upstream repositories like `impeccable` appear importable in the UI.

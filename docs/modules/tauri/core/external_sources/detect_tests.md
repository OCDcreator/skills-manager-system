# External Source Detection Tests

> **Source**: `src-tauri/src/core/external_sources/detect_tests.rs`
> **Status**: [REVIEW]

## Overview

Regression-test harness and shared helpers for external-source detection behavior.

## Public Surface

This file is test-only and exports no production API.

## Core Logic

The parent file now stays intentionally small and provides shared fixtures for the split test modules under `detect_tests/`. It keeps the Git helper setup plus `create_skill_dir`, while scenario groups live in separate files for generated layouts, recursive generic fallback, and fetched-ref scans so the detection regression coverage can grow without leaving one 400-line catch-all test file.

## Interactions

Loaded by `detect.rs` through a test-only path module declaration. The child modules intentionally overlap with `git_repo.rs` cache-fetch behavior and `git_tree.rs` path parsing because those modules together define whether upstream repositories like `impeccable` appear importable in the UI.

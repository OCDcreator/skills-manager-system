# Module: `src-tauri/src/core/projects/store_tests.rs`

## Purpose

Regression tests for project assignment persistence.

## Responsibilities

- verifies missing-store defaults, add/update/remove round-trips, and duplicate-path rejection
- covers portable path normalization for project keys
- covers legacy Windows project-key migration, including duplicate legacy entries that normalize to the same path

## Interactions

Exercises `store.rs` through the public `ProjectConfigStore` API and shares the same domain boundary as the project assignment store.

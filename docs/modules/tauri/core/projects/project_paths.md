# Project Path Helpers

> **Source**: `src-tauri/src/core/projects/project_paths.rs`
> **Status**: [REVIEW]

## Overview

Shared project-path normalization helpers reused by persistence and read-only inspection.

## Responsibilities

- trims incoming project paths
- rejects empty paths
- rejects non-absolute paths
- persists paths with `/` separators and without redundant trailing separators
- provides one normalization boundary so store and inspection stay aligned

## Interactions

Delegates the string format to `platform_paths`, then project store, inspection, and sync reuse the normalized value as the stable project key.

# Project Path Helpers

> **Source**: `src-tauri/src/core/projects/project_paths.rs`
> **Status**: [REVIEW]

## Overview

Shared project-path normalization helpers reused by persistence and read-only inspection.

## Responsibilities

- trims incoming project paths
- rejects empty paths
- rejects non-absolute paths
- provides one normalization boundary so store and inspection stay aligned

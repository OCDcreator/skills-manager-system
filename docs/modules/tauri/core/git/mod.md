# Git Core Module

> **Source**: `src-tauri/src/core/git/mod.rs`
> **Status**: [DRAFT]

## Overview

Barrel module for the git subsystem, exposing operation helpers, response types, and test support.

## Public Surface

| Export | Purpose |
|---|---|
| `operations` | Git CLI operations and parsing logic |
| `types` | Serde response DTOs shared by commands and operations |

## Export Semantics

Git command execution and parsing live in `operations.rs`; shared response DTOs live in `types.rs` so the operation file stays below architecture warning limits. Unit tests are compiled through a test-only sibling module.

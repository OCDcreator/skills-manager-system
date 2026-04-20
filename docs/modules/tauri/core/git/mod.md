# Git Core Module

> **Source**: `src-tauri/src/core/git/mod.rs`
> **Status**: [DRAFT]

## Overview

Barrel module for the git subsystem, re-exporting the `operations` sub-module.

## Public Surface

| Export | Purpose |
|---|---|
| `operations` | Git CLI operations and response types |

## Export Semantics

Single public sub-module. All git logic lives in `operations.rs`.

# App Runtime Module

> **Source**: `src-tauri/src/app_runtime/mod.rs`
> **Status**: [REVIEW]

## Overview

Defines the shared runtime boundary for headless CLI work: config-dir resolution, repo overrides, config locking, output shaping, and re-exported helper types.

## Import Relationships

```text
Upstream: src-tauri/src/lib.rs, src-tauri/src/cli/*
Downstream: src-tauri/src/app_runtime/config_lock.rs, src-tauri/src/app_runtime/context.rs, src-tauri/src/app_runtime/output.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `config_lock` | Cross-platform advisory config lock used by mutation-ready runtime paths. |
| `context` | Runtime context, path normalization, and config-lock helpers. |
| `output` | Stable CLI response schema, warnings, rendering, and result builders. |
| `output_errors` | Exit-code and structured error helpers for CLI adapters. |
| Re-exports | Keeps CLI modules importing from one shared runtime boundary. |

## Core Logic

This module stays intentionally thin. Its main job is to expose a stable runtime surface without turning the CLI into a second business-logic center.

## Interactions

Must stay aligned with the CLI feature split in `src-tauri/Cargo.toml` and the CLI dispatch layer in `src-tauri/src/cli/mod.rs`.

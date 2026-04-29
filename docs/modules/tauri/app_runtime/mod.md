# App Runtime Module

> **Source**: `src-tauri/src/app_runtime/mod.rs`
> **Status**: [REVIEW]

## Overview

Defines shared runtime helpers reused by both desktop and CLI code paths.

## Public Surface

| Export | Purpose |
|---|---|
| `atomic_write` | Atomic text-file persistence helper. |
| `config_lock` | Cross-platform advisory config lock used by mutating runtime paths. |
| `context` | Runtime context and output-path normalization helpers. |
| `output` / `output_errors` | CLI result and exit-shaping helpers. |
| Re-exports | Keeps consumers importing from one runtime boundary. |

## Core Logic

This is still an aggregation module, but it now also re-exports `write_text_atomic` so config stores can combine config locking with atomic write-rename persistence without reaching into a deeper runtime path.

## Interactions

Must stay aligned with the CLI feature split in `src-tauri/src/lib.rs` and any config store that depends on the shared lock-and-write helpers.

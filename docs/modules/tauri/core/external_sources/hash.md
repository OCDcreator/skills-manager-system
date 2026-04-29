# External Source Hash Helper

> **Source**: `src-tauri/src/core/external_sources/hash.rs`
> **Status**: [REVIEW]

## Overview

Provides the SHA-256 helper used for stable ids, path deconfliction, and variant fingerprints.

## Public Surface

| Export | Purpose |
|---|---|
| `sha256_hex` | Returns the lowercase hex digest for a byte slice. |

## Interactions

Shared by `imports.rs` and `git_repo.rs`. Keep it tiny and deterministic; higher-level fingerprint serialization rules belong in those callers.

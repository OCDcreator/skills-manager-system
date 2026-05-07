# CLI Git Command Tests

> **Source**: `src-tauri/src/cli/commands/git_tests.rs`
> **Status**: [REVIEW]

## Overview

Provides focused CLI adapter coverage for git read commands, commit, remote operations, and sync-script execution.

## Import Relationships

```text
Upstream: cargo test --features cli
Downstream: src-tauri/src/cli/commands/git.rs, system git
```

## Public Surface

| Export | Purpose |
|---|---|
| tests | Exercise git command adapters against temporary repositories and local bare remotes. |

## Core Logic

Tests configure throwaway git repos with local identity settings, avoiding network remotes while still verifying fetch/pull/push semantics. Platform-specific sync coverage checks the actual script entrypoint used by the current OS: `update.sh` on Unix and `update.ps1` on Windows.

## Interactions

Requires the system `git` binary on PATH, matching the core git operation dependency.

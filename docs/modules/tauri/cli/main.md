# CLI Binary Entrypoint

> **Source**: `src-tauri/src/cli/main.rs`
> **Status**: [REVIEW]

## Overview

Provides the explicit `skills-manager` binary entrypoint for CLI-only builds.

## Import Relationships

```text
Upstream: Cargo `[[bin]]` target `skills-manager`
Downstream: src-tauri/src/cli/mod.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `main` | Parses arguments, renders the selected response, and exits with the mapped status code. |

## Core Logic

The entrypoint intentionally stays minimal: parse with `clap`, run the shared CLI dispatcher, print JSON or pretty output, and exit with the shared code.

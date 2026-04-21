# CLI Args

> **Source**: `src-tauri/src/cli/args.rs`
> **Status**: [REVIEW]

## Overview

Defines the `clap` derive command tree for the Phase 1a read-only CLI surface.

## Import Relationships

```text
Upstream: src-tauri/src/cli/main.rs, src-tauri/src/cli/mod.rs
Downstream: clap
```

## Public Surface

| Export | Purpose |
|---|---|
| `CliArgs` | Root parser with global output/config/repo flags. |
| `RootCommand` | Top-level command-group selector. |
| `SettingsCommand` | Read-only settings subcommands. |
| `SkillsCommand` | Read-only skill query subcommands. |
| `AgentsCommand` | Read-only agent inventory subcommands. |

## Core Logic

The parser keeps JSON as the default mode, allows `--pretty` as the human-readable escape hatch, and exposes label helpers so the runtime/output layer can emit stable `command` strings.

## Interactions

New CLI subcommands should extend these enums first, then add matching dispatch and module docs in the corresponding command adapters.

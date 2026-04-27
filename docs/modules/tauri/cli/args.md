# CLI Args

> **Source**: `src-tauri/src/cli/args.rs`
> **Status**: [REVIEW]

## Overview

Defines the root `clap` parser, global flags, and top-level command groups for the full CLI surface.

## Import Relationships

```text
Upstream: src-tauri/src/cli/main.rs, src-tauri/src/cli/mod.rs
Downstream: clap, src-tauri/src/cli/command_groups.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `CliArgs` | Root parser with global output/config/repo flags. |
| `RootCommand` | Top-level settings/skills/agents/scenes/projects/git selector. |
| command re-exports | Leaf command enums re-exported from `command_groups.rs` for adapter imports. |

## Core Logic

The parser keeps JSON as the default mode, allows `--pretty` as the human-readable escape hatch, and exposes label helpers so the runtime/output layer can emit stable `command` strings. `projects apply` intentionally has no path argument, and `scenes set-skills` stays generic so the runtime can map the provided skill IDs through the scene's stored selection mode.

## Interactions

New CLI command groups should extend `RootCommand`; new leaf subcommands belong in `command_groups.rs`, then need matching dispatch, adapter tests, and module docs.

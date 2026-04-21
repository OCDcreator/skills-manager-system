# CLI Command Groups

> **Source**: `src-tauri/src/cli/command_groups.rs`
> **Status**: [REVIEW]

## Overview

Defines the leaf `clap` subcommand enums for settings, skills, agents, scenes, projects, and git.

## Import Relationships

```text
Upstream: src-tauri/src/cli/args.rs, src-tauri/src/cli/commands/*
Downstream: clap, std::path
```

## Public Surface

| Export | Purpose |
|---|---|
| `SettingsCommand` / `SkillsCommand` / `AgentsCommand` | Settings, skill, and agent query/mutation leaves. |
| `ScenesCommand` / `ProjectsCommand` / `GitCommand` | Scene, project, and git command-group leaves. |
| `label` methods | Stable command names mirrored into CLI JSON responses. |

## Core Logic

This module owns argument shape only. It does not touch runtime state or core stores; adapters in `commands/` interpret parsed values.

## Interactions

Any new enum variant must be routed in `cli/mod.rs`, implemented under `cli/commands/`, and documented with matching module docs.


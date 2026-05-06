# CLI Project Agent Args

> **Source**: `src-tauri/src/cli/commands/project_agent_args.rs`
> **Status**: [REVIEW]

## Overview

Converts project CLI per-agent layer flags into the same `ProjectAgentAssignment` map used by Tauri project commands.

## Import Relationships

```text
Upstream: src-tauri/src/cli/commands/projects.rs
Downstream: src-tauri/src/core/projects/store.rs
```

## Public Surface

| Export | Purpose |
|---|---|
| `build_layered_agent_map` | Parses repeated `AGENT=VALUE` flags and returns an optional per-agent project-layer map. |

## Core Logic

The helper activates only when project-layer flags are present. It keeps legacy `--agent` plus `--skill` input as direct skills for those declared agents, then layers `--agent-skill`, `--agent-scene`, and `--agent-exclude` entries onto the addressed agents.

## Interactions

Malformed pairs return argument-style errors before any project assignment is mutated. Final deduplication and unsupported-agent filtering remain owned by `ProjectConfigStore`.

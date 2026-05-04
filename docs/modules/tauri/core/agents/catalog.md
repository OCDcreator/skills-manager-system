# Agent Catalog

> **Source**: `src-tauri/src/core/agents/catalog.rs`
> **Status**: [REVIEW]

## Overview

Defines the built-in agent catalog covering all supported AI coding tools. Currently 12 agents: Codex, Claude Code, OpenCode, Cursor, Amp, Kilo Code, Kimi Code CLI, Roo Code, Goose, Gemini CLI, GitHub Copilot, Windsurf.

## Import Relationships

```text
Upstream: src-tauri/src/core/agents/config.rs, src-tauri/src/core/agents/discovery.rs
Downstream: none
```

## Public Surface

| Export | Purpose |
|---|---|
| `AgentCatalogEntry` | Static metadata describing one supported agent. |
| `agent_catalog` | Returns the built-in phase-three catalog. |
| `find_agent` | Looks up a supported agent key. |
| `project_skills_dir_rule` | Returns the project-local skills directory rule for an agent. |

## Core Logic

Stores the stable definitions for all supported agents, including display name plus global skills-dir/detect-dir rules. `project_skills_dir_rule` separates project-local overlays from global config roots, so OpenCode uses `.config/opencode/skills` globally but `.opencode/skills` inside projects, and Cursor uses `.cursor/skills` inside projects instead of sharing Claude Code's manifest directory. Kimi Code CLI resolves from the hidden home-directory pair `.kimi` and `.kimi/skills`. Goose and GitHub Copilot have project-only skills directories.

## Data Flow

Discovery uses the catalog to assemble inventory. Config validation uses it to reject unsupported keys.

## Interactions

Catalog additions must be explicit and deliberate. Each entry must define valid skills-dir and detect-dir rules.

## Configuration

None.

## Change Notes

Do not turn this into a dynamic adapter registry. Scene management uses agent keys from this catalog.

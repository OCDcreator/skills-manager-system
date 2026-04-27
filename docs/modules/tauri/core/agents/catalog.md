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

## Core Logic

Stores the stable definitions for all supported agents, including display name plus skills-dir/detect-dir rules. Cursor shares `.claude/skills` with Claude Code. Kimi Code CLI resolves from the hidden home-directory pair `.kimi` and `.kimi/skills`. Goose and GitHub Copilot have project-only skills directories.

## Data Flow

Discovery uses the catalog to assemble inventory. Config validation uses it to reject unsupported keys.

## Interactions

Catalog additions must be explicit and deliberate. Each entry must define valid skills-dir and detect-dir rules.

## Configuration

None.

## Change Notes

Do not turn this into a dynamic adapter registry. Scene management uses agent keys from this catalog.

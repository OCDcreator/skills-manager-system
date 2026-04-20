# Agent Catalog

> **Source**: `src-tauri/src/core/agents/catalog.rs`
> **Status**: [REVIEW]

## Overview

Defines the small built-in agent catalog for phase-three global sync.

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

Stores the stable definitions for `codex`, `claude_code`, and `opencode`, including display name plus skills-dir/detect-dir rules.

## Data Flow

Discovery uses the catalog to assemble inventory. Config validation uses it to reject unsupported keys.

## Interactions

Phase-three scope is intentionally fixed to three agents. Additions here should remain explicit and deliberate.

## Configuration

None.

## Change Notes

Do not turn this into a dynamic adapter registry in this phase.

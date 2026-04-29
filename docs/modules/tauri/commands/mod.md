# Commands Module Boundary

> **Source**: `src-tauri/src/commands/mod.rs`
> **Status**: [REVIEW]

## Overview

Declares the Rust command submodules exposed to the Tauri desktop layer.

## Public Surface

| Export | Purpose |
|---|---|
| `agent_targets` | Explicit per-entry management commands for global agent skill directories. |
| `agents` | Agent inventory, config, and sync commands. |
| `assistant` | Project assistant commands. |
| `external_sources` | External GitHub source and managed-import commands. |
| `git` | Git status and mutation commands. |
| `projects` | Project assignment commands. |
| `scenes` | Scene configuration commands. |
| `settings` | Settings commands. |
| `skills` | Skill browse and enable-state commands. |
| `terminal` | Project Assistant terminal runtime commands. |

## Core Logic

This file remains aggregation-only. It now also exposes the dedicated terminal
command module alongside the earlier assistant, git, skills, scene, project,
and external-source surfaces.

## Interactions

Must include any module referenced by `tauri::generate_handler!` in `src-tauri/src/lib.rs`.

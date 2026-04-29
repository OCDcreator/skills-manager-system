# Core Module Boundary

> **Source**: `src-tauri/src/core/mod.rs`
> **Status**: [REVIEW]

## Overview

Declares backend core domains used by Tauri command modules.

## Public Surface

| Export | Purpose |
|---|---|
| `agents` | Agent inventory and sync domain. |
| `assistant` | Project assistant domain. |
| `external_sources` | External GitHub source detection, caching, imports, and persistence. |
| `git` | Git operations domain. |
| `projects` | Project assignment domain. |
| `scenes` | Scene configuration domain. |
| `settings` | Settings persistence domain. |
| `skills` | Skill scanning, identity, document reading, and state domain. |
| `terminal` | Embedded assistant terminal launch and PTY session domain. |

## Core Logic

This file stays declarative. It now exposes `terminal` as a first-class core
domain so the embedded assistant PTY runtime stays separate from the older
retrieval-oriented assistant code.

## Interactions

Must expose any new core domain before the command layer can depend on it.

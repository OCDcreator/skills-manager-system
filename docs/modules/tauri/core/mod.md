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

## Core Logic

This file stays declarative. Task 7 adds `external_sources` as a first-class core domain rather than threading the feature through existing skills or agents modules.

## Interactions

Must expose any new core domain before the command layer can depend on it.

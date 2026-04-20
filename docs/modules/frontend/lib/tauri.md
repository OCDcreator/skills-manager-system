# Tauri API Wrapper

> **Source**: `src/lib/tauri.ts`
> **Status**: [REVIEW]

## Overview

Defines TypeScript shapes and thin wrappers for the Tauri commands used by the frontend.

## Import Relationships

```text
Upstream: src/context/AppContext.tsx, UI type consumers
Downstream: @tauri-apps/api/core, src-tauri/src/commands/*
```

## Public Surface

| Export | Purpose |
|---|---|
| `SkillSummary` | Frontend shape for scan results. |
| `ScanSkillsResponse` | Response payload for `scan_skills`. |
| `SkillDocument` | Frontend shape for a loaded `SKILL.md`. |
| `SkillStateSnapshot` | Frontend shape for repo-scoped disabled skill IDs. |
| `getRepoPath` | Invokes `get_repo_path`. |
| `setRepoPath` | Invokes `set_repo_path`. |
| `scanSkills` | Invokes `scan_skills`. |
| `getSkillDocument` | Invokes `get_skill_document`. |
| `getSkillState` | Invokes `get_skill_state`. |
| `setSkillEnabled` | Invokes `set_skill_enabled`. |

## Core Logic

Each function delegates directly to `invoke` with the command name and required payload, keeping frontend command wiring thin.

## Data Flow

Frontend context calls these wrappers; Tauri serializes responses from Rust commands back into the declared TypeScript shapes.

## Interactions

Command names and payload keys must stay aligned with `src-tauri/src/commands/settings.rs`, `src-tauri/src/commands/skills.rs`, and `tauri::generate_handler!`.

## Configuration

None.

## Change Notes

When adding a Tauri command, update Rust implementation, handler registration, TypeScript wrapper/types, and the consuming frontend state together.

# Project Draft Helper

> **Source**: `src/lib/project-draft.ts`
> **Status**: [REVIEW]

## Overview

Pure helper module for the Projects workbench draft state.

## Public Surface

| Export | Purpose |
|---|---|
| `ProjectDraft` | Create/edit draft shape including `sourceProjectPath`, `displayNameManuallyEdited`, and `unsupportedAgentKeys`. |
| `suggestProjectDisplayName` | Derives a default display name from the path basename. |
| `applyProjectPathToDraft` | Updates `projectPath` and keeps `displayName` synced to the suggestion until the user edits it manually. |
| `applyProjectDisplayNameToDraft` | Applies an explicit display name edit and locks future path-sync auto-fill. |
| `isProjectDraftDirty` | Compares the active draft against create or edit baselines. |
| `ProjectSkillSelectionFilter` | Skill selected-state filter union: `all`, `selected`, `unselected`. |
| `filterProjectSkills` / `filterProjectAgents` | Client-side search helpers with path, selected-state, and enabled-state filters. |
| `sortProjectAgentsForEditor` | Stable selected-first ordering helper for edit-mode agent lists. |
| `projectDraftToAgentAssignments` | Serializes per-agent project layers for Tauri save commands. |

## Core Logic

The module still keeps create/edit draft behavior pure, but its filter helpers now match the workbench's more precise editor controls. Skills can be filtered by free-text search, top-level relative-path buckets such as `custom` or `external`, an external group filter when the external bucket is active, and selected-state (`selected` / `unselected`), while agents can be filtered by query plus `enabled` / `disabled` state.

`sortProjectAgentsForEditor` is intentionally narrow: it only reorders the already-filtered list, pushing currently configured agents to the top during edit mode without changing the underlying persisted global agent order. Preview derivation lives in `src/lib/project-summary.ts` so this module stays centered on draft mutation and filtering.

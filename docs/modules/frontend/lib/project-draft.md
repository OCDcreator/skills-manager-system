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
| `removeProjectManagedTargetSkill` | Cancels a managed project target skill by removing the direct pick and adding a project-local exclusion. |
| `projectDraftToAgentAssignments` | Serializes per-agent project layers for Tauri save commands. |

## Core Logic

The module keeps create/edit draft behavior pure: path-derived names, dirty-state comparison, per-agent layer mutation, and serialization back to the Tauri DTO shape. Editor list filtering lives in `src/lib/project-filters.ts` so draft mutation does not also own view-facing search and sort rules.

`removeProjectManagedTargetSkill` is used by the target inventory management UI for entries that the app recognizes as managed. It does not delete filesystem content immediately; it edits the draft so the next save/apply stops selecting that project-local skill and removes it from the target during reconciliation. Re-selecting the skill through the normal toggle clears the exclusion again.

Preview derivation lives in `src/lib/project-summary.ts` so this module stays centered on draft mutation.

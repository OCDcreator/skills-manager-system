# Project Draft Helper

> **Source**: `src/lib/project-draft.ts`
> **Status**: [REVIEW]

## Overview

Pure helper module for the Projects workbench draft state. Provides single-agent and multi-agent batch mutation functions.

## Public Surface

| Export | Purpose |
|---|---|
| `ProjectAgentDraft` | Per-agent draft shape: `selectedSkillIds`, `selectedSceneIds`, `excludedSkillIds`. |
| `ProjectDraft` | Create/edit draft shape including `agents` map, `sourceProjectPath`, `displayNameManuallyEdited`, and `unsupportedAgentKeys`. |
| `suggestProjectDisplayName` | Derives a default display name from the path basename. |
| `applyProjectPathToDraft` | Updates `projectPath` and keeps `displayName` synced to the suggestion until the user edits it manually. |
| `applyProjectDisplayNameToDraft` | Applies an explicit display name edit and locks future path-sync auto-fill. |
| `isProjectDraftDirty` | Compares the active draft against create or edit baselines. |
| `removeProjectManagedTargetSkill` | Cancels a managed project target skill by removing the direct pick and adding a project-local exclusion. |
| `projectDraftToAgentAssignments` | Serializes per-agent project layers for Tauri save commands. |
| `ensureProjectAgentDraft` | Creates an empty agent draft entry if one does not exist. |
| `removeProjectAgentDraft` | Removes an agent draft entry from the draft. |
| `toggleProjectAgentSkill` | Toggles a single skill for one agent. |
| `toggleProjectAgentScene` | Toggles a single scene for one agent. |
| `toggleProjectAgentSkillForAgents` | Batch-toggles a skill across multiple agents. If all have it, deselects from all; otherwise selects for all. |
| `toggleProjectAgentSceneForAgents` | Batch-toggles a scene across multiple agents. Same all/some/none logic. |
| `toggleProjectAgentExclusion` | Toggles a skill exclusion for one agent. |
| `projectDraftFromAssignment` | Builds an edit-mode draft from a saved `ProjectAssignment`. |
| `projectDraftAgentKeys` | Returns sorted agent keys from the draft. |

## Core Logic

The module keeps create/edit draft behavior pure: path-derived names, dirty-state comparison, per-agent layer mutation, and serialization back to the Tauri DTO shape. Editor list filtering lives in `src/lib/project-filters.ts` so draft mutation does not also own view-facing search and sort rules.

### Batch Functions

`toggleProjectAgentSkillForAgents` and `toggleProjectAgentSceneForAgents` iterate over the provided `agentKeys`. They determine whether the item should be selected by checking if every agent already has it — if so, the function deselects from all; otherwise it ensures every agent has it selected. These are used by the multi-agent editor to toggle a single skill/scene row for all selected agents in one action.

`removeProjectManagedTargetSkill` is used by the target inventory management UI for entries that the app recognizes as managed. It does not delete filesystem content immediately; it edits the draft so the next save/apply stops selecting that project-local skill and removes it from the target during reconciliation. Re-selecting the skill through the normal toggle clears the exclusion again.

Preview derivation lives in `src/lib/project-summary.ts` so this module stays centered on draft mutation.

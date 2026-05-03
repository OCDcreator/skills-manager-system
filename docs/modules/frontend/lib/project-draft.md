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
| `filterProjectSkills` / `filterProjectAgents` | Client-side search helpers. |
| `buildProjectSummary` | Converts draft + inspection state into summary-friendly counts, selected item mirrors, warnings, and target data. |

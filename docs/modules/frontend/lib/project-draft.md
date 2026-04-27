# Project Draft Helper

> **Source**: `src/lib/project-draft.ts`
> **Status**: [REVIEW]

## Overview

Pure helper module for the Projects workbench draft state.

## Public Surface

| Export | Purpose |
|---|---|
| `ProjectDraft` | Create/edit draft shape including `sourceProjectPath` and `unsupportedAgentKeys`. |
| `suggestProjectDisplayName` | Derives a default display name from the path basename. |
| `isProjectDraftDirty` | Compares the active draft against create or edit baselines. |
| `filterProjectSkills` / `filterProjectAgents` | Client-side search helpers. |
| `buildProjectSummary` | Converts draft + inspection state into summary-friendly data. |

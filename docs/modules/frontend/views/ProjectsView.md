# Projects View

> **Source**: `src/views/ProjectsView.tsx`
> **Status**: [REVIEW]

## Overview

Path-first project assignment workbench for create/edit flows plus a secondary saved-project management list.

## Core Logic

Loads `ProjectConfigSnapshot` via a deferred refresh, keeps one `ProjectDraft` state object, inspects the current path through `useProjectDraftInspection`, and derives duplicate-path / expected-target / unsupported-agent state before save. The create flow opens a Tauri folder picker for `projectPath`, auto-fills `displayName` from the suggested basename until the user edits it manually, and still keeps the path locked in edit mode. Saved projects can be reopened in edit mode and the saved-project section now applies assignments with one section-level `applyProjectAssignments()` action.

## Data Flow

`useAppContext()` provides `scanResult`, `sortedAgentInventory`, and `disabledSkillIds`; `src/lib/projects.ts` handles CRUD/apply/inspection calls; `src/lib/project-draft.ts` provides pure draft helpers; `ProjectIdentityPanel`, `ProjectAssignmentEditor`, `ProjectAssignmentSummary`, and `ProjectCard` split the page into focused UI units.

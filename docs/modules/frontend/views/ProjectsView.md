# Projects View

> **Source**: `src/views/ProjectsView.tsx`
> **Status**: [REVIEW]

## Overview

Path-first project assignment workbench for create/edit flows plus a secondary saved-project management list.

## Core Logic

Loads `ProjectConfigSnapshot` via a deferred refresh, keeps one `ProjectDraft` state object, inspects the current path through `useProjectDraftInspection`, and derives duplicate-path / expected-target / unsupported-agent state before save. The create flow opens a Tauri folder picker for `projectPath`, auto-fills `displayName` from the suggested basename until the user edits it manually, and still keeps the path locked in edit mode. The view saves through per-agent project wrappers so project scenes and project-local exclusions are preserved.

The workbench applies explicit filtering in the editor: skills can be narrowed by free text, top-level repository path buckets, and selected-state (`all / selected / unselected`), while agents can be narrowed by search plus enabled state. When editing an existing project assignment, `sortProjectAgentsForEditor` pushes already-configured agents ahead of the rest so the user's current configuration stays visible without manual scrolling. `buildProjectSummary` derives per-agent inherited/global/project/exclusion preview slices for the sticky inspector. Saved projects can be reopened in edit mode and the saved-project section applies assignments with one section-level `applyProjectAssignments()` action.

## Data Flow

`useAppContext()` provides `scanResult`, `sortedAgentInventory`, and `disabledSkillIds`; `src/lib/projects.ts` handles CRUD/apply/inspection calls; `src/lib/project-draft.ts` provides pure draft helpers; `src/lib/project-summary.ts` derives read-only preview data. `ProjectLayerWorkbench` composes identity, editor, and summary panels, while `SavedProjectsSection` owns the saved-list rendering.

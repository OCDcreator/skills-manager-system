# Projects View

> **Source**: `src/views/ProjectsView.tsx`
> **Status**: [REVIEW]

## Overview

Path-first project assignment workbench for create/edit flows plus a secondary saved-project management list.

## Core Logic

Loads `ProjectConfigSnapshot` via a deferred refresh, keeps one `ProjectDraft` state object, inspects the current path through `useProjectDraftInspection`, and derives duplicate-path / expected-target / unsupported-agent state before save. The create flow opens a Tauri folder picker for `projectPath`, auto-fills `displayName` from the suggested basename until the user edits it manually, and still keeps the path locked in edit mode. The view saves through per-agent project wrappers so project skills and project scenes are preserved.

The target-skill management hook keeps ownership semantics explicit. Managed entries are treated as draft configuration: delete/cancel removes the direct project selection and adds a project-local exclusion so the next save/apply reconciles the target. Unmanaged entries are treated as target-directory content: the hook asks for confirmation, calls `deleteProjectTargetSkill`, and bumps the inspection refresh key so the existing-target card is rescanned.

The workbench applies explicit filtering in the editor: skills can be narrowed by free text, repository path buckets, a scene-style external group popover when `external` is active, and selected-state (`all / selected / unselected`), while agents can be narrowed by search plus enabled state. When editing an existing project assignment, `sortProjectAgentsForEditor` pushes already-configured agents ahead of the rest so the user's current configuration stays visible without manual scrolling. `buildProjectSummary` derives per-agent inherited/global/project preview slices for the summary inspector. Saved projects can be reopened in edit mode and the saved-project section applies assignments with one section-level `applyProjectAssignments()` action.

Responsive behavior is split across two layers. At the app-shell level, Projects opts into the wider route container and still lives inside the global compact shell below `1280px`. Inside the page, the real workbench breakpoint is local to `ProjectLayerWorkbench`: the editor and summary stay stacked through both `<900px` and `900px-1279px`, and they remain stacked through `1280px-1379px` as well. The right summary only becomes a sticky side inspector at `>=1380px`, which keeps the compact baseline stable across the shell's `<1280` band.

## Data Flow

`useAppContext()` provides `scanResult`, `sortedAgentInventory`, and `disabledSkillIds`; `src/lib/projects.ts` handles CRUD/apply calls; `src/lib/project-draft.ts` provides pure draft helpers; `src/lib/project-filters.ts` owns editor filtering; `src/lib/project-summary.ts` derives read-only preview data; `useProjectDraftInspection()` owns target inspection. `ProjectLayerWorkbench` composes identity, editor, and summary panels, while `SavedProjectsSection` owns the saved-list rendering.

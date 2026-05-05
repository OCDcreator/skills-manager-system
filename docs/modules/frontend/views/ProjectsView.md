# Projects View

> **Source**: `src/views/ProjectsView.tsx`
> **Status**: [REVIEW]

## Overview

Path-first project assignment workbench for create/edit flows plus a secondary saved-project management list.

## Core Logic

Loads `ProjectConfigSnapshot` via a deferred refresh, keeps one `ProjectDraft` state object, inspects the current path through `useProjectDraftInspection`, and derives duplicate-path / expected-target / unsupported-agent state before save. The create flow opens a Tauri folder picker for `projectPath`, auto-fills `displayName` from the suggested basename until the user edits it manually, and still keeps the path locked in edit mode. The primary save CTA lives beside the display-name field in `ProjectIdentityPanel`; edit mode also exposes a cancel action that resets the draft back to a new assignment without mutating saved config.

The workbench now applies more explicit, design-system-like filtering in the left editor: skills can be narrowed by free text, top-level repository path buckets, and selected-state (`all / selected / unselected`), while agents can be narrowed by search plus enabled state. When editing an existing project assignment, `sortProjectAgentsForEditor` pushes already-configured agents ahead of the rest so the user's current configuration stays visible without manual scrolling. `buildProjectSummary` still derives selected skill/agent summary slices so the right panel can act as a sticky inspector instead of a sparse target-only card, and the selected-skills mirror is rendered in a fixed-height nested scroll region to prevent overflow when many skills are selected. Saved projects can be reopened in edit mode and the saved-project section now applies assignments with one section-level `applyProjectAssignments()` action.

## Data Flow

`useAppContext()` provides `scanResult`, `sortedAgentInventory`, and `disabledSkillIds`; `src/lib/projects.ts` handles CRUD/apply/inspection calls; `src/lib/project-draft.ts` provides pure draft helpers; `ProjectIdentityPanel`, `ProjectAssignmentEditor`, `ProjectAssignmentSummary`, and `ProjectCard` split the page into focused UI units. The left assignment editor owns selection controls, while the right summary receives selected item slices plus inspection results for read-only review.

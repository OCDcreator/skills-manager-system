# Project Assignment Editor

> **Source**: `src/components/projects/ProjectAssignmentEditor.tsx`
> **Status**: [REVIEW]

## Overview

Project-layer editor for one selected agent at a time. Renders the agent chooser plus project direct skills, project scenes, and project-local exclusions.

## Responsibilities

- receives already-filtered `skills`, `agents`, `scenes`, and exclusion candidates
- keeps agent selection separate from the active agent layer being edited
- renders skill toggles with two-line-clamped descriptions plus visible relative-path labels
- renders compact agent toggles with `projectSkillsDirRule` hints, not global sync paths
- forwards local query and toggle events back to `ProjectLayerWorkbench`
- renders path-bucket pills and selected-state pills on one compact skill-filter row so the workbench can be narrowed without typing
- renders agent enabled-state pills separately for agent filtering
- renders project-scene toggles and exclusion toggles for the active agent draft
- caps the two chooser panes with viewport-aware maximum heights so their content scrolls instead of stretching the page
- uses the shared `skill-markdown-scroll` surface for both chooser scroll areas
- remembers each chooser pane's scroll position independently across remounts
- weights the desktop chooser grid toward skills while keeping the Agent column narrower and lighter

## Interaction Notes

The skill column uses repository-path buckets like `custom` and `external`, matching the app's broader skill vocabulary, and also supports selected-state filtering without typing. The agent column exposes `all / enabled / disabled` pills and can receive a selected-first sorted list from the view when editing an existing assignment, which keeps already-configured agents near the top of the chooser. Scene and exclusion panes are disabled until an agent layer is selected.

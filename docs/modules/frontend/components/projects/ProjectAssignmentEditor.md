# Project Assignment Editor

> **Source**: `src/components/projects/ProjectAssignmentEditor.tsx`
> **Status**: [REVIEW]

## Overview

Project-layer editor for one selected agent at a time. Renders a compact agent strip, a primary project-skill work area, and a project-scene panel.

## Responsibilities

- receives already-filtered `skills`, `agents`, and `scenes`
- keeps agent selection separate from the active agent layer being edited
- renders project-skill toggles as a dense scene-style grid with visible relative-path labels
- renders compact horizontal agent toggles with `projectSkillsDirRule` hints, not global sync paths
- forwards local query and toggle events back to `ProjectLayerWorkbench`
- delegates the skill search, path filters, selected-state filters, and external group popover to `ProjectSkillFilterToolbar`
- renders agent enabled-state pills separately for agent filtering
- gives agent and skill searches explicit placeholder text and a search icon so empty fields do not read as unlabeled boxes
- renders project-scene toggles for the active agent draft
- caps the two chooser panes with viewport-aware maximum heights so their content scrolls instead of stretching the page
- lets the primary skill pane overflow vertically so the external group popover is not clipped by the panel shell
- uses the shared `skill-markdown-scroll` surface for both chooser scroll areas
- remembers each chooser pane's scroll position independently across remounts
- weights the desktop workbench toward skills while keeping scenes in a narrow secondary column

## Interaction Notes

The skill column uses repository-path buckets like `custom` and `external`, matching the app's broader skill vocabulary. Its skill list follows the Scene skill chooser's high-density grid pattern, but keeps each skill's relative path visible because project assignment often compares many external collections. When `external` is active, the delegated toolbar can open a scene-style group popover for large external source sets. It also supports selected-state filtering without typing. The agent strip exposes `all / enabled / disabled` pills and can receive a selected-first sorted list from the view when editing an existing assignment, which keeps already-configured agents near the start of the chooser. The project layer intentionally does not expose exclusions because project-local sync cannot prevent an agent from reading skills that still exist in its global skill directory.

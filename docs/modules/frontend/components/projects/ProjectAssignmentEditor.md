# Project Assignment Editor

> **Source**: `src/components/projects/ProjectAssignmentEditor.tsx`
> **Status**: [REVIEW]

## Overview

Main left-column editor for project assignment drafts. Renders searchable multi-select lists for skills and agents.

## Responsibilities

- receives already-filtered `skills` and `agents`
- renders skill toggles with two-line-clamped descriptions plus visible relative-path labels
- renders compact agent toggles with `projectSkillsDirRule` hints, not global sync paths
- forwards local query and toggle events back to `ProjectsView`
- renders path-bucket pills for skills and enabled-state pills for agents so the workbench can be narrowed without typing
- renders an additional skill-selection filter row (`all / selected / unselected`) to quickly isolate already-picked or remaining skills
- caps the two chooser panes with viewport-aware maximum heights so their content scrolls instead of stretching the page
- uses the shared `skill-markdown-scroll` surface for both chooser scroll areas
- remembers each chooser pane's scroll position independently across remounts
- weights the desktop chooser grid toward skills while keeping the Agent column narrower and lighter

## Interaction Notes

The skill column uses repository-path buckets like `custom` and `external`, matching the app's broader skill vocabulary, and now also supports selected-state filtering without typing. The agent column exposes `all / enabled / disabled` pills and can receive a selected-first sorted list from the view when editing an existing assignment, which keeps already-configured agents near the top of the chooser.

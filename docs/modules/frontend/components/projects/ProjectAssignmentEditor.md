# Project Assignment Editor

> **Source**: `src/components/projects/ProjectAssignmentEditor.tsx`
> **Status**: [REVIEW]

## Overview

Main left-column editor for project assignment drafts. Renders searchable multi-select lists for skills and agents.

## Responsibilities

- receives already-filtered `skills` and `agents`
- renders skill toggles with two-line-clamped descriptions to keep the left work area scan-friendly
- renders compact agent toggles with project-local rule hints
- forwards local query and toggle events back to `ProjectsView`
- caps the two chooser panes with viewport-aware maximum heights so their content scrolls instead of stretching the page
- uses the shared `skill-markdown-scroll` surface for both chooser scroll areas
- remembers each chooser pane's scroll position independently across remounts
- weights the desktop chooser grid toward skills while keeping the Agent column narrower and lighter

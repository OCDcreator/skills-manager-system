# Project Assignment Editor

> **Source**: `src/components/projects/ProjectAssignmentEditor.tsx`
> **Status**: [REVIEW]

## Overview

Main left-column editor for project assignment drafts. Renders searchable multi-select lists for skills and agents.

## Responsibilities

- receives already-filtered `skills` and `agents`
- renders skill toggles with descriptions
- renders agent toggles with project-local rule hints
- forwards local query and toggle events back to `ProjectsView`

# Saved Projects Section

> **Source**: `src/components/projects/SavedProjectsSection.tsx`
> **Status**: [REVIEW]

## Overview

Renders the saved-project management section below the project workbench, including the section-level apply-all action.

## Responsibilities

- shows the empty saved-project state
- renders the saved-project heading and apply-all button
- maps each saved `ProjectAssignment` into a `ProjectCard`
- passes edit, delete, and session apply feedback callbacks through without owning persistence
- keeps saved-project actions demoted below the primary workbench instead of competing with the editor/summary surface

## Interactions

Used by `ProjectsView.tsx` after project config loading finishes. Project mutation and apply calls remain in the view so this component stays a focused list renderer.

This section has no independent `900px-1279px` logic. It inherits the route's compact baseline and simply stays a single-column management list beneath the workbench at every width.

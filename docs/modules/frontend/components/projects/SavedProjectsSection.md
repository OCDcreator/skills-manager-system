# Saved Projects Section

> **Source**: `src/components/projects/SavedProjectsSection.tsx`
> **Status**: [REVIEW]

## Overview

Renders the saved-project list below the project-layer workbench, including the section-level apply-all action.

## Responsibilities

- shows the empty saved-project state
- renders the saved-project heading and apply-all button
- maps each saved `ProjectAssignment` into a `ProjectCard`
- passes edit, delete, and session apply feedback callbacks through without owning persistence

## Interactions

Used by `ProjectsView.tsx` after project config loading finishes. Project mutation and apply calls remain in the view so this component stays a focused list renderer.

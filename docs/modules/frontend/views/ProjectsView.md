# Projects View

> **Source**: `src/views/ProjectsView.tsx`
> **Status**: [REVIEW]

## Overview

Page for managing project-level skill assignments, including project-local deployment into supported agent config directories.

## Core Logic

Loads `ProjectConfigSnapshot` via a deferred initial refresh, lets the user add project assignments, uses the globally sorted agent inventory for consistent checklist ordering, allows all supported agents to be selected regardless of global detection state, and reads `ApplyProjectAssignmentsResponse` so per-project apply feedback matches the backend payload shape.

## Data Flow

`useAppContext()` provides scanned skills plus the full supported-agent inventory in sorted order; `src/lib/projects.ts` handles CRUD/apply calls; `ProjectCard` renders each saved assignment.

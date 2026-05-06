# Project Layer Workbench

> **Source**: `src/components/projects/ProjectLayerWorkbench.tsx`
> **Status**: [REVIEW]

## Overview

Composes the editable project assignment surface: project identity, per-agent project-layer editor, and sticky summary inspector.

## Responsibilities

- keeps the Projects page view focused on data loading, save/apply handlers, and draft derivation
- passes identity save controls to `ProjectIdentityPanel`
- wires `ProjectAssignmentEditor` to the active agent layer, filtered skills, scenes, and exclusion candidates
- wires `ProjectAssignmentSummary` to the derived per-agent preview counts and unsupported saved agents
- owns the responsive two-column workbench grid and sticky right inspector placement

## Interactions

`ProjectsView.tsx` computes draft state, filters, inspection, and summary data before passing them in. This component does not call Tauri APIs and does not mutate draft state directly; all mutations flow through callback props.

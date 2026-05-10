# Project Layer Workbench

> **Source**: `src/components/projects/ProjectLayerWorkbench.tsx`
> **Status**: [REVIEW]

## Overview

Composes the editable Projects workbench: project identity, per-agent project-layer editor, and the summary/inspection surface.

## Responsibilities

- keeps the Projects page view focused on data loading, save/apply handlers, and draft derivation
- passes identity save controls to `ProjectIdentityPanel`
- wires `ProjectAssignmentEditor` to the active agent layer, filtered skills, external group summaries, and scenes
- wires `ProjectAssignmentSummary` to the derived per-agent preview counts and unsupported saved agents
- forwards target skill management callbacks and pending action state to the summary inspector
- owns the responsive workbench breakpoint where the summary stays stacked until `1380px`
- moves the save action into `ProjectIdentityPanel` instead of the summary surface

## Interactions

`ProjectsView.tsx` computes draft state, filters, inspection, and summary data before passing them in. This component does not call Tauri APIs and does not mutate draft state directly; all mutations flow through callback props.

Its compact decision is narrower than the global shell rule: even after the shell leaves the `<1280` compact band, the workbench itself still stays single-column until `min-[1380px]`. That means `<900px`, `900px-1279px`, and `1280px-1379px` all share the same stacked identity/editor/summary flow, while `>=1380px` switches to the wide editor-plus-sticky-inspector grid.

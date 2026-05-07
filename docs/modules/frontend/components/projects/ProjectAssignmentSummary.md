# Project Assignment Summary

> **Source**: `src/components/projects/ProjectAssignmentSummary.tsx`
> **Status**: [REVIEW]

## Overview

Right-side inspector panel for the Projects workbench. Mirrors each selected agent's layered preview, then shows expected write targets, already-present project target skills, unsupported saved agents, and duplicate-path warnings.

## Responsibilities

- displays the create/edit title and selected agent/direct-skill/scene counts
- mirrors per-agent inherited global skills, project direct skills, and project scenes as compact preview groups
- lists target directories returned by project-path inspection
- delegates existing project-local target skill entry rendering to the default-collapsed `ProjectTargetSkillList`
- forwards managed cancel and unmanaged delete requests without owning the mutation rules
- surfaces duplicate-path and globally-disabled-skill warnings
- keeps the right inspector capped and scrollable on narrow layouts, then absolutely fills the desktop grid cell whose height is determined by the left assignment stack; the existing target list expands into a taller internal scroll area, remembers the inspector scroll position by summary title, and relies on the identity panel for the primary save action

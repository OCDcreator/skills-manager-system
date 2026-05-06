# Project Assignment Summary

> **Source**: `src/components/projects/ProjectAssignmentSummary.tsx`
> **Status**: [REVIEW]

## Overview

Right-side inspector panel for the Projects workbench. Mirrors each selected agent's layered preview, then shows expected write targets, unsupported saved agents, and duplicate-path warnings.

## Responsibilities

- displays the create/edit title and selected agent/direct-skill/scene/exclusion counts
- mirrors per-agent inherited global skills, project direct skills, project scenes, and project exclusions as compact preview groups
- lists target directories returned by project-path inspection
- surfaces duplicate-path and globally-disabled-skill warnings
- keeps the right inspector capped and scrollable on narrow layouts, then absolutely fills the desktop grid cell whose height is determined by the left assignment stack; the expected target list fills remaining inspector space with its own scroll area, remembers its inner scroll position by summary title, and relies on the identity panel for the primary save action

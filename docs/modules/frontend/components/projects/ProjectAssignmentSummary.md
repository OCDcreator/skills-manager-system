# Project Assignment Summary

> **Source**: `src/components/projects/ProjectAssignmentSummary.tsx`
> **Status**: [REVIEW]

## Overview

Right-side inspector panel for the Projects workbench. Mirrors selected skills and agents, then shows expected write targets, unsupported saved agents, and duplicate-path warnings.

## Responsibilities

- displays the create/edit title and selected counts
- mirrors selected skills as compact chips inside a fixed-height nested scroll window and selected agents as compact status rows with project-local rule hints
- lists target directories returned by project-path inspection
- surfaces duplicate-path and globally-disabled-skill warnings
- keeps the right inspector capped and scrollable on narrow layouts, then absolutely fills the desktop grid cell whose height is determined by the left assignment stack; the expected target list fills remaining inspector space with its own scroll area, remembers its inner scroll position by summary title, and relies on the identity panel for the primary save action

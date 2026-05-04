# Project Assignment Summary

> **Source**: `src/components/projects/ProjectAssignmentSummary.tsx`
> **Status**: [REVIEW]

## Overview

Right-side inspector panel for the Projects workbench. Mirrors selected skills and agents, then shows expected write targets, unsupported saved agents, and duplicate-path warnings.

## Responsibilities

- displays the create/edit title and selected counts
- mirrors selected skills as compact chips and selected agents as compact status rows
- lists target directories returned by project-path inspection
- surfaces duplicate-path and globally-disabled-skill warnings
- stays scrollable within a viewport-aware maximum height, remembers its inner scroll position by summary title, and relies on the identity panel for the primary save action

# Project Assignment Summary

> **Source**: `src/components/projects/ProjectAssignmentSummary.tsx`
> **Status**: [REVIEW]

## Overview

Right-side summary panel for the Projects workbench. Shows draft counts, expected write targets, unsupported saved agents, duplicate-path warnings, and the primary save CTA.

## Responsibilities

- displays the create/edit title and selected counts
- lists target directories returned by project-path inspection
- surfaces duplicate-path and globally-disabled-skill warnings
- owns the save button loading/disabled state

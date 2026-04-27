# Project Draft Inspection Hook

> **Source**: `src/components/projects/useProjectDraftInspection.ts`
> **Status**: [REVIEW]

## Overview

Debounced frontend hook for read-only project-path inspection.

## Responsibilities

- waits briefly before calling `inspectProjectAssignmentPath`
- treats newer requests as authoritative and discards stale results
- exposes `inspection`, `isInspecting`, and `inspectionError` to the workbench

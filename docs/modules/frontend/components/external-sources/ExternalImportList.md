# ExternalImportList

> **Source**: `src/components/external-sources/ExternalImportList.tsx`
> **Status**: [REVIEW]

## Overview

Renders the imported-mirror list for one external source card.

## Public Surface

| Export | Purpose |
|---|---|
| `ExternalImportList` | Read/write list for update and repair actions on imported mirrors. |

## Core Logic

The component stays presentation-focused: it shows the empty state, renders agent/skill badges plus pinned and last-checked commits, surfaces stored warning messages, and disables update or repair buttons from the shared `updatingImportId` plus a local `busyImportAction` intent so the visible busy label matches the action the user actually triggered. Busy-state truth comes from `resolveImportBusyState()` in `src/lib/external-sources.ts`, which prevents a repair operation on an updateable import from being mislabeled as an update. It does not decide whether an import is stale; it only reflects `updateAvailable` from the backend snapshot.

## Interactions

Used by `ExternalSourceCard`. Commit formatting relies on `shortCommit()`, and warning/update semantics must stay aligned with `ImportedExternalSkillRecord` from `src/lib/tauri.ts`. Repairing an import with `updateAvailable === true` must still show the repair busy label instead of borrowing the update label.

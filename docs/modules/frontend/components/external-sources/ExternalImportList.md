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

The component stays presentation-focused: it shows the empty state, renders agent/skill badges plus pinned and last-checked commits, surfaces stored warning messages, and disables update or repair buttons from the single `updatingImportId` flag. It does not decide whether an import is stale; it only reflects `updateAvailable` from the backend snapshot.

## Interactions

Used by `ExternalSourceCard`. Commit formatting relies on `shortCommit()`, and warning/update semantics must stay aligned with `ImportedExternalSkillRecord` from `src/lib/tauri.ts`.

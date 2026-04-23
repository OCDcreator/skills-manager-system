# Unsaved Changes Dialog

> **Source**: `src/components/UnsavedChangesDialog.tsx`
> **Status**: [REVIEW]

## Overview

Renders the modal prompt shown when the user tries to leave the Agents page with unsaved draft changes.

## Public Surface

| Export | Purpose |
|---|---|
| `UnsavedChangesDialog` | Save/discard/stay modal with inline save-error feedback. |

## Core Logic

Shows the pending target view label, runs async save-and-sync when requested, keeps the modal open on save failure, and routes discard/cancel directly back to the navigation guard actions.

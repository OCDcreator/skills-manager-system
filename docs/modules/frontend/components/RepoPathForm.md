# RepoPathForm

> **Source**: `src/components/RepoPathForm.tsx`
> **Status**: [REVIEW]

## Overview

Renders the settings form used to configure the local managed skill-repository path.

## Import Relationships

Used by `src/views/SettingsView.tsx`; depends on the Tauri dialog plugin, `AppContext`, and settings i18n keys.

## Public Surface

| Export | Purpose |
|---|---|
| `RepoPathForm` | Controlled form for browsing, editing, and saving the repository path. |

## Core Logic

The component keeps local draft state only after the user edits or browses. Until then it renders the persisted `repoPath` directly, which avoids effect-driven state syncing while still resetting the form after a successful save. Submit clears the transient status message first, then reports either `settings.saved` or `settings.error` after `saveRepoPath()` resolves.

The form stays single-column in compact mode. The path row uses `flex-col` by default and only upgrades to a two-item `sm:flex-row` layout for the input plus browse button. The input keeps `min-w-0` and `overflow-x-auto` so long Windows or macOS paths remain readable without blowing out the panel in the `<1280` compact shell.

## Data Flow

The saved path originates from context, is edited in local state, and returns to context through `saveRepoPath`.

## Interactions

Must stay in sync with the Tauri dialog plugin, `AppContext.saveRepoPath`, and settings i18n keys.

## Change Notes

Keep directory picking, validation, and persistence semantics in the context/API/backend layers; this component should stay UI-focused and overflow-safe.

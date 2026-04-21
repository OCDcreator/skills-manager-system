# RepoPathForm

> **Source**: `src/components/RepoPathForm.tsx`
> **Status**: [REVIEW]

## Overview

Renders the settings form used to configure the local `my-skills` repository path.

## Import Relationships

```text
Upstream: src/views/SettingsView.tsx
Downstream: @tauri-apps/plugin-dialog, src/context/AppContext.tsx, src/i18n/index.ts
```

## Public Surface

| Export | Purpose |
|---|---|
| `RepoPathForm` | Controlled form for browsing, editing, and saving the repository path. |

## Core Logic

The component keeps local draft state only after the user edits or browses. Until then it renders the persisted `repoPath` directly, which avoids effect-driven state syncing while still resetting the form after a successful save.

## Data Flow

The saved path originates from context, is edited in local state, and returns to context through `saveRepoPath`.

## Interactions

Must stay in sync with the Tauri dialog plugin, `AppContext.saveRepoPath`, and settings i18n keys.

## Configuration

The directory picker uses the current draft path as `defaultPath` when present.

## Change Notes

Keep path validation or persistence rules in the context/API/backend layers; this component should stay UI-focused.

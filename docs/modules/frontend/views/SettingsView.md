# SettingsView

> **Source**: `src/views/SettingsView.tsx`
> **Status**: [REVIEW]

## Overview

Composes the settings page for configuring the skill repository path.

## Import Relationships

```text
Upstream: src/App.tsx
Downstream: src/components/RepoPathForm.tsx, src/i18n/index.ts
```

## Public Surface

| Export | Purpose |
|---|---|
| `SettingsView` | Page-level view for repository path settings. |

## Core Logic

The view renders translated title and description text, then delegates path editing to `RepoPathForm`.

## Data Flow

No data is fetched here. `RepoPathForm` owns interaction with context.

## Interactions

Depends on `settings.title` and `settings.description` i18n keys.

## Configuration

None.

## Change Notes

Keep this view as page composition; add settings-specific form behavior in dedicated components or services.

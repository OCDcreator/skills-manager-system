# SettingsView

> **Source**: `src/views/SettingsView.tsx`
> **Status**: [REVIEW]

## Overview

Composes the settings page for configuring the managed skill-repository path.

## Import Relationships

`App.tsx` routes here; the view renders title/description copy above `RepoPathForm`.

## Public Surface

| Export | Purpose |
|---|---|
| `SettingsView` | Page-level view for repository path settings. |

## Core Logic

The view is intentionally thin. It renders a single bordered settings panel with translated title and description text, then delegates all path editing and save behavior to `RepoPathForm`.

This page is part of the compact baseline with no width-specific branch: it stays single-column at `<900px`, `900px-1279px`, and `>=1280px`. The goal is stability and overflow safety rather than a multi-panel settings workbench.

## Data Flow

No data is fetched here. `RepoPathForm` owns interaction with context.

## Interactions

Depends on `settings.title` and `settings.description` i18n keys.

## Change Notes

Keep this view as page composition; add settings-specific form behavior in dedicated components or services rather than widening this page into a multi-surface settings hub.

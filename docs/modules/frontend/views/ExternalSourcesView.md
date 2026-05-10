# ExternalSourcesView

> **Source**: `src/views/ExternalSourcesView.tsx`
> **Status**: [REVIEW]

## Overview

Provides the dedicated management surface for external GitHub source records, source warnings, detected variants, and import-level status.

## Public Surface

| Export | Purpose |
|---|---|
| `ExternalSourcesView` | Top-level page for external source management. |

## Core Logic

The view reads all source CRUD and import actions from `AppContext`, shows the page intro plus refresh button, warns when no repo path is configured, hosts `AddExternalSourceForm`, and renders either a loading shell or `ExternalSourceList`. It owns source-list orchestration and is the only frontend surface that exposes add, fetch, and remove-source actions directly; agent-specific import/update/repair behavior is delegated into the list/card layer.

Its compact behavior matches the new baseline: the header section stays vertical until `xl`, and the page keeps a simple stacked flow for both `<900px` and `900px-1279px` instead of introducing a special mid-width drawer or split. Only at `xl` does the intro row widen into a left-description plus right-refresh layout.

## Interactions

Must stay aligned with `src/components/external-sources/*`, `src/lib/tauri.ts`, and the `sources.*` i18n keys. Source creation remains here; agent-scoped variant actions reused in the cards should keep the same backend semantics as the agent-facing surfaces.

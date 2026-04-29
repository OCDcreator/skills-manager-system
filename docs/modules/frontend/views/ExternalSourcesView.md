# ExternalSourcesView

> **Source**: `src/views/ExternalSourcesView.tsx`
> **Status**: [REVIEW]

## Overview

Provides the dedicated desktop management surface for external GitHub source records, source warnings, detected variants, and import-level status.

## Public Surface

| Export | Purpose |
|---|---|
| `ExternalSourcesView` | Top-level page for external source management. |

## Core Logic

The view reads all source CRUD and import actions from `AppContext`, shows the page intro plus refresh button, warns when no repo path is configured, hosts `AddExternalSourceForm`, and renders either a loading shell or `ExternalSourceList`. It owns source-list orchestration and is the only frontend surface that exposes add, fetch, and remove-source actions directly.

## Interactions

Must stay aligned with `src/components/external-sources/*`, `src/lib/tauri.ts`, and the `sources.*` i18n keys. Agent-scoped variant actions reused here should keep the same backend semantics as `AgentExternalVariantPanel`.

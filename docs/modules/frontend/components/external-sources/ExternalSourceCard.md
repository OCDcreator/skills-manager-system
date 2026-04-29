# ExternalSourceCard

> **Source**: `src/components/external-sources/ExternalSourceCard.tsx`
> **Status**: [REVIEW]

## Overview

Displays one external source record together with its detected variants and imported mirrors.

## Public Surface

| Export | Purpose |
|---|---|
| `ExternalSourceCard` | Main card for source-level fetch, remove, import, update, and repair actions. |

## Core Logic

The card derives compact repo labels, status classes, commit display text, and the set of already imported variant keys. It keeps source-level actions separate from import-level actions: fetch/remove work on `record.id`, variant import works on detected upstream variants, and imported mirrors delegate update/repair rendering to `ExternalImportList`. Removal confirmation is the only inline destructive UX handled here.

## Interactions

Lives under `ExternalSourceList` on the dedicated sources page. It must stay aligned with `ExternalSourceSnapshotItem`, `warningSummary()`, and the backend rule that source deletion may require cascading import deletion.

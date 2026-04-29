# ExternalSourceCard

> **Source**: `src/components/external-sources/ExternalSourceCard.tsx`
> **Status**: [REVIEW]

## Overview

Displays one external source record together with its detected variants and imported mirrors.

## Public Surface

| Export | Purpose |
|---|---|
| `ExternalSourceCard` | Main card for source-level fetch, remove, import, update, repair, and repo-open actions. |

## Core Logic

The card now defaults to a collapsed summary-first layout so long source lists stay scan-friendly. The always-visible header derives its purpose line from the first detected variant that exposes parsed `SKILL.md` description metadata; only when no metadata is available does it fall back to the older generated-bundle wording. It also shows generated-bundle and imported-count tags, agent badges, and a clickable repository URL that opens through Tauri's opener plugin with a browser fallback.

Expanded content still keeps source-level actions separate from import-level actions: fetch/remove work on `record.id`, variant import works on detected upstream variants, and imported mirrors delegate update/repair rendering to `ExternalImportList`. Removal confirmation remains the only inline destructive UX handled here.

## Interactions

Lives under `ExternalSourceList` on the dedicated sources page. It must stay aligned with `ExternalSourceSnapshotItem`, `sourceAgentLabels()`, `warningSummary()`, and the desktop opener capability because the repo link is intended to jump out to the upstream website directly from the card.

# ExternalSourceCard

> **Source**: `src/components/external-sources/ExternalSourceCard.tsx`
> **Status**: [REVIEW]

## Overview

Displays one external source record together with its detected candidates and imported mirrors.

## Public Surface

| Export | Purpose |
|---|---|
| `ExternalSourceCard` | Main card for source-level fetch, remove, import, update, repair, and repo-open actions. |

## Core Logic

The card now defaults to a collapsed summary-first layout so long source lists stay scan-friendly. The always-visible header derives its purpose line from the first detected variant that exposes parsed `SKILL.md` description metadata; only when no metadata is available does it fall back to the older generated-bundle wording. The header gives the summary column all remaining space while the source-level controls keep their natural width on wide screens, then lets those controls reflow into an auto-fitting grid below the summary on narrower screens so long descriptions and repo URLs cannot squeeze the buttons. It also shows kind-aware source tags, imported-count tags, agent badges, and a clickable repository URL that opens through Tauri's opener plugin with a browser fallback. The metadata row distinguishes the configured branch/subpath from the fetched default branch so nested skill repositories remain explainable after import.

Expanded content still keeps source-level actions separate from import-level actions: fetch/remove work on `record.id`, candidate import works on detected upstream variants, and imported mirrors delegate update/repair rendering to `ExternalImportList`. Each candidate row now shows detection-class badges, an optional suggested-agent hint, a lightweight direct-child preview for the variant folder itself that separates folders from files, and a local target-agent `<select>`. The import button is disabled until a target is chosen, defaults from backend suggestions when available, and resolves imported-state truth against the chosen local target plus upstream variant path instead of blindly trusting the detector's neutral fallback key.

The long candidate list is now capped with an internal scroll region that reuses the shared `skill-markdown-scroll` skin, so repositories with many generated outputs or many generic fallback candidates do not stretch the full source card indefinitely or drift away from the rest of the app's scrollbar styling. Removal confirmation remains the only inline destructive UX handled here.

## Interactions

Lives under `ExternalSourceList` on the dedicated sources page. It must stay aligned with `ExternalSourceSnapshotItem`, `sourceAgentLabels()`, `warningSummary()`, and the desktop opener capability because the repo link is intended to jump out to the upstream website directly from the card.

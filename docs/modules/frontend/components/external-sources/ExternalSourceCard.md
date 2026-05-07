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

The card defaults to a collapsed summary-first layout so long source lists stay scan-friendly. The always-visible header derives its purpose line from the first detected variant that exposes parsed `SKILL.md` description metadata; only when no metadata is available does it fall back to the generated-bundle wording. The header gives the summary column all remaining space while the source-level controls keep their natural width on wide screens, then lets those controls reflow into an auto-fitting grid below the summary on narrower screens so long descriptions and repo URLs cannot squeeze the buttons. It also shows kind-aware source tags, imported-count tags, agent badges, and a clickable repository URL that opens through Tauri's opener plugin with a browser fallback. The metadata row distinguishes the configured branch/subpath from the fetched default branch so nested skill repositories remain explainable after import.

Expanded content now keeps the source-level warning callout in the card but delegates the candidate/import workbench to `ExternalSourceAgentGroups`. That child groups detected variants and imported mirrors by target Agent, so the user compares Codex candidates with Codex mirrors, OpenCode candidates with OpenCode mirrors, and only unresolved generic candidates keep a manual target selector. Source-level fetch/remove still work on `record.id`, while candidate import works on the selected upstream variant and target Agent.

The card preserves selected manual targets while expanded/collapsed because that state remains owned here and is passed to the grouped child. Removal confirmation remains the only inline destructive UX handled here.

## Interactions

Lives under `ExternalSourceList` on the dedicated sources page. It must stay aligned with `ExternalSourceSnapshotItem`, `sourceAgentLabels()`, `warningSummary()`, `ExternalSourceAgentGroups`, and the desktop opener capability because the repo link is intended to jump out to the upstream website directly from the card.

# ExternalSourceAgentGroups

> **Source**: `src/components/external-sources/ExternalSourceAgentGroups.tsx`
> **Status**: [REVIEW]

## Overview

Renders the expanded external-source import work area grouped by target Agent.

## Public Surface

| Export | Purpose |
|---|---|
| `ExternalSourceAgentGroups` | Per-source grouped candidate and imported-mirror renderer. |

## Core Logic

The component receives one source card's detected variants, imported mirrors, source busy state, repo-path state, and import/update/repair callbacks. It derives stable groups through `groupExternalVariantsByAgent()`, then renders each Agent as the primary decision unit so candidates and already-imported mirrors for Codex, Claude Code, OpenCode, and other supported targets stay together.

Known target groups import directly into their Agent without another selector. Generic candidates that cannot be resolved to a supported Agent are placed in the `manual` group and keep the explicit target selector before import. Candidate rows still show detection class, suggested-agent hints, source-of-truth path, direct child folders/files, import state, and a same-content note when another detected variant shares the same fingerprint. The section header also shows how many duplicate-content groups exist so generated bundles with equivalent agent outputs are easier to choose from. Imported mirrors delegate row-level update/repair controls to `ExternalImportList` inside the same Agent group.

## Interactions

Used only by `ExternalSourceCard`. Keep this component aligned with `ExternalSourceSnapshotItem`, `ExternalVariantSnapshot`, `ImportedExternalSkillRecord`, `AgentBrandIcon`, `ExternalImportList`, and the grouping rules in `src/lib/external-sources.ts`.

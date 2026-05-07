# Project Summary Helper

> **Source**: `src/lib/project-summary.ts`
> **Status**: [REVIEW]

## Overview

Pure helper module that derives project-layer preview data from a `ProjectDraft`, agent inventory, skills, scenes, disabled skills, and optional path inspection output.

## Public Surface

| Export | Purpose |
|---|---|
| `ProjectPreviewItem` | One skill preview row with source and disabled-state flags. |
| `ProjectAgentSummary` | Per-agent summary shape consumed by the Projects inspector. |
| `buildProjectSummary` | Computes inherited global contributions, project direct picks, project scenes, visible counts, target info, existing target entries, warnings, and unsupported agent keys. |

## Core Logic

The helper treats the project layer as an overlay on the global agent layer: global direct skills plus global scenes are previewed first, after subtracting the agent's own global exclusions, then project direct skills and project scenes are added. Project-local exclusions are subtracted from the visible contribution set so managed target cancellation immediately disappears from the desired preview, even before filesystem reconciliation runs. Path inspection data is attached per agent so the right inspector can show both expected write targets and existing project-local target skill entries without giving the summary component direct access to Tauri calls. Scene references resolve through `isSceneSkillEnabled`, so manual scene ordering and disabled scene skills match the scene module's current semantics.

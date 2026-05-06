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
| `buildProjectSummary` | Computes inherited global contributions, project direct picks, project scenes, visible counts, target info, warnings, and unsupported agent keys. |

## Core Logic

The helper treats the project layer as an overlay on the global agent layer: global direct skills plus global scenes are previewed first, after subtracting the agent's own global exclusions, then project direct skills and project scenes are added. Project-local exclusions remain in the data shape for legacy compatibility, but the visible Projects workbench does not present them as a control because they cannot stop an agent from reading skills that remain available through its global directory. Scene references resolve through `isSceneSkillEnabled`, so manual scene ordering and disabled scene skills match the scene module's current semantics.

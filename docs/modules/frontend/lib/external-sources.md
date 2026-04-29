# External Source UI Helpers

> **Source**: `src/lib/external-sources.ts`
> **Status**: [REVIEW]

## Overview

Provides tiny presentation helpers shared by the external-source and managed-skill UI surfaces.

## Public Surface

| Export | Purpose |
|---|---|
| `externalSourceName` | Builds a compact owner/repo-style label from `repoUrl`. |
| `shortCommit` | Truncates commit hashes to eight characters with a fallback label. |
| `statusClasses` | Maps source status values to Tailwind badge classes. |
| `warningSummary` | Collapses many warnings into a count label while preserving single-warning text. |

## Core Logic

This file stays presentation-only. It trims trailing slashes before deriving owner/repo labels and intentionally does not normalize or validate repository URLs beyond that display concern.

## Interactions

Used by `ExternalSourcesView`, `AgentExternalVariantPanel`, and `SkillDetailPanel`. Domain validation still belongs to Rust `core/external_sources/*`.

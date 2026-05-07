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
| `EXTERNAL_IMPORT_TARGETS` | Shared ordered list of agent keys that can receive a managed external import. |
| `agentLabel` | Maps an agent key to its display label for badges and selectors. |
| `sourceAgentLabels` | Deduplicates human-friendly agent labels from detected variants. |
| `resolveImportBusyState` | Computes whether one imported mirror row is currently updating or repairing. |

## Core Logic

This file stays presentation-only. It trims trailing slashes before deriving owner/repo labels, intentionally does not normalize or validate repository URLs beyond that display concern, maps agent and neutral skill-repository variant keys to user-facing labels, centralizes the allowed manual-import target list used by the sources UI, and derives summary badges from suggested target agents before falling back to the detector's raw variant key. It also centralizes the busy-state logic shared by `ExternalImportList` so repair and update labels stay truthful when a row is disabled mid-action.

## Interactions

Used by `ExternalSourcesView`, `AgentExternalVariantPanel`, and `SkillDetailPanel`. Domain validation still belongs to Rust `core/external_sources/*`.

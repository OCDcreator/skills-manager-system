# Agent Selection Summary

> **Source**: `src/components/agents/AgentSelectionSummary.tsx`
> **Status**: [REVIEW]

## Overview

Displays the deduped effective skill preview for one agent draft.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentSelectionSummary` | Final sync/exclude/global-ignore preview, per-row toggles, direct-skill deselection, and batch draft actions. |

## Core Logic

Renders per-skill source labels, sync/excluded/global-disabled badges, summary counts, exclusion/restore actions backed by the shared draft toggle helper, and deselect actions for skills selected directly on the agent. Deselecting a direct skill updates `selectedSkillIds` instead of only mutating exclusions, so a deleted target entry is not recreated by the next sync unless another source still selects it.

The preview also owns a small checkbox selection state for batch draft edits. Batch exclude adds selected non-global-disabled skills to `excludedSkillIds`, while batch deselect removes selected direct skills from `selectedSkillIds` and preserves scene-backed selections through exclusions. The preview list reuses the skill reading pane scrollbar styling and now remembers its scroll offset per agent draft.

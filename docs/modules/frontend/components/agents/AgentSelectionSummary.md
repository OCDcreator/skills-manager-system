# Agent Selection Summary

> **Source**: `src/components/agents/AgentSelectionSummary.tsx`
> **Status**: [REVIEW]

## Overview

Displays the deduped effective skill preview for one agent draft inside the capped three-panel agent row.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentSelectionSummary` | Final sync/synced/exclude/global-ignore preview, per-row toggles, direct-skill deselection, and batch draft actions. |

## Core Logic

Renders per-skill source labels, pending-sync / synced / excluded / global-disabled badges, summary counts, exclusion/restore actions backed by the shared draft toggle helper, and deselect actions for skills selected directly on the agent. Deselecting a direct skill updates `selectedSkillIds` instead of only mutating exclusions, so a deleted target entry is not recreated by the next sync unless another source still selects it.

The preview also owns a small checkbox selection state for batch draft edits plus a local status filter for `将同步`, `已同步`, `已排除`, and `全局忽略`. Clicking an active pill toggles back to the full list without touching the underlying draft. Batch exclude adds selected non-global-disabled skills to `excludedSkillIds`, while batch deselect removes selected direct skills from `selectedSkillIds` and preserves scene-backed selections through exclusions. Instead of depending on a guessed fixed preview height, the card now relies on the parent row's viewport cap, keeps the existing wrapped pill/action toolbar, and lets the remembered-scroll list fill only its own leftover height on `xl` screens while preserving a capped fallback on narrower layouts. It still reuses the skill reading pane scrollbar styling and surfaces a filter-scoped empty state when no resolved items match the current pill.

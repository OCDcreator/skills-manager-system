# Agent Selection Summary

> **Source**: `src/components/agents/AgentSelectionSummary.tsx`
> **Status**: [REVIEW]

## Overview

Displays the deduped effective skill preview for one agent draft.

## Public Surface

| Export | Purpose |
|---|---|
| `AgentSelectionSummary` | Final sync/exclude/global-ignore preview and exclusion toggles. |

## Core Logic

Renders per-skill source labels, sync/excluded/global-disabled badges, summary counts, and exclusion/restore actions backed by the shared draft toggle helper.

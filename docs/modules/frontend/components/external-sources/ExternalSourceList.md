# ExternalSourceList

> **Source**: `src/components/external-sources/ExternalSourceList.tsx`
> **Status**: [REVIEW]

## Overview

Provides the list shell for the dedicated external-sources page.

## Public Surface

| Export | Purpose |
|---|---|
| `ExternalSourceList` | Empty-state wrapper plus card mapper for source snapshots. |

## Core Logic

This component is intentionally thin. It renders the empty-state callout when no sources exist and otherwise maps each snapshot to `ExternalSourceCard`, forwarding all source, import, repo-path, and external variant props without adding business rules.

For compact behavior it keeps a simple one-column `grid gap-4` wrapper at every width. The cards themselves handle their own dense summary/expand behavior, so this list does not need an extra `900px-1279px` mode.

## Interactions

Used by `ExternalSourcesView`. Keep the prop contract aligned with `ExternalSourceCard` instead of growing page-level branching logic here.

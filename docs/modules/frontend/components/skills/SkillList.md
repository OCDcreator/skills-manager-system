# SkillList

> **Source**: `src/components/skills/SkillList.tsx`
> **Status**: [REVIEW]

## Overview

Displays a selectable grid of skill summaries for one source group, including enable/disable state, source-scoped bulk actions, checkbox selection mode, and external-source badges.

## Public Surface

| Export | Purpose |
|---|---|
| `SkillList` | List component for one titled skill group. |

## Core Logic

The component renders a responsive fixed-row card grid, highlights the selected skill, dims disabled skills, and forwards toggle/select actions to the parent. Each source window has its own header controls for enabling all visible skills, disabling all visible skills, and entering multi-select mode. In multi-select mode, the selected count remains inline in the title row, cards expose a compact checkbox in the top-right corner instead of spending a full content row on selection controls, and the bulk buttons retarget to only the selected skills. Before the first scan returns, the list renders a compact status panel and a scanning label instead of the empty-filter message or a zero count.

The card grid now has an explicit compact-density contract:

- base / compact band: `minmax(15rem, 1fr)` cards with `11rem` rows below `1280px`
- wide band: `minmax(18rem, 1fr)` cards with `13.5rem` rows at `>=1280px`

That keeps more scannable cards visible on smaller laptop widths without changing the per-card information hierarchy. Titles, badges, description, and path regions still stay truncated and vertically pinned so custom and external cards remain the same visual size even when external badges are present; the path stays directly under the description while only the per-card enable/disable button disappears in multi-select mode. Each source window keeps only the card grid internally scrollable with the shared `skill-markdown-scroll` scrollbar skin, and the scroll container still opts into `useRememberedScrollPosition()` so refreshes and remounts return the list to its prior vertical position. External skills keep additive badge treatment: unmanaged external entries show a manual-external badge, while managed GitHub mirrors show a separate managed-source badge derived from `skill.managedSource`.

## Interactions

The list still does not fetch documents or persist state. Bulk operations are source-scoped to the `skills` prop currently rendered by the list, and selection state is local to the list window. Badge semantics must stay aligned with `scan_repo_skills_with_external_sources()` and the `ManagedSourceInfo` payload in `src/lib/tauri.ts`.

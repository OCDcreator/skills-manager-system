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

The component renders a responsive fixed-row card grid, highlights the selected skill, dims disabled skills, and forwards toggle/select actions to the parent. Each source window has its own header controls for enabling all visible skills, disabling all visible skills, and entering multi-select mode. In multi-select mode, cards expose a compact checkbox in the top-right corner instead of spending a full content row on selection controls, and the bulk buttons retarget to only the selected skills. Card rows use a fixed height with truncated title, badge, description, and path regions so custom and external skill cards stay the same visual size even when external badges are present. Each source window uses an expanded but slightly softened viewport-aware height cap that assumes the filter toolbar is compact, and keeps only the card grid internally scrollable with the shared `skill-markdown-scroll` scrollbar skin. External skills now get additive badge treatment: unmanaged external entries show a manual-external badge, while managed GitHub mirrors show a separate managed-source badge derived from `skill.managedSource`.

## Interactions

The list still does not fetch documents or persist state. Bulk operations are source-scoped to the `skills` prop currently rendered by the list, and selection state is local to the list window. Badge semantics must stay aligned with `scan_repo_skills_with_external_sources()` and the `ManagedSourceInfo` payload in `src/lib/tauri.ts`.

# SkillList

> **Source**: `src/components/skills/SkillList.tsx`
> **Status**: [REVIEW]

## Overview

Displays a selectable grid of skill summaries for one source group, including enable/disable state and external-source badges.

## Public Surface

| Export | Purpose |
|---|---|
| `SkillList` | List component for one titled skill group. |

## Core Logic

The component renders a responsive card grid, highlights the selected skill, dims disabled skills, and forwards toggle/select actions to the parent. Each source window caps its height and keeps only the card grid internally scrollable with the shared `skill-markdown-scroll` scrollbar skin, so long custom/external buckets no longer stretch the whole Skills view. External skills now get additive badge treatment: unmanaged external entries show a manual-external badge, while managed GitHub mirrors show a separate managed-source badge derived from `skill.managedSource`.

## Interactions

The list still does not fetch documents or persist state. Badge semantics must stay aligned with `scan_repo_skills_with_external_sources()` and the `ManagedSourceInfo` payload in `src/lib/tauri.ts`.

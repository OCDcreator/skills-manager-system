# SkillDetailPanel

> **Source**: `src/components/skills/SkillDetailPanel.tsx`
> **Status**: [REVIEW]

## Overview

Displays details, enabled/disabled metadata, and markdown content for the currently selected skill.

## Import Relationships

```text
Upstream: src/views/SkillsView.tsx
Downstream: react-markdown, remark-gfm, src/lib/tauri.ts, src/i18n/index.ts
```

## Public Surface

| Export | Purpose |
|---|---|
| `SkillDetailPanel` | Read-only panel for a selected skill summary and optional loaded document. |

## Core Logic

When no skill is selected, the panel renders a translated selection prompt. When a skill exists, it shows name, description, source type, current enabled/disabled status, relative path, and either the loaded markdown document or a loading message.

## Data Flow

`SkillsView` passes `SkillSummary` and `SkillDocument` values from context. Markdown content is rendered with GFM support.

## Interactions

Depends on `SkillSummary` and `SkillDocument` type shapes from `src/lib/tauri.ts` plus `skills.detail.*` and `skills.status.*` i18n keys.

## Configuration

Markdown styling is controlled by Tailwind Typography class names.

## Change Notes

Any change to document fields returned by `get_skill_document` must update this panel and the TypeScript interface together.

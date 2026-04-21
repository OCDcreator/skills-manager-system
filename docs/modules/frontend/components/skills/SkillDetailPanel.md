# SkillDetailPanel

> **Source**: `src/components/skills/SkillDetailPanel.tsx`
> **Status**: [REVIEW]

## Overview

Displays details, enabled/disabled metadata, and markdown content for the currently selected skill.

## Import Relationships

```text
Upstream: src/views/SkillsView.tsx
Downstream: marked, highlight.js, github-markdown-css, src/lib/tauri.ts, src/i18n/index.ts
```

## Public Surface

| Export | Purpose |
|---|---|
| `SkillDetailPanel` | Read-only panel for a selected skill summary and optional loaded document. |

## Core Logic

When no skill is selected, the panel renders a translated selection prompt. When a skill exists, it shows name, description, source type, current enabled/disabled status, relative path, a frontmatter wrap toggle, and either the loaded markdown document or a loading message. On wide windows the panel docks to the viewport and turns the markdown body into an internal scroll area. The preview uses `marked` for HTML generation, `highlight.js` for fenced-code highlighting, and `github-markdown-css` for GitHub-style markdown presentation.

## Data Flow

`SkillsView` passes `SkillSummary` and `SkillDocument` values from context. Before rendering, leading YAML frontmatter is converted into a fenced `yaml` code block tagged as frontmatter so it stays visible in the preview while the rest of the document keeps normal markdown rendering. The local wrap toggle controls only that frontmatter code block.

## Interactions

Depends on `SkillSummary` and `SkillDocument` type shapes from `src/lib/tauri.ts` plus `skills.detail.*` and `skills.status.*` i18n keys.

## Configuration

Markdown styling is controlled by the shared stylesheet via `github-markdown-css` plus local `.skill-markdown-body` / scrollbar overrides. The panel must preserve `min-w-0`, keep the main reading scrollbar on the outer right edge of the reading frame instead of the inner article content, contain horizontal overflow inside code/table blocks, keep frontmatter wrapping enabled by default, and cap its viewport height on wide screens so markdown reading happens inside the detail rail instead of stretching the whole page.

## Change Notes

Any change to document fields returned by `get_skill_document` must update this panel and the TypeScript interface together.

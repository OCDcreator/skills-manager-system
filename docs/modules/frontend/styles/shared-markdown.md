# Shared Markdown Styles

> **Source**: `src/styles/shared-markdown.css`
> **Status**: [REVIEW]

## Overview

Hosts the shared markdown presentation layer used by the skills detail panel and the reusable scroll containers used across markdown-heavy or sidecar-heavy surfaces.

## Import Relationships

```text
Upstream: src/styles.css
Downstream: src/components/skills/SkillDetailPanel.tsx, src/components/git/GitFileList.tsx, src/components/git/GitDiffViewer.tsx, src/components/agents/AgentGlobalSkillList.tsx, src/components/agents/AgentSelectionSummary.tsx, src/components/agents/AgentSkillSelector.tsx, src/components/agents/AgentOrderModal.tsx, src/components/agents/AgentSceneSelector.tsx, src/components/projects/ProjectAssignmentSummary.tsx, src/components/projects/ProjectAssignmentEditor.tsx
```

## Core Logic

This file keeps three tightly related concerns together:

- `.skill-markdown-body` overrides for dark markdown rendering on top of the shared GitHub markdown theme
- `.skill-frontmatter-*` rules for wrapped versus horizontally scrolling frontmatter blocks
- `.skill-markdown-scroll` custom scrollbar treatment shared by multiple domains

Grouping them here keeps the cross-module markdown/scroll contract centralized without restoring a catch-all global stylesheet.

## Interactions

If another module needs the same scrollbar skin, reuse `.skill-markdown-scroll` instead of duplicating the scrollbar selectors. If a future markdown surface needs a meaningfully different visual treatment, give it a new scoped selector instead of expanding these rules into a generic markdown bucket.

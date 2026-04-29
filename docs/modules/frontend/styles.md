# Frontend Styles

> **Source**: `src/styles.css`
> **Status**: [REVIEW]

## Overview

Provides the frontend stylesheet entrypoint. It keeps vendor markdown/highlight imports and routes app-owned CSS into a small set of responsibility-based files under `src/styles/`.

## Import Relationships

```text
Upstream: src/main.tsx
Downstream: src/styles/foundation.css, src/styles/shared-markdown.css, src/styles/agents.css, github-markdown-css, highlight.js theme CSS
```

## Core Logic

The entrypoint stays intentionally thin so the app keeps one stable global import while actual styles are grouped by domain:

- `foundation.css` for app-wide element defaults and root sizing
- `shared-markdown.css` for markdown rendering, frontmatter handling, and shared custom scrollbars used across skills, projects, git, and agent side panels
- `agents.css` for the floating agent rail scrollbar-hiding behavior

## Interactions

This file must remain compatible with the Tailwind pipeline because it also hosts the `@tailwind base/components/utilities` directives. Any new cross-module CSS should be added to an existing domain file when possible instead of restoring a large monolithic stylesheet.

## Change Notes

If a new style domain grows beyond a small addition to these files, prefer introducing another responsibility-based CSS file under `src/styles/` and importing it here rather than appending a new unrelated block to one of the existing files.

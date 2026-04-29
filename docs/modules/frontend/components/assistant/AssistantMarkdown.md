# AssistantMarkdown

> **Source**: `src/components/assistant/AssistantMarkdown.tsx`
> **Status**: [DRAFT]

## Overview

Renders markdown content as sanitized HTML inside assistant message bubbles. Uses `marked` with `highlight.js/lib/common` for code syntax highlighting so the assistant keeps syntax color without pulling in the full highlight language bundle. Raw HTML in content is escaped via `escapeHtml` before passing to `marked.parse`, preventing any user-provided HTML from becoming active DOM.

## Import Relationships

```text
Upstream: src/components/assistant/ProjectAssistantPanel.tsx
Downstream: marked, highlight.js/lib/common
```

## Public Surface

| Export | Purpose |
|---|---|
| `AssistantMarkdown` | Takes `content` string, renders as dark-theme markdown HTML. |

## Core Logic

Escapes `&<>"` in the raw content before passing to `marked.parse()`, so no raw HTML from the backend response can render as active DOM. Creates a shared `Marked` instance with GFM support and a custom code renderer that applies `highlight.js/lib/common`. Uses `dangerouslySetInnerHTML` with Tailwind utility overrides for dark-theme typography inside the `markdown-body` class.

## Data Flow

Markdown string in → `escapeHtml()` → `marked.parse()` → safe HTML string → rendered inside constrained dark bubble.

## Interactions

Relies on `github-markdown-css/github-markdown-dark.css` imported in `src/styles.css`.

## Configuration

None.

# AssistantMarkdown

> **Source**: `src/components/assistant/AssistantMarkdown.tsx`
> **Status**: [DRAFT]

## Overview

Renders markdown content as sanitized HTML inside assistant message bubbles. Uses `marked` with `highlight.js` for code syntax highlighting. Raw HTML in content is escaped via `escapeHtml` before passing to `marked.parse`, preventing any user-provided HTML from becoming active DOM.

## Import Relationships

```text
Upstream: src/components/assistant/ProjectAssistantPanel.tsx
Downstream: marked, highlight.js
```

## Public Surface

| Export | Purpose |
|---|---|
| `AssistantMarkdown` | Takes `content` string, renders as dark-theme markdown HTML. |

## Core Logic

Escapes `&<>"` in the raw content before passing to `marked.parse()`, so no raw HTML from the backend response can render as active DOM. Creates a shared `Marked` instance with GFM support and a custom code renderer that applies `highlight.js`. Uses `dangerouslySetInnerHTML` with Tailwind utility overrides for dark-theme typography inside the `markdown-body` class.

## Data Flow

Markdown string in → `escapeHtml()` → `marked.parse()` → safe HTML string → rendered inside constrained dark bubble.

## Interactions

Relies on the markdown vendor styles imported through the `src/styles.css` entrypoint. Assistant-specific typography remains local to the component via Tailwind utilities instead of introducing another global CSS file.

## Configuration

None.

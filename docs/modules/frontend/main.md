# Frontend Main

> **Source**: `src/main.tsx`
> **Status**: [REVIEW]

## Overview

Bootstraps the React application into the DOM and loads global i18n and the CSS entrypoint side effects.

## Import Relationships

```text
Upstream: index.html
Downstream: src/App.tsx, src/i18n/index.ts, src/styles.css
```

## Public Surface

| Export | Purpose |
|---|---|
| None | This module is an application entrypoint. |

## Core Logic

The module creates a React root from `#root` and renders `App` inside `React.StrictMode`.

## Data Flow

No app data is transformed here. It only starts the frontend tree.

## Interactions

Requires `index.html` to provide an element with `id="root"`.

## Configuration

Imports i18n initialization plus the stylesheet entrypoint. `src/styles.css` now fans out into the modular CSS files under `src/styles/` so the entrypoint stays stable while global style domains remain split by responsibility.

## Change Notes

If the root element id or providers change, update this entrypoint and the hosting HTML together.

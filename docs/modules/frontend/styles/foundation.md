# Frontend Style Foundation

> **Source**: `src/styles/foundation.css`
> **Status**: [REVIEW]

## Overview

Defines the app-wide CSS foundation layer: root palette defaults, application font stack, root height behavior, and baseline element inheritance shared by every page.

## Import Relationships

```text
Upstream: src/styles.css
Downstream: browser root/body/button/input elements, #root
```

## Core Logic

All rules live in Tailwind's `@layer base` so they compose with preflight instead of fighting it. The file intentionally stays small and only owns defaults that truly apply across the whole app.

## Interactions

Do not move feature-specific colors, spacing, or component skins here. Anything narrower than app-wide foundation should stay in Tailwind classes or a narrower CSS domain file.

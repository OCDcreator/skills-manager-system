# Frontend Style Foundation

> **Source**: `src/styles/foundation.css`
> **Status**: [REVIEW]

## Overview

Defines the app-wide CSS foundation layer: root palette defaults, application font stack, root height behavior, baseline element inheritance, and the shared workbench shell layout primitives used by `AppShell`.

## Import Relationships

```text
Upstream: src/styles.css
Downstream: browser root/body/button/input elements, #root
```

## Core Logic

Base rules live in Tailwind's `@layer base` so they compose with preflight instead of fighting it. The root now exposes OKLCH workbench tokens for the tinted neutral product palette, with a restrained green accent and non-pure dark surfaces.

The `@layer components` rules define stable `app-shell*` classes for the responsive desktop workbench shell: mobile receives a compact top header with horizontally scrollable icon navigation that hides scrollbar chrome without disabling scroll, while desktop switches to a sticky, scrollable left rail and keeps the main content width controlled by the React shell. Shared nav items also define a visible focus outline in the workbench accent color, and the mobile nav keeps extra scrollport padding so keyboard focus rings are not clipped by horizontal overflow.

## Interactions

Keep app-wide shell chrome here so `AppShell` stays readable and avoids scattered utility strings. Feature-specific colors, spacing, and component skins should still stay in Tailwind classes or a narrower CSS domain file unless they are part of the global shell contract.

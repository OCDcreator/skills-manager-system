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

The `@layer components` rules define stable `app-shell*` classes for the responsive desktop workbench shell. Below `900px`, the shell uses a compact top header with horizontally scrollable icon navigation that hides scrollbar chrome without disabling scroll. From `900px` to `1279px`, the shell switches to a sticky compact icon rail that gives page content back most of the reclaimed width. At `1280px` and above, the full left rail returns with labels and subtitle restored.

Shared nav items define a visible focus outline in the workbench accent color, the mobile nav keeps extra scrollport padding so keyboard focus rings are not clipped by horizontal overflow, and the compact rail hides persistent labels instead of squeezing bilingual text into a narrow column.

## Interactions

Keep app-wide shell chrome here so `AppShell` stays readable and avoids scattered utility strings. Feature-specific colors, spacing, and component skins should still stay in Tailwind classes or a narrower CSS domain file unless they are part of the global shell contract.

# scroll-memory.ts

> **Source**: `src/lib/scroll-memory.ts`
> **Status**: [DRAFT]

## Overview

Provides a small frontend hook for remembering internal panel scroll positions across remounts and refreshes.

## Responsibilities

- expose `useRememberedScrollPosition(id)` for scrollable `HTMLElement` refs
- persist `scrollTop` and `scrollLeft` in `localStorage`
- retry restoration across a short `requestAnimationFrame` window so late layout settling does not snap panels back to the top

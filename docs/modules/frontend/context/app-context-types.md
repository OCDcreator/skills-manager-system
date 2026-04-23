# App Context Types

> **Source**: `src/context/app-context-types.ts`
> **Status**: [REVIEW]

## Overview

Defines the frontend context contract separately from the provider implementation so `AppContext.tsx` can stay within the repo's size guardrails.

## Public Surface

| Export | Purpose |
|---|---|
| `AppView` / `NavigationGuard` / `PendingNavigation` | Re-exported shared navigation types. |
| `AppContextValue` | Full consumer-facing shape exposed through `useAppContext()`. |

## Core Logic

Bundles app-wide view state, repo path, skill scan state, agent inventory, sync actions, and navigation-guard actions into one typed interface used by the provider and all consumers.

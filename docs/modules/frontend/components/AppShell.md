# AppShell

> **Source**: `src/components/AppShell.tsx`
> **Status**: [REVIEW]

## Overview

Provides the shared page chrome for the desktop app, including the header, guarded navigation buttons, global error banner, main content container, and unsaved-changes modal host.

## Public Surface

| Export | Purpose |
|---|---|
| `AppShell` | Layout component accepting `children` and an optional content-width class override. |

## Core Logic

The shell reads `activeView`, `errorMessage`, guarded `setActiveView`, and pending-navigation actions from context. It renders the translated app title/subtitle, six view buttons, an error strip, and the unsaved-changes dialog whenever the Agents page blocks a route change.

## Interactions

User clicks on navigation buttons call guarded `setActiveView`, which may immediately change the current route or open the unsaved-changes dialog first. Navigation choices must stay aligned with the `AppView` union.

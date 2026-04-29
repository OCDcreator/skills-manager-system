# Agent Styles

> **Source**: `src/styles/agents.css`
> **Status**: [REVIEW]

## Overview

Provides the small agent-specific global style domain that hides scrollbars for the floating agent navigation rail while preserving scrollability on shorter viewports.

## Import Relationships

```text
Upstream: src/styles.css
Downstream: src/components/agents/AgentFloatingNav.tsx
```

## Core Logic

The file only owns `.agent-floating-nav-scroll` because that behavior is both cross-browser and easier to maintain in CSS than as ad hoc inline style fragments. Keeping it isolated avoids leaking agent-specific behavior into the shared markdown or foundation layers.

## Interactions

If more agent-only global selectors are needed later, add them here before considering a broader shared file.

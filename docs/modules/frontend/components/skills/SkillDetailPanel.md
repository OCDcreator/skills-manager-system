# SkillDetailPanel

> **Source**: `src/components/skills/SkillDetailPanel.tsx`
> **Status**: [REVIEW]

## Overview

Displays the selected skill's metadata and markdown document, including additive metadata for managed external mirrors.

## Public Surface

| Export | Purpose |
|---|---|
| `SkillDetailPanel` | Read-only detail rail for one selected skill and optional loaded document. |

## Core Logic

The panel still owns markdown preview, syntax highlighting, and the frontmatter wrap toggle, but it now also consults `AppContext.externalSources` to resolve the live import record for `skill.managedSource.importId`. Managed external skills render repo URL, pinned commit, agent key, optional upstream variant path, update-available badge, and deduplicated warnings derived from both persisted import warnings and runtime integrity mismatches.

## Interactions

This panel treats managed GitHub imports as enriched `external` skills; it does not mutate them. Update and repair actions remain on the sources and agents surfaces. It must stay aligned with `ManagedSourceInfo`, `ExternalSourceWarning`, and the warning-code semantics from Rust. Its markdown preview and frontmatter wrapping also depend on the shared selectors defined under the `src/styles.css` entrypoint, especially the `shared-markdown.css` domain file.

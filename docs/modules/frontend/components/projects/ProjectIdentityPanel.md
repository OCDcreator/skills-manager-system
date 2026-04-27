# Project Identity Panel

> **Source**: `src/components/projects/ProjectIdentityPanel.tsx`
> **Status**: [REVIEW]

## Overview

Path-first identity editor for the Projects workbench. Lets the user confirm project path and display name before choosing skills and agents.

## Responsibilities

- renders the primary `projectPath` input
- renders the secondary `displayName` input
- shows inferred-name, duplicate-path, read-only, and inspection-status hints
- keeps the path field read-only while editing an existing saved assignment

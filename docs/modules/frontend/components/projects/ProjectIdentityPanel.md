# Project Identity Panel

> **Source**: `src/components/projects/ProjectIdentityPanel.tsx`
> **Status**: [REVIEW]

## Overview

Path-first identity editor for the Projects workbench. Lets the user confirm project path and display name before choosing skills and agents.

## Responsibilities

- renders a path-first identity layout where `projectPath` owns the primary hero row
- exposes a create-only compact folder-picker button inside the path input group
- renders the secondary `displayName` input on its own follow-up row with the same dark input treatment as the path field
- shows inferred-name, duplicate-path, read-only, and inspection-status hints
- keeps the path field read-only while editing an existing saved assignment

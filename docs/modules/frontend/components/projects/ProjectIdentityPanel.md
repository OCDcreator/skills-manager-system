# Project Identity Panel

> **Source**: `src/components/projects/ProjectIdentityPanel.tsx`
> **Status**: [REVIEW]

## Overview

Path-first identity editor for the Projects workbench. Lets the user confirm project path and display name before choosing skills and agents, and keeps the primary save CTA beside the display-name row.

## Responsibilities

- renders a path-first identity layout where `projectPath` owns the primary hero row
- exposes a create-only compact folder-picker button inside a 3rem-tall path input group
- renders the secondary `displayName` input on its own compact row with the same 3rem-tall dark input treatment as the path field
- hosts the primary save button to the right of the display-name field so saving stays near the draft identity inputs
- shows a secondary cancel-edit action in edit mode so users can return to a new assignment draft without saving local edits
- shows inferred-name and edit-mode read-only hints as left-aligned inline guidance inside the empty display-name field, then hides them once the user types
- shows duplicate-path and inspection-status hints below the identity inputs
- keeps the path field read-only while editing an existing saved assignment

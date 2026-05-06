# Project Skill Filter Toolbar

> **Source**: `src/components/projects/ProjectSkillFilterToolbar.tsx`
> **Status**: [REVIEW]

## Overview

Dedicated filter toolbar for the Projects workbench skill chooser.

## Responsibilities

- renders the project-skill search field
- renders top-level path pills such as `custom` and `external`
- opens a scene-style external group popover only after the `external` pill is active
- renders selected-state pills for `all / selected / unselected`
- forwards all filter changes back to `ProjectsView`

## Interaction Notes

The external group popover mirrors `SceneSkillChooser`: it is anchored to the active external group button, floats above the filter row, and lists summaries produced by `buildExternalGroupSummaries()`. Selecting `all` or `custom` resets the external group filter to `all`, so hidden subgroup state does not keep affecting the visible skill list.

# Agent Discovery Tests

> **Source**: `src-tauri/src/core/agents/discovery_tests.rs`
> **Status**: [REVIEW]

## Overview

Focused unit tests for agent inventory discovery.

## Responsibilities

- verifies detected/default/missing path modes
- verifies path overrides take precedence
- verifies hidden home-directory agents such as Kimi detect correctly
- verifies inventory exposes both global and project-local directory rules

## Interactions

Kept separate from `discovery.rs` so the production module remains below the architecture warning line while preserving coverage for catalog and config interactions.

# Agent Sync Tests

> **Source**: `src-tauri/src/core/agents/sync_tests.rs`
> **Status**: [REVIEW]

## Overview

Holds focused Rust tests for the per-agent sync workflow.

## Public Surface

This is a test-only module and exports no runtime API.

## Core Logic

Builds disposable repositories and target directories to verify per-agent direct selection, explicit scene selections, scene/direct dedupe, exclusions, global hard-disable filtering, scoped apply, disabled-agent cleanup, override-path precedence, and target entries that use the source skill folder name. The fixture helpers are kept compact because this test module sits at the architecture warning boundary.

## Data Flow

Test fixtures create repo/config/scene/target state, invoke `apply_agent_sync` or `apply_agent_sync_for_agent`, and assert filesystem results.

## Current Note

The current branch change for this module is formatting-only; runtime behavior is unchanged.

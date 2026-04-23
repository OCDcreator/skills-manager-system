# Agent Skill Selection

> **Source**: `src-tauri/src/core/agents/selection.rs`
> **Status**: [REVIEW]

## Overview

Resolves each agent's desired skill list from direct selections, selected scenes, exclusions, and the global hard-disable gate.

## Public Surface

This module is `pub(crate)` to the agents domain and exposes selection context loading plus agent resolution helpers.

## Core Logic

Loads repository skills, repo-scoped disabled skill IDs, and scene definitions; computes globally available skill count; resolves `direct ∪ scenes - exclusions`; and filters missing or globally disabled skill IDs before sync builds target entries.

## Interactions

Used by `sync.rs` so that high-level apply orchestration stays focused on ledger and target reconciliation rather than selection rules.

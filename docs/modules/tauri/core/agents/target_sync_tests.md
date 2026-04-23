# Agent Target Sync Tests

> **Source**: `src-tauri/src/core/agents/target_sync_tests.rs`
> **Status**: [REVIEW]

## Overview

Holds focused Rust tests for the low-level target reconciliation safety rules.

## Public Surface

This is a test-only module and exports no runtime API.

## Core Logic

Exercises `apply_desired_entries` directly to prove that removing managed entries never deletes unrelated manual content already present in the target directory.

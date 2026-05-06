# Project Apply Status

> **Source**: `src-tauri/src/core/projects/apply_status.rs`
> **Status**: [REVIEW]

## Overview

Computes transient project-agent apply freshness from the project sync ledger.

## Public Surface

This module is internal to `core::projects`. It exposes helpers to `sync.rs` for:

- deriving per-agent `current`, `stale`, `neverApplied`, and `unsupported` statuses
- building stable resolution hashes from project path, agent key, resolved skill sources, project exclusions, target directory, and sync mode

## Interactions

`sync.rs` owns loading and saving the ledger. This module only compares the current resolved project layer against ledger entries and returns DTOs for project config responses.

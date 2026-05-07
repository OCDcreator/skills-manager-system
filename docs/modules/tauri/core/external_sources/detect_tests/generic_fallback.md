# Detect Generic Fallback Tests

> **Source**: `src-tauri/src/core/external_sources/detect_tests/generic_fallback.rs`
> **Status**: [REVIEW]

## Overview

Exercises recursive generic fallback detection for external skill repositories.

## Public Surface

This file is test-only and exports no production API.

## Core Logic

These tests prove that generic fallback candidates can coexist with supported layouts, that recursive nested `SKILL.md` folders are discovered under the selected scan root, that noisy infrastructure directories are skipped, and that root-level repositories still surface the neutral `.` candidate. The subpath test also locks in the broader recursive behavior for configured subtrees rather than the older direct-child-only scan.

## Interactions

Loaded by `detect_tests.rs`. Keep it aligned with `generic_skill_detection.rs` and the source-page expectation that manual fallback import rows represent every importable `SKILL.md` folder not already claimed by a supported generated layout.

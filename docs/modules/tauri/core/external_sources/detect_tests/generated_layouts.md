# Detect Generated Layout Tests

> **Source**: `src-tauri/src/core/external_sources/detect_tests/generated_layouts.rs`
> **Status**: [REVIEW]

## Overview

Exercises supported generated-agent layouts, alias support, and mixed warning behavior for external-source detection.

## Public Surface

This file is test-only and exports no production API.

## Core Logic

These tests cover the supported generated-bundle rule table across legacy `dist/agents/...`, root hidden folders, aligned `dist/*` variants, and `.codex` alias roots. They also verify that unknown generated-looking directories still raise `unsupported_agent_variant` warnings while remaining eligible for generic fallback import, and that supported generated layouts continue to win the overall source kind even when generic fallback candidates are also present.

## Interactions

Loaded by `detect_tests.rs`. Keep it aligned with `generated_agent_detection.rs` and the UI expectation that exact/alias-supported candidates are surfaced as managed-agent imports rather than neutral repository candidates.

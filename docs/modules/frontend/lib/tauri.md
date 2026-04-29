# Tauri API Wrapper

> **Source**: `src/lib/tauri.ts`
> **Status**: [REVIEW]

## Overview

Defines TypeScript DTOs plus thin `invoke()` wrappers for the frontend's Tauri commands.

## Public Surface

| Export | Purpose |
|---|---|
| `ManagedSourceInfo` | Skill-level metadata for managed GitHub mirrors shown in skills UI. |
| `ExternalSourceRecord` / `ExternalSourceSnapshotItem` | Source-page and agent-page snapshot shapes. |
| `ImportedExternalSkillRecord` / `ExternalImportResult` | Managed import state and mutation result DTOs. |
| `listExternalSources` / `addExternalSource` / `fetchExternalSource` | Source-record command wrappers. |
| `importExternalVariant` / `updateExternalImport` / `removeExternalSource` / `repairExternalImport` | Managed-import command wrappers. |
| other exports | Existing repo, skill, scene, settings, and agent DTOs plus wrappers. |

## Core Logic

The file keeps frontend wiring thin while expanding the shared contract for external-source management. `SkillSummary` and `SkillDocument` now carry optional `managedSource` metadata, and `AgentKey` is reused across both sync and external-import flows so source variants can target the same agent catalog as the rest of the app.

## Interactions

Must stay aligned with `src-tauri/src/commands/external_sources.rs`, `src-tauri/src/core/external_sources/service.rs`, the skill scanner payloads in `src-tauri/src/core/skills/scan.rs`, and the command registrations in `src-tauri/src/lib.rs`.

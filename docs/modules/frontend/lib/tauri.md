# Tauri API Wrapper

> **Source**: `src/lib/tauri.ts`
> **Status**: [REVIEW]

## Overview

Defines TypeScript DTOs plus thin `invoke()` wrappers for the frontend's Tauri commands.

## Public Surface

| Export | Purpose |
|---|---|
| `ManagedSourceInfo` | Skill-level metadata for managed GitHub mirrors shown in skills UI. |
| `ExternalVariantKey` | External-source variant key type, including real agent keys plus the neutral `skill_repository` key. |
| `ExternalCandidateDetectionClass` | External candidate provenance type for exact, alias, or generic fallback detection. |
| `ExternalSourceRecord` / `ExternalSourceSnapshotItem` | Source-page and agent-page snapshot shapes. |
| `AddExternalSourceInput` | Frontend input shape for adding a source URL with optional branch and repository subpath. |
| `ImportedExternalSkillRecord` / `ExternalImportResult` | Managed import state and mutation result DTOs. |
| `loadCachedSkills` / `scanSkills` | Skill browser startup hydration and incremental refresh wrappers. |
| `listExternalSources` / `addExternalSource` / `fetchExternalSource` | Source-record command wrappers. |
| `importExternalVariant` / `updateExternalImport` / `removeExternalSource` / `repairExternalImport` | Managed-import command wrappers. |
| other exports | Existing repo, skill, scene, settings, and agent DTOs plus wrappers. |

## Core Logic

The file keeps frontend wiring thin while expanding the shared contract for external-source management. `ExternalSourceRecord` includes the configured branch and subpath beside the fetched default branch so the UI can distinguish user intent from upstream state. `SkillSummary` and `SkillDocument` carry optional `managedSource` metadata, and external-source variants use `ExternalVariantKey` so generated agent bundles can still target real agent keys while ordinary skill repositories use the neutral `skill_repository` key. `loadCachedSkills` returns the previous skill scan snapshot when one exists, while `scanSkills` performs the cached incremental refresh. `ExternalVariantSnapshot` now also carries optional `name` and `description` fields parsed from upstream `SKILL.md` metadata, a `contentFingerprint` for same-content variant comparison, paired `childDirectories` and `childFiles` preview lists for the variant folder itself, plus `detectionClass`, `suggestedTargetAgents`, and `detectedAgentHint` so the sources UI can separate exact supported layouts, alias-compatible layouts such as `.codex`, and fully manual fallback candidates without inventing extra frontend heuristics. `AgentTargetSkillEntry` includes an optional `symlinkTargetPath` so the Agent Sync sidecar can show where visible directory links point without adding another command round-trip. `AgentInventoryItem` exposes both the global `skillsDirRule` and the project-local `projectSkillsDirRule` so agent sync and project assignment surfaces do not conflate global config roots with project overlays.

## Interactions

Must stay aligned with `src-tauri/src/commands/external_sources.rs`, `src-tauri/src/core/external_sources/service.rs`, the skill scanner payloads in `src-tauri/src/core/skills/scan.rs`, and the command registrations in `src-tauri/src/lib.rs`.

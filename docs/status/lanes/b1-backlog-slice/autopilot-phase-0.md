# Autopilot Baseline: Phase 0

> **Status**: [BASELINE]
> **Preset**: `Bugfix / Backlog`
> **Lane**: `b1-backlog-slice`
> **Repository**: `skills-manager-system`

## Objective

- Implement the approved CLI Foundation plan from docs/superpowers/plans/2026-04-21-cli-foundation.md one validated slice at a time, keeping the desktop app healthy while adding a machine-first headless CLI.

## Lane scope

- Execute the highest-priority reproducible bugfix or backlog slice with bounded scope.

## Seeded entrypoints

- `AGENTS.md`
- `docs/`
- `src/`
- `scripts/check-architecture.mjs`
- `src/components/scenes/SceneCard.tsx`
- `src/context/AppContext.tsx`

## Inferred validation commands

- Lint: `npm run lint` (source: `CLI override`)
- Typecheck: `cargo check --manifest-path src-tauri/Cargo.toml` (source: `CLI override`)
- Full test: `npm run verify` (source: `CLI override`)
- Build: `npm run build` (source: `CLI override`)
- Vulture: not inferred

## Notes

- This document captures the baseline for lane `b1-backlog-slice`.
- The first unattended round in this lane should write `docs/status/lanes/b1-backlog-slice/autopilot-phase-1.md`.

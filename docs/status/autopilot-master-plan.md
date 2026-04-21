# Autopilot Master Plan

> **Preset**: `Bugfix / Backlog`
> **Repository**: `skills-manager-system`
> **Controller mode**: Explicit sequential lanes from `automation/autopilot-config.json`
> **Note**: This file is a human-facing cross-lane overview, not the live `[NEXT]` truth source.

## Overall objective

- Implement the approved CLI Foundation plan from docs/superpowers/plans/2026-04-21-cli-foundation.md one validated slice at a time, keeping the desktop app healthy while adding a machine-first headless CLI.
- Keep each queued slice small, reproducible, and easy to validate
- Prefer the highest-confidence bugfix or backlog item first

## Lane order

- `b1-backlog-slice` — highest-priority reproducible bug or backlog slice
- `b2-backlog-slice` — the next queued slice after B1 lands
- `b3-checkpoint` — document what shipped and whether unattended continuation still makes sense

## Shared entrypoints

- `AGENTS.md`
- `docs/superpowers/specs/2026-04-21-cli-foundation-design.md`
- `docs/superpowers/plans/2026-04-21-cli-foundation.md`
- `src-tauri/Cargo.toml`
- `src-tauri/src/core/`
- `src-tauri/src/commands/`
- `src/`
- `docs/modules/`

## Shared validation baseline

- Lint: `npm run lint` (source: `CLI override`)
- Typecheck: `cargo check --manifest-path src-tauri/Cargo.toml` (source: `CLI override`)
- Full test: `npm run verify` (source: `CLI override`)
- Build: `npm run build` (source: `CLI override`)
- Vulture: not inferred

## Guardrails

- Only one lane is active at a time
- The controller advances to the next lane only after the current lane roadmap has no remaining `[NEXT]` or `[QUEUED]` items
- Do not extend the queue automatically beyond the preset checkpoint
- Prefer the approved implementation plan task order over freestyle decomposition

# Autopilot Lane Map

> **Preset**: `Bugfix / Backlog`
> **Scheduling**: Sequential lane controller
> **Note**: The active lane comes from `automation/autopilot-config.json`; this file is a static index.

## Lane directories

- `b1-backlog-slice`
  - roadmap: `docs/status/lanes/b1-backlog-slice/autopilot-round-roadmap.md`
  - baseline: `docs/status/lanes/b1-backlog-slice/autopilot-phase-0.md`
- `b2-backlog-slice`
  - roadmap: `docs/status/lanes/b2-backlog-slice/autopilot-round-roadmap.md`
  - baseline: `docs/status/lanes/b2-backlog-slice/autopilot-phase-0.md`
- `b3-checkpoint`
  - roadmap: `docs/status/lanes/b3-checkpoint/autopilot-round-roadmap.md`
  - baseline: `docs/status/lanes/b3-checkpoint/autopilot-phase-0.md`

## Suggested entrypoints

- `AGENTS.md`
- `docs/`
- `src/`
- `scripts/check-architecture.mjs`
- `src/components/scenes/SceneCard.tsx`
- `src/context/AppContext.tsx`

## Validation baseline

- Lint: `npm run lint` (source: `CLI override`)
- Typecheck: `cargo check --manifest-path src-tauri/Cargo.toml` (source: `CLI override`)
- Full test: `npm run verify` (source: `CLI override`)
- Build: `npm run build` (source: `CLI override`)
- Vulture: not inferred

## Boundaries

- No broad polish or unrelated cleanup
- No queue expansion beyond the preset checkpoint unless a human edits the lane roadmap
- Keep `automation/runtime/` ignored and local-only

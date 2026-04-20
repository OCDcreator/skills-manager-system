# Module Documentation

> Source modules must have matching docs under `docs/modules/`. Guard scripts fail when this mapping drifts.

## Coverage Rules

Mappings are defined in `module-docs.config.json`:

- Frontend source: `src/**/{name}.ts(x)` -> `docs/modules/frontend/**/{name}.md`
- Tauri backend source: `src-tauri/src/**/{name}.rs` -> `docs/modules/tauri/**/{name}.md`
- `index.ts`, `mod.rs`, and other aggregation modules need docs because they define public module boundaries.
- UI components, hooks/context, command wrappers, core Rust logic, types, adapters, and entrypoints all need docs when they are tracked source.

## Non-Source Docs

These docs support the documentation system and are intentionally ignored by coverage:

- `docs/modules/README.md`
- `docs/modules/_TEMPLATE.md`
- `docs/modules/_WORKFLOW.md`
- `docs/modules/frontend/README.md`
- `docs/modules/tauri/README.md`

Keep any additional exceptions encoded in `module-docs.config.json`.

## Required Checks

Run both checks before merge:

```bash
node scripts/check-module-doc-coverage.mjs
node scripts/check-module-doc-diff.mjs --range origin/main...HEAD
```

The diff check combines the specified branch range with the current working tree, so local uncommitted source/doc drift is caught before commit.

`npm run verify` runs the module-doc checks before the existing architecture, build, and Rust validation gates.

## Writing Conventions

Each mapped document should follow `_TEMPLATE.md`, adapted to the module type:

- View/component modules: responsibilities, props/state, data flow, and i18n keys.
- Context/service modules: public surface, async flows, state transitions, and consumers.
- Rust commands: Tauri command boundary, input/output, error mapping, and core calls.
- Rust core modules: business rules, filesystem or parsing behavior, tests, and invariants.
- Entry/barrel modules: aggregation surface and startup responsibility.

Do not invent behavior to fill the template. Use “None” or “Not applicable” when a section does not fit.

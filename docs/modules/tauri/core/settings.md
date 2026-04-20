# Settings Store

> **Source**: `src-tauri/src/core/settings.rs`
> **Status**: [REVIEW]

## Overview

Owns persistence for app settings stored in a JSON file under the Tauri app config directory.

## Import Relationships

```text
Upstream: src-tauri/src/commands/settings.rs, src-tauri/src/commands/skills.rs
Downstream: std::fs, serde, anyhow
```

## Public Surface

| Export | Purpose |
|---|---|
| `AppSettings` | Serializable settings payload with optional `repo_path`. |
| `SettingsStore` | File-backed settings loader and saver. |
| `SettingsStore::new` | Creates a store for a base config directory. |
| `SettingsStore::load` | Reads settings or returns defaults when missing. |
| `SettingsStore::save_repo_path` | Writes the selected repository path to `settings.json`. |

## Core Logic

Loading returns default settings when the file is absent, otherwise reads and parses JSON. Saving creates the base directory, converts the optional path to a string, serializes pretty JSON, and writes `settings.json`.

## Data Flow

Tauri commands pass the app config directory into `SettingsStore`; the store reads/writes `AppSettings` and returns typed settings.

## Interactions

The JSON field naming uses camelCase for frontend compatibility. Tests use `tempfile` to verify missing-file defaults and round-tripping.

## Configuration

The settings filename is fixed as `settings.json`.

## Change Notes

If settings grow beyond repository path, update serialization tests and ensure commands remain thin wrappers.

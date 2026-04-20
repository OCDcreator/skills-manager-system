# Skills Browser Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the first working `skills-manager-system` desktop slice with a real Tauri app shell, persisted `my-skills` repository path settings, real repository scanning, searchable `custom`/`external` skill browsing, and raw `SKILL.md` detail preview.

**Architecture:** Use a small React + Tauri shell with local two-view navigation and a single `AppContext` that owns repo-path state, scan state, warnings, and selected skill detail. Keep Rust commands thin and place settings persistence plus repository scanning in `src-tauri/src/core`, with Rust unit tests covering settings, metadata parsing, scan behavior, document reading, and stable ID generation.

**Tech Stack:** Tauri 2, Rust 2021, React 19, TypeScript 5, Vite 7, Tailwind CSS 3, i18next, react-markdown, serde_yaml, walkdir

---

## File Map

- `package.json` — npm scripts, frontend dependencies, Tauri CLI dependency, and the repo-wide `verify` gate.
- `eslint.config.js` — ESLint for TypeScript React files.
- `postcss.config.js` / `tailwind.config.js` / `vite.config.ts` / `tsconfig*.json` / `index.html` — frontend build system.
- `src/main.tsx` / `src/App.tsx` / `src/styles.css` — frontend entry, root composition, and Tailwind base styles.
- `src/components/AppShell.tsx` — top-level chrome with Skills / Settings navigation and status banner area.
- `src/components/RepoPathForm.tsx` — repository-path edit and browse form.
- `src/components/skills/SkillFilters.tsx` — search box and source filter controls.
- `src/components/skills/SkillList.tsx` — grouped skill list with `custom` and `external` sections.
- `src/components/skills/SkillDetailPanel.tsx` — selected skill metadata and Markdown preview.
- `src/context/AppContext.tsx` — repo path, scan result, selected skill, loading states, refresh actions.
- `src/lib/tauri.ts` — all frontend `invoke` wrappers and shared TypeScript DTOs.
- `src/lib/skills/filters.ts` — source summaries, search filtering, list truncation, and grouping helpers.
- `src/views/SettingsView.tsx` — settings page composition.
- `src/views/SkillsView.tsx` — skills browser page composition.
- `src/i18n/index.ts` / `src/i18n/en.json` / `src/i18n/zh.json` — translation bootstrap and strings for this phase.
- `src-tauri/Cargo.toml` / `src-tauri/build.rs` / `src-tauri/tauri.conf.json` / `src-tauri/tauri.dev.conf.json` / `src-tauri/capabilities/default.json` / `src-tauri/icons/app-icon.svg` — Tauri scaffold and build metadata.
- `src-tauri/src/main.rs` / `src-tauri/src/lib.rs` — Tauri entrypoint and command registration.
- `src-tauri/src/commands/mod.rs` / `src-tauri/src/commands/settings.rs` / `src-tauri/src/commands/skills.rs` — thin command layer.
- `src-tauri/src/core/mod.rs` / `src-tauri/src/core/settings.rs` — persisted app settings.
- `src-tauri/src/core/skills/mod.rs` / `src-tauri/src/core/skills/metadata.rs` / `src-tauri/src/core/skills/scan.rs` / `src-tauri/src/core/skills/documents.rs` — repository scanning and document reading.

### Task 1: Scaffold The Runnable Frontend And Tauri Foundation

**Files:**
- Modify: `package.json`
- Create: `eslint.config.js`
- Create: `postcss.config.js`
- Create: `tailwind.config.js`
- Create: `vite.config.ts`
- Create: `tsconfig.json`
- Create: `tsconfig.app.json`
- Create: `tsconfig.node.json`
- Create: `index.html`
- Create: `src/main.tsx`
- Create: `src/App.tsx`
- Create: `src/styles.css`
- Create: `src/views/SkillsView.tsx`
- Create: `src/views/SettingsView.tsx`
- Create: `src-tauri/Cargo.toml`
- Create: `src-tauri/build.rs`
- Create: `src-tauri/tauri.conf.json`
- Create: `src-tauri/tauri.dev.conf.json`
- Create: `src-tauri/capabilities/default.json`
- Create: `src-tauri/icons/app-icon.svg`
- Create: `src-tauri/src/main.rs`
- Create: `src-tauri/src/lib.rs`

- [ ] **Step 1: Replace the root `package.json` with real app scripts and dependencies**

```json
{
  "name": "skills-manager-system",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "check:architecture": "node scripts/check-architecture.mjs",
    "test:architecture": "node --test scripts/check-architecture.test.mjs",
    "dev": "vite",
    "build": "tsc -b && vite build",
    "lint": "eslint .",
    "tauri": "tauri",
    "tauri:dev": "tauri dev --config src-tauri/tauri.dev.conf.json",
    "tauri:build": "tauri build",
    "verify": "npm run check:architecture && npm run test:architecture && npm run build && cargo check --manifest-path src-tauri/Cargo.toml && cargo test --manifest-path src-tauri/Cargo.toml"
  },
  "dependencies": {
    "@tauri-apps/api": "^2.10.1",
    "@tauri-apps/plugin-dialog": "^2.6.0",
    "clsx": "^2.1.1",
    "i18next": "^25.8.13",
    "lucide-react": "^0.575.0",
    "react": "^19.2.0",
    "react-dom": "^19.2.0",
    "react-i18next": "^16.5.4",
    "react-markdown": "^10.1.0",
    "remark-gfm": "^4.0.1"
  },
  "devDependencies": {
    "@eslint/js": "^9.39.1",
    "@tauri-apps/cli": "^2.10.0",
    "@types/node": "^24.10.1",
    "@types/react": "^19.2.7",
    "@types/react-dom": "^19.2.3",
    "@vitejs/plugin-react": "^5.1.1",
    "autoprefixer": "^10.4.27",
    "eslint": "^9.39.1",
    "eslint-plugin-react-hooks": "^7.0.1",
    "eslint-plugin-react-refresh": "^0.4.24",
    "globals": "^16.5.0",
    "postcss": "^8.5.6",
    "tailwindcss": "^3.4.19",
    "typescript": "~5.9.3",
    "typescript-eslint": "^8.48.0",
    "vite": "^7.3.1"
  }
}
```

- [ ] **Step 2: Add the frontend toolchain and placeholder views**

```js
// eslint.config.js
import js from "@eslint/js";
import globals from "globals";
import reactHooks from "eslint-plugin-react-hooks";
import reactRefresh from "eslint-plugin-react-refresh";
import tseslint from "typescript-eslint";

export default tseslint.config(
  { ignores: ["dist", "src-tauri/target"] },
  {
    extends: [js.configs.recommended, ...tseslint.configs.recommended],
    files: ["**/*.{ts,tsx}"],
    languageOptions: {
      ecmaVersion: 2020,
      globals: globals.browser,
    },
    plugins: {
      "react-hooks": reactHooks,
      "react-refresh": reactRefresh,
    },
    rules: {
      ...reactHooks.configs.recommended.rules,
      "react-refresh/only-export-components": ["warn", { allowConstantExport: true }],
    },
  }
);
```

```js
// postcss.config.js
export default {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
};
```

```js
// tailwind.config.js
/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        surface: "#0f172a",
        panel: "#111827",
        border: "#334155",
        accent: "#38bdf8",
      },
    },
  },
  plugins: [],
};
```

```ts
// vite.config.ts
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
});
```

```json
// tsconfig.json
{
  "files": [],
  "references": [
    { "path": "./tsconfig.app.json" },
    { "path": "./tsconfig.node.json" }
  ]
}
```

```json
// tsconfig.app.json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "allowJs": false,
    "skipLibCheck": true,
    "esModuleInterop": true,
    "allowSyntheticDefaultImports": true,
    "strict": true,
    "forceConsistentCasingInFileNames": true,
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx"
  },
  "include": ["src"]
}
```

```json
// tsconfig.node.json
{
  "compilerOptions": {
    "composite": true,
    "skipLibCheck": true,
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "allowSyntheticDefaultImports": true
  },
  "include": ["vite.config.ts"]
}
```

```html
<!-- index.html -->
<!doctype html>
<html lang="zh-CN">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Skills Manager System</title>
  </head>
  <body class="bg-slate-950">
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>
```

```tsx
// src/views/SkillsView.tsx
export function SkillsView() {
  return (
    <section className="rounded-2xl border border-slate-800 bg-slate-900 p-6 text-slate-100">
      <h2 className="text-xl font-semibold">Skills</h2>
      <p className="mt-2 text-sm text-slate-400">
        Skills browser placeholder. The real scan integration arrives in later tasks.
      </p>
    </section>
  );
}
```

```tsx
// src/views/SettingsView.tsx
export function SettingsView() {
  return (
    <section className="rounded-2xl border border-slate-800 bg-slate-900 p-6 text-slate-100">
      <h2 className="text-xl font-semibold">Settings</h2>
      <p className="mt-2 text-sm text-slate-400">
        Repository settings placeholder. The persisted path form arrives in later tasks.
      </p>
    </section>
  );
}
```

- [ ] **Step 3: Add a minimal React entrypoint so the frontend builds before any Tauri integration**

```tsx
// src/App.tsx
import { useState } from "react";
import { SkillsView } from "./views/SkillsView";
import { SettingsView } from "./views/SettingsView";

type AppView = "skills" | "settings";

export default function App() {
  const [view, setView] = useState<AppView>("skills");

  return (
    <div className="min-h-screen bg-slate-950 text-slate-100">
      <header className="border-b border-slate-800 bg-slate-900/80">
        <div className="mx-auto flex max-w-7xl items-center justify-between px-6 py-4">
          <div>
            <h1 className="text-2xl font-semibold">Skills Manager System</h1>
            <p className="text-sm text-slate-400">Phase 1 foundation</p>
          </div>
          <nav className="flex gap-2">
            <button
              className={`rounded-lg px-4 py-2 text-sm ${view === "skills" ? "bg-sky-500 text-slate-950" : "bg-slate-800 text-slate-200"}`}
              onClick={() => setView("skills")}
            >
              Skills
            </button>
            <button
              className={`rounded-lg px-4 py-2 text-sm ${view === "settings" ? "bg-sky-500 text-slate-950" : "bg-slate-800 text-slate-200"}`}
              onClick={() => setView("settings")}
            >
              Settings
            </button>
          </nav>
        </div>
      </header>

      <main className="mx-auto max-w-7xl px-6 py-8">
        {view === "skills" ? <SkillsView /> : <SettingsView />}
      </main>
    </div>
  );
}
```

```tsx
// src/main.tsx
import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
```

```css
/* src/styles.css */
@tailwind base;
@tailwind components;
@tailwind utilities;

:root {
  color: #e2e8f0;
  background: #020617;
  font-family: "Segoe UI", "Inter", system-ui, sans-serif;
}

body {
  margin: 0;
  min-width: 320px;
  min-height: 100vh;
}

button,
input {
  font: inherit;
}
```

- [ ] **Step 4: Add the minimal Tauri scaffold, config, and icon source**

```toml
# src-tauri/Cargo.toml
[package]
name = "skills-manager-system"
version = "0.1.0"
description = "Skills Manager System"
authors = ["lt"]
edition = "2021"
rust-version = "1.77.2"

[lib]
name = "app_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build = { version = "2.5.4", features = [] }

[dependencies]
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tauri = { version = "2.10.0", features = [] }
tauri-plugin-dialog = "2"

[dev-dependencies]
tempfile = "3.26.0"
```

```rust
// src-tauri/build.rs
fn main() {
    tauri_build::build()
}
```

```json
// src-tauri/tauri.conf.json
{
  "$schema": "../node_modules/@tauri-apps/cli/config.schema.json",
  "productName": "Skills Manager System",
  "version": "0.1.0",
  "identifier": "com.ocdcreator.skills-manager-system",
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:1420",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "Skills Manager System",
        "width": 1440,
        "height": 920,
        "resizable": true
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

```json
// src-tauri/tauri.dev.conf.json
{
  "$schema": "../node_modules/@tauri-apps/cli/config.schema.json",
  "identifier": "com.ocdcreator.skills-manager-system.dev"
}
```

```json
// src-tauri/capabilities/default.json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Default desktop capability",
  "windows": ["main"],
  "permissions": ["core:default", "dialog:default"]
}
```

```xml
<!-- src-tauri/icons/app-icon.svg -->
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
  <rect width="512" height="512" rx="112" fill="#0f172a" />
  <path d="M136 144h240v56H136zM136 228h176v56H136zM136 312h240v56H136z" fill="#38bdf8" />
  <circle cx="368" cy="256" r="44" fill="#e2e8f0" />
</svg>
```

```rust
// src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    app_lib::run();
}
```

```rust
// src-tauri/src/lib.rs
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 5: Install dependencies and generate the Tauri icon set**

Run: `npm install`  
Expected: npm exits `0`, creates `package-lock.json`, and installs frontend plus Tauri CLI dependencies.

Run: `npm exec tauri icon src-tauri/icons/app-icon.svg`  
Expected: generated icon files appear under `src-tauri/icons/`, including `32x32.png`, `128x128.png`, `icon.ico`, and `icon.icns`.

- [ ] **Step 6: Verify the scaffold before moving to business logic**

Run: `npm run build`  
Expected: TypeScript exits `0`, Vite prints `built in`, and the `dist/` directory is produced.

Run: `cargo check --manifest-path src-tauri/Cargo.toml`  
Expected: Rust exits `0` and reports a finished `dev` profile build for `skills-manager-system`.

- [ ] **Step 7: Commit the foundation scaffold**

```bash
git add package.json package-lock.json eslint.config.js postcss.config.js tailwind.config.js vite.config.ts tsconfig.json tsconfig.app.json tsconfig.node.json index.html src src-tauri
git commit -m "chore: scaffold tauri browser foundation"
```

### Task 2: Persist The Repository Path In Rust And Expose Thin Settings Commands

**Files:**
- Create: `src-tauri/src/commands/mod.rs`
- Create: `src-tauri/src/commands/settings.rs`
- Create: `src-tauri/src/core/mod.rs`
- Create: `src-tauri/src/core/settings.rs`
- Modify: `src-tauri/src/lib.rs`
- Test: `src-tauri/src/core/settings.rs`

- [ ] **Step 1: Write failing Rust tests for the settings store**

```rust
// append to src-tauri/src/core/settings.rs first
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn load_returns_none_when_file_does_not_exist() {
        let dir = tempdir().unwrap();
        let store = SettingsStore::new(dir.path().to_path_buf());

        assert_eq!(store.load().unwrap().repo_path, None);
    }

    #[test]
    fn save_repo_path_round_trips() {
        let dir = tempdir().unwrap();
        let store = SettingsStore::new(dir.path().to_path_buf());

        store
            .save_repo_path(Some(Path::new("C:/Users/test/Desktop/Write/custom-project/my-skills")))
            .unwrap();

        assert_eq!(
            store.load().unwrap().repo_path.as_deref(),
            Some("C:/Users/test/Desktop/Write/custom-project/my-skills")
        );
    }
}
```

- [ ] **Step 2: Run the settings tests and confirm they fail before implementation**

Run: `cargo test --manifest-path src-tauri/Cargo.toml settings -- --nocapture`  
Expected: FAIL with compile errors for missing `SettingsStore` and `load` / `save_repo_path` implementations.

- [ ] **Step 3: Implement the JSON-backed settings store**

```rust
// replace src-tauri/src/core/settings.rs
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub repo_path: Option<String>,
}

pub struct SettingsStore {
    base_dir: PathBuf,
}

impl SettingsStore {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    pub fn load(&self) -> Result<AppSettings> {
        let path = self.settings_path();
        if !path.exists() {
            return Ok(AppSettings::default());
        }

        let raw = fs::read_to_string(&path).with_context(|| format!("Failed to read {:?}", path))?;
        let settings = serde_json::from_str::<AppSettings>(&raw)
            .with_context(|| format!("Failed to parse {:?}", path))?;
        Ok(settings)
    }

    pub fn save_repo_path(&self, repo_path: Option<&Path>) -> Result<AppSettings> {
        fs::create_dir_all(&self.base_dir)
            .with_context(|| format!("Failed to create {:?}", self.base_dir))?;

        let settings = AppSettings {
            repo_path: repo_path.map(|value| value.to_string_lossy().to_string()),
        };

        let json = serde_json::to_string_pretty(&settings)?;
        fs::write(self.settings_path(), json).context("Failed to write settings.json")?;
        Ok(settings)
    }

    fn settings_path(&self) -> PathBuf {
        self.base_dir.join("settings.json")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn load_returns_none_when_file_does_not_exist() {
        let dir = tempdir().unwrap();
        let store = SettingsStore::new(dir.path().to_path_buf());

        assert_eq!(store.load().unwrap().repo_path, None);
    }

    #[test]
    fn save_repo_path_round_trips() {
        let dir = tempdir().unwrap();
        let store = SettingsStore::new(dir.path().to_path_buf());

        store
            .save_repo_path(Some(Path::new("C:/Users/test/Desktop/Write/custom-project/my-skills")))
            .unwrap();

        assert_eq!(
            store.load().unwrap().repo_path.as_deref(),
            Some("C:/Users/test/Desktop/Write/custom-project/my-skills")
        );
    }
}
```

- [ ] **Step 4: Add thin settings commands and register them in the Tauri app**

```rust
// src-tauri/src/core/mod.rs
pub mod settings;
```

```rust
// src-tauri/src/commands/settings.rs
use std::path::Path;

use crate::core::settings::SettingsStore;

#[tauri::command]
pub fn get_repo_path(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let config_dir = app.path().app_config_dir().map_err(|error| error.to_string())?;
    SettingsStore::new(config_dir)
        .load()
        .map(|settings| settings.repo_path)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn set_repo_path(app: tauri::AppHandle, path: String) -> Result<Option<String>, String> {
    let trimmed = path.trim();
    let normalized = if trimmed.is_empty() {
        None
    } else {
        Some(Path::new(trimmed))
    };

    let config_dir = app.path().app_config_dir().map_err(|error| error.to_string())?;
    SettingsStore::new(config_dir)
        .save_repo_path(normalized)
        .map(|settings| settings.repo_path)
        .map_err(|error| error.to_string())
}
```

```rust
// src-tauri/src/commands/mod.rs
pub mod settings;
```

```rust
// replace src-tauri/src/lib.rs
mod commands;
mod core;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::settings::get_repo_path,
            commands::settings::set_repo_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 5: Run the settings tests again and confirm the command layer still compiles**

Run: `cargo test --manifest-path src-tauri/Cargo.toml settings -- --nocapture`  
Expected: PASS with both `load_returns_none_when_file_does_not_exist` and `save_repo_path_round_trips`.

Run: `cargo check --manifest-path src-tauri/Cargo.toml`  
Expected: PASS with the new `get_repo_path` and `set_repo_path` commands registered.

- [ ] **Step 6: Commit the persisted settings layer**

```bash
git add src-tauri/src/core/mod.rs src-tauri/src/core/settings.rs src-tauri/src/commands/mod.rs src-tauri/src/commands/settings.rs src-tauri/src/lib.rs
git commit -m "feat: persist repository path settings"
```

### Task 3: Implement Metadata Parsing, Repository Scanning, Document Reading, And Skills Commands

**Files:**
- Create: `src-tauri/src/core/skills/mod.rs`
- Create: `src-tauri/src/core/skills/metadata.rs`
- Create: `src-tauri/src/core/skills/scan.rs`
- Create: `src-tauri/src/core/skills/documents.rs`
- Create: `src-tauri/src/commands/skills.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/core/mod.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/Cargo.toml`
- Test: `src-tauri/src/core/skills/metadata.rs`
- Test: `src-tauri/src/core/skills/scan.rs`
- Test: `src-tauri/src/core/skills/documents.rs`

- [ ] **Step 1: Add failing Rust tests for metadata, scan rules, stable IDs, and document reading**

```toml
# add these dependencies to src-tauri/Cargo.toml
[dependencies]
serde_yaml = "0.9"
walkdir = "2.5"
```

```rust
// append to src-tauri/src/core/skills/metadata.rs first
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn parse_reads_name_and_description() {
        let dir = tempdir().unwrap();
        fs::write(
            dir.path().join("SKILL.md"),
            "---\nname: frontend-design\ndescription: polished UI systems\n---\n# body",
        )
        .unwrap();

        let metadata = parse_skill_metadata(&dir.path().join("SKILL.md")).unwrap();
        assert_eq!(metadata.name.as_deref(), Some("frontend-design"));
        assert_eq!(metadata.description.as_deref(), Some("polished UI systems"));
    }

    #[test]
    fn parse_falls_back_when_frontmatter_is_missing() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("SKILL.md"), "# plain markdown").unwrap();

        let metadata = parse_skill_metadata(&dir.path().join("SKILL.md")).unwrap();
        assert_eq!(metadata.name, None);
        assert_eq!(metadata.description, None);
    }
}
```

```rust
// append to src-tauri/src/core/skills/scan.rs first
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn scan_custom_only_reads_first_level_directories() {
        let repo = tempdir().unwrap();
        fs::create_dir_all(repo.path().join("custom/searxng")).unwrap();
        fs::create_dir_all(repo.path().join("custom/group/sub-skill")).unwrap();
        fs::write(repo.path().join("custom/searxng/SKILL.md"), "---\nname: searxng\n---").unwrap();
        fs::write(repo.path().join("custom/group/sub-skill/SKILL.md"), "---\nname: nested\n---").unwrap();

        let response = scan_repo_skills(repo.path()).unwrap();
        let ids: Vec<_> = response.skills.iter().map(|skill| skill.id.as_str()).collect();

        assert!(ids.contains(&"custom:searxng"));
        assert!(!ids.contains(&"custom:group/sub-skill"));
    }

    #[test]
    fn scan_external_recurses_and_ignores_reference_only_sources() {
        let repo = tempdir().unwrap();
        fs::create_dir_all(repo.path().join("external/html-ppt-skill")).unwrap();
        fs::create_dir_all(repo.path().join("external/awesome-design-md/apple")).unwrap();
        fs::write(repo.path().join("external/html-ppt-skill/SKILL.md"), "---\nname: html-ppt\n---").unwrap();
        fs::write(repo.path().join("external/awesome-design-md/apple/README.md"), "# reference").unwrap();

        let response = scan_repo_skills(repo.path()).unwrap();
        let ids: Vec<_> = response.skills.iter().map(|skill| skill.id.as_str()).collect();

        assert!(ids.contains(&"external:html-ppt-skill"));
        assert!(!ids.iter().any(|id| id.contains("awesome-design-md")));
    }

    #[test]
    fn scan_ignores_noise_directories() {
        let repo = tempdir().unwrap();
        fs::create_dir_all(repo.path().join("external/node_modules/pkg")).unwrap();
        fs::create_dir_all(repo.path().join("external/anthropics-skills/frontend-design")).unwrap();
        fs::write(repo.path().join("external/node_modules/pkg/SKILL.md"), "---\nname: nope\n---").unwrap();
        fs::write(
            repo.path().join("external/anthropics-skills/frontend-design/SKILL.md"),
            "---\nname: frontend-design\n---",
        )
        .unwrap();

        let response = scan_repo_skills(repo.path()).unwrap();
        let ids: Vec<_> = response.skills.iter().map(|skill| skill.id.as_str()).collect();

        assert!(!ids.iter().any(|id| id.contains("node_modules")));
        assert!(ids.contains(&"external:anthropics-skills/frontend-design"));
    }
}
```

```rust
// append to src-tauri/src/core/skills/documents.rs first
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn read_skill_document_returns_content_for_valid_relative_path() {
        let repo = tempdir().unwrap();
        fs::create_dir_all(repo.path().join("custom/searxng")).unwrap();
        fs::write(
            repo.path().join("custom/searxng/SKILL.md"),
            "---\nname: searxng\ndescription: search helper\n---\n# Content",
        )
        .unwrap();

        let document = read_skill_document(repo.path(), "custom/searxng").unwrap();

        assert_eq!(document.id, "custom:searxng");
        assert!(document.content.contains("# Content"));
    }

    #[test]
    fn read_skill_document_rejects_parent_directory_traversal() {
        let repo = tempdir().unwrap();
        let error = read_skill_document(repo.path(), "../outside").unwrap_err();
        assert!(error.to_string().contains("Invalid relative path"));
    }
}
```

- [ ] **Step 2: Run the new Rust tests and confirm they fail before implementation**

Run: `cargo test --manifest-path src-tauri/Cargo.toml skills -- --nocapture`  
Expected: FAIL with missing `parse_skill_metadata`, `scan_repo_skills`, and `read_skill_document` implementations.

- [ ] **Step 3: Implement metadata parsing with safe fallbacks**

```rust
// src-tauri/src/core/skills/metadata.rs
use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SkillMetadata {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Frontmatter {
    name: Option<String>,
    description: Option<String>,
}

pub fn parse_skill_metadata(skill_md_path: &Path) -> Result<SkillMetadata> {
    let content = fs::read_to_string(skill_md_path)?;
    let trimmed = content.trim_start();

    if !trimmed.starts_with("---") {
        return Ok(SkillMetadata::default());
    }

    let remainder = &trimmed[3..];
    let Some(end_index) = remainder.find("---") else {
        return Ok(SkillMetadata::default());
    };

    let yaml = &remainder[..end_index];
    let parsed = serde_yaml::from_str::<Frontmatter>(yaml).unwrap_or(Frontmatter {
        name: None,
        description: None,
    });

    Ok(SkillMetadata {
        name: parsed.name,
        description: parsed.description,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn parse_reads_name_and_description() {
        let dir = tempdir().unwrap();
        fs::write(
            dir.path().join("SKILL.md"),
            "---\nname: frontend-design\ndescription: polished UI systems\n---\n# body",
        )
        .unwrap();

        let metadata = parse_skill_metadata(&dir.path().join("SKILL.md")).unwrap();
        assert_eq!(metadata.name.as_deref(), Some("frontend-design"));
        assert_eq!(metadata.description.as_deref(), Some("polished UI systems"));
    }

    #[test]
    fn parse_falls_back_when_frontmatter_is_missing() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("SKILL.md"), "# plain markdown").unwrap();

        let metadata = parse_skill_metadata(&dir.path().join("SKILL.md")).unwrap();
        assert_eq!(metadata.name, None);
        assert_eq!(metadata.description, None);
    }
}
```

- [ ] **Step 4: Implement the scan core and document reader with stable IDs and ignore rules**

```rust
// src-tauri/src/core/skills/mod.rs
pub mod documents;
pub mod metadata;
pub mod scan;
```

```rust
// src-tauri/src/core/skills/scan.rs
use anyhow::{anyhow, Result};
use serde::Serialize;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

use super::metadata::parse_skill_metadata;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SkillSummary {
    pub id: String,
    pub name: String,
    pub description: String,
    pub source_type: String,
    pub relative_path: String,
    pub directory_path: String,
    pub skill_document_path: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ScanSkillsResponse {
    pub skills: Vec<SkillSummary>,
    pub warnings: Vec<String>,
}

const IGNORED_DIRS: &[&str] = &[".git", "node_modules", "dist", "target", ".tmp-skills"];

pub fn scan_repo_skills(repo_root: &Path) -> Result<ScanSkillsResponse> {
    if !repo_root.exists() {
        return Err(anyhow!("Configured repository path does not exist"));
    }

    let mut warnings = Vec::new();
    let mut skills = Vec::new();

    let custom_root = repo_root.join("custom");
    if custom_root.exists() {
        for entry in std::fs::read_dir(&custom_root)? {
            let entry = entry?;
            let skill_dir = entry.path();
            if !skill_dir.is_dir() {
                continue;
            }
            let skill_md = skill_dir.join("SKILL.md");
            if !skill_md.exists() {
                continue;
            }
            skills.push(build_skill_summary(repo_root, &skill_dir, "custom")?);
        }
    } else {
        warnings.push("Missing custom/ directory; continuing with remaining sources.".to_string());
    }

    let external_root = repo_root.join("external");
    if external_root.exists() {
        for entry in WalkDir::new(&external_root)
            .into_iter()
            .filter_entry(should_walk)
            .filter_map(Result::ok)
        {
            if entry.file_type().is_file() && entry.file_name() == "SKILL.md" {
                let skill_dir = entry.path().parent().unwrap();
                skills.push(build_skill_summary(repo_root, skill_dir, "external")?);
            }
        }
    } else {
        warnings.push("Missing external/ directory; continuing with remaining sources.".to_string());
    }

    skills.sort_by(|left, right| left.id.cmp(&right.id));

    Ok(ScanSkillsResponse { skills, warnings })
}

fn should_walk(entry: &DirEntry) -> bool {
    !IGNORED_DIRS.iter().any(|ignored| entry.file_name() == *ignored)
}

fn build_skill_summary(repo_root: &Path, skill_dir: &Path, source_type: &str) -> Result<SkillSummary> {
    let relative_path = normalize_relative_path(repo_root, skill_dir)?;
    let skill_md_path = skill_dir.join("SKILL.md");
    let metadata = parse_skill_metadata(&skill_md_path)?;
    let default_name = skill_dir
        .file_name()
        .map(|value| value.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown-skill".to_string());

    Ok(SkillSummary {
        id: build_skill_id(source_type, &relative_path),
        name: metadata.name.unwrap_or(default_name),
        description: metadata.description.unwrap_or_default(),
        source_type: source_type.to_string(),
        relative_path,
        directory_path: skill_dir.to_string_lossy().to_string(),
        skill_document_path: skill_md_path.to_string_lossy().to_string(),
    })
}

pub fn build_skill_id(source_type: &str, relative_path: &str) -> String {
    let source_relative = relative_path
        .strip_prefix("custom/")
        .or_else(|| relative_path.strip_prefix("external/"))
        .unwrap_or(relative_path);
    format!("{source_type}:{source_relative}")
}

fn normalize_relative_path(repo_root: &Path, skill_dir: &Path) -> Result<String> {
    let relative = skill_dir.strip_prefix(repo_root)?;
    Ok(relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy().to_string())
        .collect::<Vec<_>>()
        .join("/"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn scan_custom_only_reads_first_level_directories() {
        let repo = tempdir().unwrap();
        fs::create_dir_all(repo.path().join("custom/searxng")).unwrap();
        fs::create_dir_all(repo.path().join("custom/group/sub-skill")).unwrap();
        fs::write(repo.path().join("custom/searxng/SKILL.md"), "---\nname: searxng\n---").unwrap();
        fs::write(repo.path().join("custom/group/sub-skill/SKILL.md"), "---\nname: nested\n---").unwrap();

        let response = scan_repo_skills(repo.path()).unwrap();
        let ids: Vec<_> = response.skills.iter().map(|skill| skill.id.as_str()).collect();

        assert!(ids.contains(&"custom:searxng"));
        assert!(!ids.contains(&"custom:group/sub-skill"));
    }

    #[test]
    fn scan_external_recurses_and_ignores_reference_only_sources() {
        let repo = tempdir().unwrap();
        fs::create_dir_all(repo.path().join("external/html-ppt-skill")).unwrap();
        fs::create_dir_all(repo.path().join("external/awesome-design-md/apple")).unwrap();
        fs::write(repo.path().join("external/html-ppt-skill/SKILL.md"), "---\nname: html-ppt\n---").unwrap();
        fs::write(repo.path().join("external/awesome-design-md/apple/README.md"), "# reference").unwrap();

        let response = scan_repo_skills(repo.path()).unwrap();
        let ids: Vec<_> = response.skills.iter().map(|skill| skill.id.as_str()).collect();

        assert!(ids.contains(&"external:html-ppt-skill"));
        assert!(!ids.iter().any(|id| id.contains("awesome-design-md")));
    }

    #[test]
    fn scan_ignores_noise_directories() {
        let repo = tempdir().unwrap();
        fs::create_dir_all(repo.path().join("external/node_modules/pkg")).unwrap();
        fs::create_dir_all(repo.path().join("external/anthropics-skills/frontend-design")).unwrap();
        fs::write(repo.path().join("external/node_modules/pkg/SKILL.md"), "---\nname: nope\n---").unwrap();
        fs::write(
            repo.path().join("external/anthropics-skills/frontend-design/SKILL.md"),
            "---\nname: frontend-design\n---",
        )
        .unwrap();

        let response = scan_repo_skills(repo.path()).unwrap();
        let ids: Vec<_> = response.skills.iter().map(|skill| skill.id.as_str()).collect();

        assert!(!ids.iter().any(|id| id.contains("node_modules")));
        assert!(ids.contains(&"external:anthropics-skills/frontend-design"));
    }
}
```

```rust
// src-tauri/src/core/skills/documents.rs
use anyhow::{anyhow, Result};
use serde::Serialize;
use std::fs;
use std::path::{Component, Path};

use super::metadata::parse_skill_metadata;
use super::scan::build_skill_id;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SkillDocument {
    pub id: String,
    pub name: String,
    pub description: String,
    pub source_type: String,
    pub relative_path: String,
    pub content: String,
}

pub fn read_skill_document(repo_root: &Path, relative_path: &str) -> Result<SkillDocument> {
    validate_relative_path(relative_path)?;

    let skill_dir = repo_root.join(relative_path);
    let skill_md_path = skill_dir.join("SKILL.md");
    let content = fs::read_to_string(&skill_md_path)?;
    let metadata = parse_skill_metadata(&skill_md_path)?;

    let source_type = if relative_path.starts_with("custom/") {
        "custom"
    } else if relative_path.starts_with("external/") {
        "external"
    } else {
        return Err(anyhow!("Invalid skill source path"));
    };

    let name = metadata.name.unwrap_or_else(|| {
        skill_dir
            .file_name()
            .map(|value| value.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown-skill".to_string())
    });

    Ok(SkillDocument {
        id: build_skill_id(source_type, relative_path),
        name,
        description: metadata.description.unwrap_or_default(),
        source_type: source_type.to_string(),
        relative_path: relative_path.to_string(),
        content,
    })
}

fn validate_relative_path(relative_path: &str) -> Result<()> {
    let path = Path::new(relative_path);
    for component in path.components() {
        if matches!(component, Component::ParentDir | Component::Prefix(_) | Component::RootDir) {
            return Err(anyhow!("Invalid relative path"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn read_skill_document_returns_content_for_valid_relative_path() {
        let repo = tempdir().unwrap();
        fs::create_dir_all(repo.path().join("custom/searxng")).unwrap();
        fs::write(
            repo.path().join("custom/searxng/SKILL.md"),
            "---\nname: searxng\ndescription: search helper\n---\n# Content",
        )
        .unwrap();

        let document = read_skill_document(repo.path(), "custom/searxng").unwrap();

        assert_eq!(document.id, "custom:searxng");
        assert!(document.content.contains("# Content"));
    }

    #[test]
    fn read_skill_document_rejects_parent_directory_traversal() {
        let repo = tempdir().unwrap();
        let error = read_skill_document(repo.path(), "../outside").unwrap_err();
        assert!(error.to_string().contains("Invalid relative path"));
    }
}
```

- [ ] **Step 5: Add skills commands and register them in `lib.rs`, then rerun backend verification**

```rust
// src-tauri/src/core/mod.rs
pub mod settings;
pub mod skills;
```

```rust
// src-tauri/src/commands/skills.rs
use crate::core::skills::documents::{read_skill_document, SkillDocument};
use crate::core::skills::scan::{scan_repo_skills, ScanSkillsResponse};
use crate::core::settings::SettingsStore;

#[tauri::command]
pub fn scan_skills(app: tauri::AppHandle) -> Result<ScanSkillsResponse, String> {
    let config_dir = app.path().app_config_dir().map_err(|error| error.to_string())?;
    let settings = SettingsStore::new(config_dir)
        .load()
        .map_err(|error| error.to_string())?;
    let repo_path = settings
        .repo_path
        .ok_or_else(|| "Repository path is not configured".to_string())?;

    scan_repo_skills(std::path::Path::new(&repo_path)).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_skill_document(app: tauri::AppHandle, relative_path: String) -> Result<SkillDocument, String> {
    let config_dir = app.path().app_config_dir().map_err(|error| error.to_string())?;
    let settings = SettingsStore::new(config_dir)
        .load()
        .map_err(|error| error.to_string())?;
    let repo_path = settings
        .repo_path
        .ok_or_else(|| "Repository path is not configured".to_string())?;

    read_skill_document(std::path::Path::new(&repo_path), &relative_path)
        .map_err(|error| error.to_string())
}
```

```rust
// src-tauri/src/commands/mod.rs
pub mod settings;
pub mod skills;
```

```rust
// replace src-tauri/src/lib.rs
mod commands;
mod core;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::settings::get_repo_path,
            commands::settings::set_repo_path,
            commands::skills::scan_skills,
            commands::skills::get_skill_document
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Run: `cargo test --manifest-path src-tauri/Cargo.toml skills -- --nocapture`  
Expected: PASS with metadata, scan, and document tests green.

Run: `cargo check --manifest-path src-tauri/Cargo.toml`  
Expected: PASS with `scan_skills` and `get_skill_document` commands registered.

- [ ] **Step 6: Commit the repository scan backend**

```bash
git add src-tauri/Cargo.toml src-tauri/src/core/mod.rs src-tauri/src/core/skills src-tauri/src/commands/mod.rs src-tauri/src/commands/skills.rs src-tauri/src/lib.rs
git commit -m "feat: scan repository skills"
```

### Task 4: Wire Frontend Types, App Context, I18n, And The Settings Workflow

**Files:**
- Create: `src/lib/tauri.ts`
- Create: `src/context/AppContext.tsx`
- Create: `src/i18n/index.ts`
- Create: `src/i18n/en.json`
- Create: `src/i18n/zh.json`
- Create: `src/components/AppShell.tsx`
- Create: `src/components/RepoPathForm.tsx`
- Modify: `src/App.tsx`
- Modify: `src/main.tsx`
- Modify: `src/views/SettingsView.tsx`
- Modify: `src/views/SkillsView.tsx`
- Modify: `src/styles.css`

- [ ] **Step 1: Add the frontend Tauri wrapper and translation bootstrap**

```ts
// src/lib/tauri.ts
import { invoke } from "@tauri-apps/api/core";

export interface SkillSummary {
  id: string;
  name: string;
  description: string;
  sourceType: "custom" | "external";
  relativePath: string;
  directoryPath: string;
  skillDocumentPath: string;
}

export interface ScanSkillsResponse {
  skills: SkillSummary[];
  warnings: string[];
}

export interface SkillDocument {
  id: string;
  name: string;
  description: string;
  sourceType: "custom" | "external";
  relativePath: string;
  content: string;
}

export const getRepoPath = () => invoke<string | null>("get_repo_path");
export const setRepoPath = (path: string) => invoke<string | null>("set_repo_path", { path });
export const scanSkills = () => invoke<ScanSkillsResponse>("scan_skills");
export const getSkillDocument = (relativePath: string) =>
  invoke<SkillDocument>("get_skill_document", { relativePath });
```

```ts
// src/i18n/index.ts
import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import en from "./en.json";
import zh from "./zh.json";

void i18n.use(initReactI18next).init({
  resources: {
    en: { translation: en },
    zh: { translation: zh },
  },
  lng: navigator.language.toLowerCase().startsWith("zh") ? "zh" : "en",
  fallbackLng: "en",
  interpolation: {
    escapeValue: false,
  },
});

export default i18n;
```

```json
// src/i18n/en.json
{
  "app.title": "Skills Manager System",
  "app.subtitle": "Browse and review your local skill repository",
  "nav.skills": "Skills",
  "nav.settings": "Settings",
  "settings.title": "Repository Settings",
  "settings.description": "Choose the local my-skills repository path used for scanning.",
  "settings.pathLabel": "Repository path",
  "settings.pathPlaceholder": "C:/Users/you/Desktop/Write/custom-project/my-skills",
  "settings.browse": "Browse",
  "settings.save": "Save",
  "settings.saved": "Repository path saved. Refreshing scan results...",
  "settings.error": "Failed to save the repository path.",
  "skills.placeholder.title": "Skills browser is loading",
  "skills.placeholder.body": "The searchable browser arrives after the shared app state is wired."
}
```

```json
// src/i18n/zh.json
{
  "app.title": "Skills Manager System",
  "app.subtitle": "浏览并检查本地技能仓库",
  "nav.skills": "技能浏览",
  "nav.settings": "设置",
  "settings.title": "仓库设置",
  "settings.description": "选择用于扫描的本地 my-skills 仓库路径。",
  "settings.pathLabel": "仓库路径",
  "settings.pathPlaceholder": "C:/Users/you/Desktop/Write/custom-project/my-skills",
  "settings.browse": "选择目录",
  "settings.save": "保存",
  "settings.saved": "仓库路径已保存，正在刷新扫描结果……",
  "settings.error": "保存仓库路径失败。",
  "skills.placeholder.title": "技能浏览正在接线",
  "skills.placeholder.body": "共享应用状态完成后，这里会展示可搜索的技能浏览器。"
}
```

- [ ] **Step 2: Implement the shared app state so both views use the same repo-path and scan data**

```tsx
// src/context/AppContext.tsx
import { createContext, useCallback, useContext, useEffect, useMemo, useState } from "react";
import type { PropsWithChildren } from "react";
import * as api from "../lib/tauri";
import type { ScanSkillsResponse, SkillDocument, SkillSummary } from "../lib/tauri";

export type AppView = "skills" | "settings";

interface AppContextValue {
  activeView: AppView;
  repoPath: string | null;
  scanResult: ScanSkillsResponse;
  selectedSkill: SkillSummary | null;
  selectedDocument: SkillDocument | null;
  isLoading: boolean;
  isSavingPath: boolean;
  errorMessage: string | null;
  setActiveView: (view: AppView) => void;
  refreshSkills: () => Promise<void>;
  saveRepoPath: (nextPath: string) => Promise<void>;
  selectSkill: (skill: SkillSummary | null) => Promise<void>;
}

const AppContext = createContext<AppContextValue | null>(null);

export function AppProvider({ children }: PropsWithChildren) {
  const [activeView, setActiveView] = useState<AppView>("skills");
  const [repoPath, setRepoPath] = useState<string | null>(null);
  const [scanResult, setScanResult] = useState<ScanSkillsResponse>({ skills: [], warnings: [] });
  const [selectedSkill, setSelectedSkill] = useState<SkillSummary | null>(null);
  const [selectedDocument, setSelectedDocument] = useState<SkillDocument | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isSavingPath, setIsSavingPath] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  const refreshSkills = useCallback(async () => {
    if (!repoPath) {
      setScanResult({ skills: [], warnings: [] });
      setSelectedSkill(null);
      setSelectedDocument(null);
      return;
    }

    setIsLoading(true);
    try {
      const response = await api.scanSkills();
      setScanResult(response);
      setErrorMessage(null);

      if (selectedSkill) {
        const refreshed = response.skills.find((skill) => skill.id === selectedSkill.id) ?? null;
        setSelectedSkill(refreshed);
        if (!refreshed) {
          setSelectedDocument(null);
        }
      }
    } catch (error) {
      setErrorMessage(error instanceof Error ? error.message : String(error));
    } finally {
      setIsLoading(false);
    }
  }, [repoPath, selectedSkill]);

  const saveRepoPath = useCallback(async (nextPath: string) => {
    setIsSavingPath(true);
    try {
      const savedPath = await api.setRepoPath(nextPath);
      setRepoPath(savedPath);
      setActiveView("skills");
      setSelectedSkill(null);
      setSelectedDocument(null);
      setErrorMessage(null);
    } catch (error) {
      setErrorMessage(error instanceof Error ? error.message : String(error));
    } finally {
      setIsSavingPath(false);
    }
  }, []);

  const selectSkill = useCallback(async (skill: SkillSummary | null) => {
    setSelectedSkill(skill);
    if (!skill) {
      setSelectedDocument(null);
      return;
    }

    try {
      const document = await api.getSkillDocument(skill.relativePath);
      setSelectedDocument(document);
      setErrorMessage(null);
    } catch (error) {
      setErrorMessage(error instanceof Error ? error.message : String(error));
      setSelectedDocument(null);
    }
  }, []);

  useEffect(() => {
    void (async () => {
      try {
        const savedPath = await api.getRepoPath();
        setRepoPath(savedPath);
      } catch (error) {
        setErrorMessage(error instanceof Error ? error.message : String(error));
      } finally {
        setIsLoading(false);
      }
    })();
  }, []);

  useEffect(() => {
    void refreshSkills();
  }, [refreshSkills]);

  const value = useMemo<AppContextValue>(
    () => ({
      activeView,
      repoPath,
      scanResult,
      selectedSkill,
      selectedDocument,
      isLoading,
      isSavingPath,
      errorMessage,
      setActiveView,
      refreshSkills,
      saveRepoPath,
      selectSkill,
    }),
    [activeView, repoPath, scanResult, selectedSkill, selectedDocument, isLoading, isSavingPath, errorMessage, refreshSkills, saveRepoPath, selectSkill]
  );

  return <AppContext.Provider value={value}>{children}</AppContext.Provider>;
}

export function useAppContext() {
  const context = useContext(AppContext);
  if (!context) {
    throw new Error("useAppContext must be used inside AppProvider");
  }
  return context;
}
```

- [ ] **Step 3: Build the app shell and the settings form on top of the shared state**

```tsx
// src/components/AppShell.tsx
import type { PropsWithChildren } from "react";
import { useTranslation } from "react-i18next";
import { useAppContext } from "../context/AppContext";

export function AppShell({ children }: PropsWithChildren) {
  const { t } = useTranslation();
  const { activeView, setActiveView, errorMessage } = useAppContext();

  return (
    <div className="min-h-screen bg-slate-950 text-slate-100">
      <header className="border-b border-slate-800 bg-slate-900/90 backdrop-blur">
        <div className="mx-auto flex max-w-7xl items-center justify-between px-6 py-4">
          <div>
            <h1 className="text-2xl font-semibold">{t("app.title")}</h1>
            <p className="text-sm text-slate-400">{t("app.subtitle")}</p>
          </div>
          <nav className="flex gap-2">
            {(["skills", "settings"] as const).map((view) => (
              <button
                key={view}
                className={`rounded-lg px-4 py-2 text-sm ${activeView === view ? "bg-sky-400 text-slate-950" : "bg-slate-800 text-slate-200"}`}
                onClick={() => setActiveView(view)}
              >
                {view === "skills" ? t("nav.skills") : t("nav.settings")}
              </button>
            ))}
          </nav>
        </div>
        {errorMessage ? (
          <div className="border-t border-rose-900/50 bg-rose-950/60 px-6 py-3 text-sm text-rose-200">
            {errorMessage}
          </div>
        ) : null}
      </header>

      <main className="mx-auto max-w-7xl px-6 py-8">{children}</main>
    </div>
  );
}
```

```tsx
// src/components/RepoPathForm.tsx
import { useEffect, useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { useTranslation } from "react-i18next";
import { useAppContext } from "../context/AppContext";

export function RepoPathForm() {
  const { t } = useTranslation();
  const { repoPath, isSavingPath, saveRepoPath } = useAppContext();
  const [draftPath, setDraftPath] = useState(repoPath ?? "");

  useEffect(() => {
    setDraftPath(repoPath ?? "");
  }, [repoPath]);

  const handleBrowse = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: draftPath || undefined,
    });

    if (typeof selected === "string") {
      setDraftPath(selected);
    }
  };

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    await saveRepoPath(draftPath);
  };

  return (
    <form className="space-y-4" onSubmit={handleSubmit}>
      <label className="block text-sm font-medium text-slate-200" htmlFor="repo-path">
        {t("settings.pathLabel")}
      </label>
      <div className="flex gap-3">
        <input
          id="repo-path"
          className="flex-1 rounded-xl border border-slate-700 bg-slate-900 px-4 py-3 text-sm text-slate-100 outline-none focus:border-sky-400"
          placeholder={t("settings.pathPlaceholder")}
          value={draftPath}
          onChange={(event) => setDraftPath(event.target.value)}
        />
        <button
          className="rounded-xl border border-slate-700 bg-slate-800 px-4 py-3 text-sm text-slate-100"
          onClick={handleBrowse}
          type="button"
        >
          {t("settings.browse")}
        </button>
      </div>
      <button
        className="rounded-xl bg-sky-400 px-4 py-3 text-sm font-medium text-slate-950 disabled:cursor-not-allowed disabled:opacity-60"
        disabled={isSavingPath}
        type="submit"
      >
        {t("settings.save")}
      </button>
    </form>
  );
}
```

```tsx
// src/views/SettingsView.tsx
import { useTranslation } from "react-i18next";
import { RepoPathForm } from "../components/RepoPathForm";

export function SettingsView() {
  const { t } = useTranslation();

  return (
    <section className="space-y-6 rounded-2xl border border-slate-800 bg-slate-900 p-6">
      <div>
        <h2 className="text-xl font-semibold">{t("settings.title")}</h2>
        <p className="mt-2 text-sm text-slate-400">{t("settings.description")}</p>
      </div>
      <RepoPathForm />
    </section>
  );
}
```

- [ ] **Step 4: Update the root app entry so i18n and context wrap the two-view shell**

```tsx
// replace src/App.tsx
import { AppShell } from "./components/AppShell";
import { AppProvider, useAppContext } from "./context/AppContext";
import { SettingsView } from "./views/SettingsView";
import { SkillsView } from "./views/SkillsView";

function AppBody() {
  const { activeView } = useAppContext();
  return <AppShell>{activeView === "skills" ? <SkillsView /> : <SettingsView />}</AppShell>;
}

export default function App() {
  return (
    <AppProvider>
      <AppBody />
    </AppProvider>
  );
}
```

```tsx
// replace src/main.tsx
import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./i18n";
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
```

```tsx
// replace src/views/SkillsView.tsx with a state-aware placeholder
import { useTranslation } from "react-i18next";
import { useAppContext } from "../context/AppContext";

export function SkillsView() {
  const { t } = useTranslation();
  const { isLoading, repoPath } = useAppContext();

  return (
    <section className="rounded-2xl border border-slate-800 bg-slate-900 p-6 text-slate-100">
      <h2 className="text-xl font-semibold">{t("nav.skills")}</h2>
      <p className="mt-2 text-sm text-slate-400">
        {repoPath ? t("skills.placeholder.body") : t("settings.description")}
      </p>
      <p className="mt-4 text-sm text-slate-500">
        {isLoading ? t("skills.placeholder.title") : repoPath ?? t("settings.pathPlaceholder")}
      </p>
    </section>
  );
}
```

```css
/* append to src/styles.css */
* {
  box-sizing: border-box;
}

#root {
  min-height: 100vh;
}
```

- [ ] **Step 5: Verify the shared app state and settings workflow**

Run: `npm run build`  
Expected: PASS with the new context, i18n, and dialog-plugin imports resolved.

Run: `npm run verify`  
Expected: PASS with architecture checks, frontend build, `cargo check`, and `cargo test`.

- [ ] **Step 6: Commit the shared app state and settings UI**

```bash
git add src/lib/tauri.ts src/context/AppContext.tsx src/i18n src/components/AppShell.tsx src/components/RepoPathForm.tsx src/views/SettingsView.tsx src/views/SkillsView.tsx src/App.tsx src/main.tsx src/styles.css
git commit -m "feat: wire repository settings workflow"
```

### Task 5: Build The Searchable Skills Browser, Detail Panel, And Final Validation Pass

**Files:**
- Create: `src/lib/skills/filters.ts`
- Create: `src/components/skills/SkillFilters.tsx`
- Create: `src/components/skills/SkillList.tsx`
- Create: `src/components/skills/SkillDetailPanel.tsx`
- Modify: `src/views/SkillsView.tsx`
- Modify: `src/i18n/en.json`
- Modify: `src/i18n/zh.json`
- Modify: `src/context/AppContext.tsx`

- [ ] **Step 1: Add the pure frontend helpers for search, source summaries, and description truncation**

```ts
// src/lib/skills/filters.ts
import type { SkillSummary } from "../tauri";

export type SourceFilter = "all" | "custom" | "external";

export interface SourceSummary {
  key: SourceFilter;
  count: number;
}

export function buildSourceSummaries(skills: SkillSummary[]): SourceSummary[] {
  const custom = skills.filter((skill) => skill.sourceType === "custom").length;
  const external = skills.filter((skill) => skill.sourceType === "external").length;

  return [
    { key: "all", count: skills.length },
    { key: "custom", count: custom },
    { key: "external", count: external },
  ];
}

export function filterSkills(
  skills: SkillSummary[],
  search: string,
  sourceFilter: SourceFilter
) {
  const lowered = search.trim().toLowerCase();

  return skills.filter((skill) => {
    if (sourceFilter !== "all" && skill.sourceType !== sourceFilter) {
      return false;
    }

    if (!lowered) {
      return true;
    }

    return (
      skill.name.toLowerCase().includes(lowered) ||
      skill.description.toLowerCase().includes(lowered) ||
      skill.relativePath.toLowerCase().includes(lowered)
    );
  });
}

export function groupSkills(skills: SkillSummary[]) {
  return {
    custom: skills.filter((skill) => skill.sourceType === "custom"),
    external: skills.filter((skill) => skill.sourceType === "external"),
  };
}

export function truncateDescription(description: string, maxLength = 100) {
  if (description.length <= maxLength) {
    return description;
  }
  return `${description.slice(0, maxLength - 1)}…`;
}
```

- [ ] **Step 2: Implement the filters, grouped list, and Markdown detail panel**

```tsx
// src/components/skills/SkillFilters.tsx
import { useTranslation } from "react-i18next";
import type { SourceFilter, SourceSummary } from "../../lib/skills/filters";

interface SkillFiltersProps {
  search: string;
  onSearchChange: (value: string) => void;
  sourceFilter: SourceFilter;
  onSourceFilterChange: (value: SourceFilter) => void;
  summaries: SourceSummary[];
  onRefresh: () => Promise<void>;
  isRefreshing: boolean;
}

export function SkillFilters(props: SkillFiltersProps) {
  const { search, onSearchChange, sourceFilter, onSourceFilterChange, summaries, onRefresh, isRefreshing } = props;
  const { t } = useTranslation();

  return (
    <div className="space-y-4 rounded-2xl border border-slate-800 bg-slate-900 p-4">
      <div className="flex gap-3">
        <input
          className="flex-1 rounded-xl border border-slate-700 bg-slate-950 px-4 py-3 text-sm text-slate-100 outline-none focus:border-sky-400"
          placeholder={t("skills.search")}
          value={search}
          onChange={(event) => onSearchChange(event.target.value)}
        />
        <button
          className="rounded-xl border border-slate-700 bg-slate-800 px-4 py-3 text-sm text-slate-100 disabled:opacity-60"
          disabled={isRefreshing}
          onClick={() => void onRefresh()}
          type="button"
        >
          {t("skills.refresh")}
        </button>
      </div>
      <div className="flex flex-wrap gap-2">
        {summaries.map((summary) => (
          <button
            key={summary.key}
            className={`rounded-full px-3 py-1.5 text-sm ${sourceFilter === summary.key ? "bg-sky-400 text-slate-950" : "bg-slate-800 text-slate-200"}`}
            onClick={() => onSourceFilterChange(summary.key)}
            type="button"
          >
            {t(`skills.source.${summary.key}`)} ({summary.count})
          </button>
        ))}
      </div>
    </div>
  );
}
```

```tsx
// src/components/skills/SkillList.tsx
import { useTranslation } from "react-i18next";
import type { SkillSummary } from "../../lib/tauri";
import { truncateDescription } from "../../lib/skills/filters";

interface SkillListProps {
  title: string;
  skills: SkillSummary[];
  selectedSkillId: string | null;
  onSelect: (skill: SkillSummary) => void;
}

export function SkillList({ title, skills, selectedSkillId, onSelect }: SkillListProps) {
  const { t } = useTranslation();

  return (
    <section className="space-y-3 rounded-2xl border border-slate-800 bg-slate-900 p-4">
      <header className="flex items-center justify-between">
        <h3 className="text-sm font-semibold uppercase tracking-wide text-slate-300">{title}</h3>
        <span className="text-xs text-slate-500">{skills.length}</span>
      </header>
      <div className="space-y-3">
        {skills.length === 0 ? (
          <div className="rounded-xl border border-dashed border-slate-700 px-4 py-6 text-sm text-slate-500">
            {t("skills.empty")}
          </div>
        ) : null}
        {skills.map((skill) => (
          <button
            key={skill.id}
            className={`w-full rounded-xl border p-4 text-left ${selectedSkillId === skill.id ? "border-sky-400 bg-sky-400/10" : "border-slate-800 bg-slate-950"}`}
            onClick={() => onSelect(skill)}
            type="button"
          >
            <div className="flex items-center justify-between gap-3">
              <strong className="text-sm font-semibold text-slate-100">{skill.name}</strong>
              <span className="rounded-full bg-slate-800 px-2 py-1 text-xs text-slate-300">
                {skill.sourceType === "custom" ? t("skills.source.custom") : t("skills.source.external")}
              </span>
            </div>
            <p className="mt-2 text-sm text-slate-400">
              {skill.description ? truncateDescription(skill.description) : t("skills.noDescription")}
            </p>
            <p className="mt-2 text-xs text-slate-500">{skill.relativePath}</p>
          </button>
        ))}
      </div>
    </section>
  );
}
```

```tsx
// src/components/skills/SkillDetailPanel.tsx
import { useTranslation } from "react-i18next";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import type { SkillDocument, SkillSummary } from "../../lib/tauri";

interface SkillDetailPanelProps {
  skill: SkillSummary | null;
  document: SkillDocument | null;
}

export function SkillDetailPanel({ skill, document }: SkillDetailPanelProps) {
  const { t } = useTranslation();

  if (!skill) {
    return (
      <aside className="rounded-2xl border border-slate-800 bg-slate-900 p-6 text-sm text-slate-400">
        {t("skills.selectPrompt")}
      </aside>
    );
  }

  return (
    <aside className="space-y-4 rounded-2xl border border-slate-800 bg-slate-900 p-6">
      <div className="space-y-2">
        <h3 className="text-xl font-semibold text-slate-100">{skill.name}</h3>
        <p className="text-sm text-slate-400">{skill.description || t("skills.noDescription")}</p>
        <dl className="grid grid-cols-[96px_1fr] gap-2 text-sm text-slate-300">
          <dt className="text-slate-500">{t("skills.detail.sourceLabel")}</dt>
          <dd>{skill.sourceType === "custom" ? t("skills.source.custom") : t("skills.source.external")}</dd>
          <dt className="text-slate-500">{t("skills.detail.pathLabel")}</dt>
          <dd className="break-all">{skill.relativePath}</dd>
        </dl>
      </div>

      <div className="rounded-xl border border-slate-800 bg-slate-950 p-4">
        {document ? (
          <article className="prose prose-invert max-w-none prose-pre:bg-slate-900 prose-code:text-sky-200">
            <ReactMarkdown remarkPlugins={[remarkGfm]}>{document.content}</ReactMarkdown>
          </article>
        ) : (
          <p className="text-sm text-slate-400">{t("skills.detail.loadingDocument")}</p>
        )}
      </div>
    </aside>
  );
}
```

- [ ] **Step 3: Replace the placeholder skills view with the real searchable browser**

```tsx
// replace src/views/SkillsView.tsx
import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { useAppContext } from "../context/AppContext";
import { buildSourceSummaries, filterSkills, groupSkills, type SourceFilter } from "../lib/skills/filters";
import { SkillDetailPanel } from "../components/skills/SkillDetailPanel";
import { SkillFilters } from "../components/skills/SkillFilters";
import { SkillList } from "../components/skills/SkillList";

export function SkillsView() {
  const { t } = useTranslation();
  const { repoPath, scanResult, selectedSkill, selectedDocument, selectSkill, refreshSkills, isLoading, errorMessage } = useAppContext();
  const [search, setSearch] = useState("");
  const [sourceFilter, setSourceFilter] = useState<SourceFilter>("all");

  const filteredSkills = useMemo(
    () => filterSkills(scanResult.skills, search, sourceFilter),
    [scanResult.skills, search, sourceFilter]
  );
  const grouped = useMemo(() => groupSkills(filteredSkills), [filteredSkills]);
  const summaries = useMemo(() => buildSourceSummaries(scanResult.skills), [scanResult.skills]);

  if (!repoPath) {
    return (
      <section className="rounded-2xl border border-dashed border-slate-700 bg-slate-900 p-8 text-center">
        <h2 className="text-xl font-semibold text-slate-100">{t("skills.unconfigured")}</h2>
        <p className="mt-3 text-sm text-slate-400">
          {t("skills.unconfiguredBody")}
        </p>
      </section>
    );
  }

  return (
    <div className="grid gap-6 xl:grid-cols-[1.25fr_1.25fr_1fr]">
      <div className="space-y-6 xl:col-span-2">
        <SkillFilters
          search={search}
          onSearchChange={setSearch}
          sourceFilter={sourceFilter}
          onSourceFilterChange={setSourceFilter}
          summaries={summaries}
          onRefresh={refreshSkills}
          isRefreshing={isLoading}
        />

        {scanResult.warnings.length > 0 ? (
          <div className="rounded-2xl border border-amber-700/50 bg-amber-950/40 p-4 text-sm text-amber-100">
            {scanResult.warnings.join(" ")}
          </div>
        ) : null}

        {errorMessage ? (
          <div className="rounded-2xl border border-rose-700/50 bg-rose-950/40 p-4 text-sm text-rose-100">
            {errorMessage}
          </div>
        ) : null}

        <div className="grid gap-6 lg:grid-cols-2">
          <SkillList
            title={t("skills.section.custom")}
            skills={grouped.custom}
            selectedSkillId={selectedSkill?.id ?? null}
            onSelect={(skill) => void selectSkill(skill)}
          />
          <SkillList
            title={t("skills.section.external")}
            skills={grouped.external}
            selectedSkillId={selectedSkill?.id ?? null}
            onSelect={(skill) => void selectSkill(skill)}
          />
        </div>
      </div>

      <SkillDetailPanel skill={selectedSkill} document={selectedDocument} />
    </div>
  );
}
```

- [ ] **Step 4: Update translations and re-run the full repository gate**

```json
// merge these keys into src/i18n/en.json
{
  "skills.search": "Search by name, description, or path",
  "skills.refresh": "Refresh",
  "skills.empty": "No skills matched the current filter.",
  "skills.unconfigured": "Repository path is not configured",
  "skills.unconfiguredBody": "Open Settings, choose the local my-skills repository, and return here to browse the results.",
  "skills.selectPrompt": "Select a skill to view its metadata and SKILL.md content.",
  "skills.noDescription": "No description",
  "skills.source.all": "All",
  "skills.source.custom": "Custom",
  "skills.source.external": "External",
  "skills.section.custom": "Custom Skills",
  "skills.section.external": "External Skills",
  "skills.detail.sourceLabel": "Source",
  "skills.detail.pathLabel": "Path",
  "skills.detail.loadingDocument": "Loading document…"
}
```

```json
// merge these keys into src/i18n/zh.json
{
  "skills.search": "按名称、描述或路径搜索",
  "skills.refresh": "刷新",
  "skills.empty": "当前筛选条件下没有匹配的技能。",
  "skills.unconfigured": "尚未配置仓库路径",
  "skills.unconfiguredBody": "请先进入设置页面，选择本地 my-skills 仓库，然后回到这里浏览结果。",
  "skills.selectPrompt": "选择一个技能以查看元数据和 SKILL.md 内容。",
  "skills.noDescription": "暂无描述",
  "skills.source.all": "全部",
  "skills.source.custom": "自定义",
  "skills.source.external": "外部",
  "skills.section.custom": "自定义技能",
  "skills.section.external": "外部技能",
  "skills.detail.sourceLabel": "来源",
  "skills.detail.pathLabel": "路径",
  "skills.detail.loadingDocument": "正在加载文档……"
}
```

Run: `npm run verify`  
Expected: PASS with architecture checks, frontend build, `cargo check`, and the full Rust test suite.

Run: `cargo test --manifest-path src-tauri/Cargo.toml`  
Expected: PASS with settings, metadata, scan, and document tests all green.

- [ ] **Step 5: Commit the searchable skills browser**

```bash
git add src/lib/skills/filters.ts src/components/skills src/views/SkillsView.tsx src/i18n/en.json src/i18n/zh.json src/context/AppContext.tsx
git commit -m "feat: add searchable skills browser"
```

# Skill Enable State Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add repo-scoped, app-local skill enable/disable persistence to the existing browser without touching agent sync, scene switching, or project-local output.

**Architecture:** Keep the phase-one scan and document commands as the discovery backbone, add a dedicated Rust `skill-state` persistence module under `src-tauri/src/core/skills`, expose thin Tauri commands to load and update disabled skill IDs, and let `AppContext` join that state with existing scan results for filtering and row-level toggles. Extend the current browser components instead of creating new frontend orchestration layers, and update module docs alongside every source change to satisfy the repo guard.

**Tech Stack:** Tauri 2, Rust 2021, React 19, TypeScript 5, Vite 7, Tailwind CSS 3, i18next, serde_json

---

## File Map

- `src-tauri/src/core/skills/state.rs` — new repo-scoped skill-state store, repo-key normalization, JSON persistence, and unit tests.
- `src-tauri/src/core/skills/mod.rs` — exports the new skill-state module.
- `src-tauri/src/commands/skills.rs` — thin `get_skill_state` and `set_skill_enabled` Tauri commands.
- `src-tauri/src/lib.rs` — registers the new commands.
- `src/lib/tauri.ts` — TypeScript DTOs and invoke wrappers for the new commands.
- `src/context/AppContext.tsx` — owns disabled-ID state, refresh orchestration, and toggle persistence.
- `src/lib/skills/filters.ts` — adds status filters and status summary helpers.
- `src/components/skills/SkillFilters.tsx` — renders status filter chips beside the existing browser filters.
- `src/components/skills/SkillList.tsx` — renders status badges and row-level enable/disable controls.
- `src/components/skills/SkillDetailPanel.tsx` — shows current status in metadata.
- `src/views/SkillsView.tsx` — adds page-local status filter state and passes new props through the existing composition.
- `src/i18n/en.json` / `src/i18n/zh.json` — phase-two copy for status and toggle labels.
- `docs/modules/frontend/context/AppContext.md` — documents new disabled-ID orchestration and toggle action.
- `docs/modules/frontend/lib/tauri.md` — documents the new DTO and command wrappers.
- `docs/modules/frontend/lib/skills/filters.md` — documents status-aware filter helpers.
- `docs/modules/frontend/views/SkillsView.md` — documents combined search/source/status view behavior.
- `docs/modules/frontend/components/skills/SkillFilters.md` — documents status chips.
- `docs/modules/frontend/components/skills/SkillList.md` — documents row toggle behavior.
- `docs/modules/frontend/components/skills/SkillDetailPanel.md` — documents status display.
- `docs/modules/tauri/lib.md` — documents new registered commands.
- `docs/modules/tauri/commands/skills.md` — documents new state commands and command-layer scope.
- `docs/modules/tauri/core/skills/mod.md` — documents the added `state` module export.
- `docs/modules/tauri/core/skills/state.md` — new module doc for repo-scoped state persistence.

### Task 1: Add The Rust Skill-State Persistence Core

**Files:**
- Create: `src-tauri/src/core/skills/state.rs`
- Modify: `src-tauri/src/core/skills/mod.rs`
- Test: `src-tauri/src/core/skills/state.rs`

- [ ] **Step 1: Write the failing Rust tests first**

Add these tests to the new `src-tauri/src/core/skills/state.rs` file before the implementation:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn load_for_repo_returns_empty_state_when_store_does_not_exist() {
        let dir = tempdir().unwrap();
        let repo = tempdir().unwrap();
        let store = SkillStateStore::new(dir.path().to_path_buf());

        let snapshot = store.load_for_repo(repo.path()).unwrap();

        assert!(snapshot.disabled_skill_ids.is_empty());
    }

    #[test]
    fn set_skill_enabled_persists_disabled_ids_for_one_repo() {
        let dir = tempdir().unwrap();
        let repo = tempdir().unwrap();
        let store = SkillStateStore::new(dir.path().to_path_buf());

        let snapshot = store
            .set_skill_enabled(repo.path(), "custom:searxng", false)
            .unwrap();

        assert_eq!(snapshot.disabled_skill_ids, vec!["custom:searxng".to_string()]);
        assert_eq!(
            store.load_for_repo(repo.path()).unwrap().disabled_skill_ids,
            vec!["custom:searxng".to_string()]
        );
    }

    #[test]
    fn enabling_skill_removes_it_from_disabled_ids() {
        let dir = tempdir().unwrap();
        let repo = tempdir().unwrap();
        let store = SkillStateStore::new(dir.path().to_path_buf());

        store
            .set_skill_enabled(repo.path(), "custom:searxng", false)
            .unwrap();
        let snapshot = store
            .set_skill_enabled(repo.path(), "custom:searxng", true)
            .unwrap();

        assert!(snapshot.disabled_skill_ids.is_empty());
    }

    #[test]
    fn state_is_scoped_per_repo_path() {
        let dir = tempdir().unwrap();
        let repo_a = tempdir().unwrap();
        let repo_b = tempdir().unwrap();
        let store = SkillStateStore::new(dir.path().to_path_buf());

        store
            .set_skill_enabled(repo_a.path(), "custom:searxng", false)
            .unwrap();

        assert_eq!(
            store.load_for_repo(repo_a.path()).unwrap().disabled_skill_ids,
            vec!["custom:searxng".to_string()]
        );
        assert!(store.load_for_repo(repo_b.path()).unwrap().disabled_skill_ids.is_empty());
    }

    #[test]
    fn load_for_repo_tolerates_unknown_disabled_ids() {
        let dir = tempdir().unwrap();
        let repo = tempdir().unwrap();
        let store = SkillStateStore::new(dir.path().to_path_buf());
        let repo_key = build_repo_state_key(repo.path()).unwrap();

        fs::create_dir_all(dir.path()).unwrap();
        fs::write(
            dir.path().join("skill-state.json"),
            format!(
                "{{\"repos\":{{\"{repo_key}\":{{\"repoPath\":\"{}\",\"disabledSkillIds\":[\"external:missing-skill\"]}}}}}}",
                normalize_repo_path(repo.path())
            ),
        )
        .unwrap();

        let snapshot = store.load_for_repo(repo.path()).unwrap();

        assert_eq!(
            snapshot.disabled_skill_ids,
            vec!["external:missing-skill".to_string()]
        );
    }
}
```

- [ ] **Step 2: Run the targeted Rust tests and confirm they fail**

Run: `cargo test --manifest-path src-tauri/Cargo.toml state -- --nocapture`  
Expected: FAIL with compile errors because `SkillStateStore`, `SkillStateSnapshot`, `build_repo_state_key`, and `normalize_repo_path` do not exist yet.

- [ ] **Step 3: Implement the repo-scoped state store**

Replace `src-tauri/src/core/skills/state.rs` with this implementation:

```rust
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SkillStateSnapshot {
    pub disabled_skill_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct PersistedSkillStateFile {
    pub repos: BTreeMap<String, PersistedRepoSkillState>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct PersistedRepoSkillState {
    pub repo_path: String,
    pub disabled_skill_ids: Vec<String>,
}

pub struct SkillStateStore {
    base_dir: PathBuf,
}

impl SkillStateStore {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    pub fn load_for_repo(&self, repo_path: &Path) -> Result<SkillStateSnapshot> {
        let repo_key = build_repo_state_key(repo_path)?;
        let file = self.load_file()?;

        Ok(file
            .repos
            .get(&repo_key)
            .map(|repo_state| SkillStateSnapshot {
                disabled_skill_ids: normalize_disabled_skill_ids(&repo_state.disabled_skill_ids),
            })
            .unwrap_or_default())
    }

    pub fn set_skill_enabled(
        &self,
        repo_path: &Path,
        skill_id: &str,
        enabled: bool,
    ) -> Result<SkillStateSnapshot> {
        fs::create_dir_all(&self.base_dir)
            .with_context(|| format!("Failed to create {:?}", self.base_dir))?;

        let repo_key = build_repo_state_key(repo_path)?;
        let normalized_repo_path = normalize_repo_path(repo_path);
        let mut file = self.load_file()?;

        let repo_state = file
            .repos
            .entry(repo_key)
            .or_insert_with(|| PersistedRepoSkillState {
                repo_path: normalized_repo_path.clone(),
                disabled_skill_ids: Vec::new(),
            });

        let mut disabled_ids = repo_state
            .disabled_skill_ids
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();

        if enabled {
            disabled_ids.remove(skill_id);
        } else {
            disabled_ids.insert(skill_id.to_string());
        }

        repo_state.repo_path = normalized_repo_path;
        repo_state.disabled_skill_ids = disabled_ids.into_iter().collect();

        let snapshot = SkillStateSnapshot {
            disabled_skill_ids: repo_state.disabled_skill_ids.clone(),
        };

        self.save_file(&file)?;

        Ok(snapshot)
    }

    fn load_file(&self) -> Result<PersistedSkillStateFile> {
        let path = self.state_path();
        if !path.exists() {
            return Ok(PersistedSkillStateFile::default());
        }

        let raw = fs::read_to_string(&path).with_context(|| format!("Failed to read {:?}", path))?;
        serde_json::from_str::<PersistedSkillStateFile>(&raw)
            .with_context(|| format!("Failed to parse {:?}", path))
    }

    fn save_file(&self, file: &PersistedSkillStateFile) -> Result<()> {
        let json = serde_json::to_string_pretty(file)?;
        fs::write(self.state_path(), json).context("Failed to write skill-state.json")?;
        Ok(())
    }

    fn state_path(&self) -> PathBuf {
        self.base_dir.join("skill-state.json")
    }
}

pub fn build_repo_state_key(repo_path: &Path) -> Result<String> {
    let canonical = repo_path
        .canonicalize()
        .unwrap_or_else(|_| repo_path.to_path_buf());
    Ok(normalize_repo_path(&canonical))
}

pub fn normalize_repo_path(path: &Path) -> String {
    let normalized = path.to_string_lossy().replace('\\', "/");

    if cfg!(windows) {
        normalized.to_lowercase()
    } else {
        normalized
    }
}

fn normalize_disabled_skill_ids(ids: &[String]) -> Vec<String> {
    ids.iter()
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}
```

Update `src-tauri/src/core/skills/mod.rs` to export the module:

```rust
pub mod documents;
pub mod metadata;
pub mod scan;
pub mod state;
```

- [ ] **Step 4: Re-run the targeted Rust tests and confirm they pass**

Run: `cargo test --manifest-path src-tauri/Cargo.toml state -- --nocapture`  
Expected: PASS for all five new state-store tests.

### Task 2: Expose Thin Skill-State Commands And Frontend API Wrappers

**Files:**
- Modify: `src-tauri/src/commands/skills.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src/lib/tauri.ts`

- [ ] **Step 1: Extend the Rust command layer without moving business logic into commands**

Update `src-tauri/src/commands/skills.rs` to add a shared repo-path loader plus two new commands:

```rust
use std::path::Path;
use tauri::Manager;

use crate::core::settings::SettingsStore;
use crate::core::skills::documents::{read_skill_document, SkillDocument};
use crate::core::skills::scan::{scan_repo_skills, ScanSkillsResponse};
use crate::core::skills::state::{SkillStateSnapshot, SkillStateStore};

fn load_repo_path(app: &tauri::AppHandle) -> Result<String, String> {
    let config_dir = app.path().app_config_dir().map_err(|error| error.to_string())?;
    let settings = SettingsStore::new(config_dir)
        .load()
        .map_err(|error| error.to_string())?;

    settings
        .repo_path
        .ok_or_else(|| "Repository path is not configured".to_string())
}

#[tauri::command]
pub fn scan_skills(app: tauri::AppHandle) -> Result<ScanSkillsResponse, String> {
    let repo_path = load_repo_path(&app)?;
    scan_repo_skills(Path::new(&repo_path)).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_skill_document(app: tauri::AppHandle, relative_path: String) -> Result<SkillDocument, String> {
    let repo_path = load_repo_path(&app)?;
    read_skill_document(Path::new(&repo_path), &relative_path).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_skill_state(app: tauri::AppHandle) -> Result<SkillStateSnapshot, String> {
    let repo_path = load_repo_path(&app)?;
    let config_dir = app.path().app_config_dir().map_err(|error| error.to_string())?;

    SkillStateStore::new(config_dir)
        .load_for_repo(Path::new(&repo_path))
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn set_skill_enabled(
    app: tauri::AppHandle,
    skill_id: String,
    enabled: bool,
) -> Result<SkillStateSnapshot, String> {
    if skill_id.trim().is_empty() {
        return Err("Skill id is required".to_string());
    }

    let repo_path = load_repo_path(&app)?;
    let config_dir = app.path().app_config_dir().map_err(|error| error.to_string())?;

    SkillStateStore::new(config_dir)
        .set_skill_enabled(Path::new(&repo_path), &skill_id, enabled)
        .map_err(|error| error.to_string())
}
```

- [ ] **Step 2: Register the new commands in the Tauri app entry**

Update `src-tauri/src/lib.rs`:

```rust
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
            commands::skills::get_skill_document,
            commands::skills::get_skill_state,
            commands::skills::set_skill_enabled
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 3: Add the matching TypeScript DTO and invoke wrappers**

Update `src/lib/tauri.ts`:

```ts
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

export interface SkillStateSnapshot {
  disabledSkillIds: string[];
}

export const getRepoPath = () => invoke<string | null>("get_repo_path");

export const setRepoPath = (path: string) =>
  invoke<string | null>("set_repo_path", { path });

export const scanSkills = () => invoke<ScanSkillsResponse>("scan_skills");

export const getSkillDocument = (relativePath: string) =>
  invoke<SkillDocument>("get_skill_document", { relativePath });

export const getSkillState = () =>
  invoke<SkillStateSnapshot>("get_skill_state");

export const setSkillEnabled = (skillId: string, enabled: boolean) =>
  invoke<SkillStateSnapshot>("set_skill_enabled", { skillId, enabled });
```

- [ ] **Step 4: Verify the new command surface compiles before touching React state**

Run: `cargo check --manifest-path src-tauri/Cargo.toml`  
Expected: PASS with the new command signatures and `state.rs` module wired.

Run: `npm run build`  
Expected: PASS with `src/lib/tauri.ts` compiling and no missing exported types.

### Task 3: Teach AppContext To Load And Persist Skill-State

**Files:**
- Modify: `src/context/AppContext.tsx`

- [ ] **Step 1: Extend the context contract with disabled-ID state and one toggle action**

Update the `AppContextValue` interface and local state:

```tsx
interface AppContextValue {
  activeView: AppView;
  repoPath: string | null;
  scanResult: ScanSkillsResponse;
  selectedSkill: SkillSummary | null;
  selectedDocument: SkillDocument | null;
  disabledSkillIds: string[];
  isLoading: boolean;
  isSavingPath: boolean;
  updatingSkillId: string | null;
  errorMessage: string | null;
  setActiveView: (view: AppView) => void;
  refreshSkills: () => Promise<void>;
  saveRepoPath: (nextPath: string) => Promise<void>;
  selectSkill: (skill: SkillSummary | null) => Promise<void>;
  setSkillEnabled: (skillId: string, enabled: boolean) => Promise<void>;
}
```

```tsx
const [disabledSkillIds, setDisabledSkillIds] = useState<string[]>([]);
const [updatingSkillId, setUpdatingSkillId] = useState<string | null>(null);
```

- [ ] **Step 2: Update refresh flow so scan and state both load for the current repo**

Replace the current `refreshSkills` callback with this version:

```tsx
const refreshSkills = useCallback(async () => {
  if (!repoPath) {
    setScanResult({ skills: [], warnings: [] });
    setSelectedSkill(null);
    setSelectedDocument(null);
    setDisabledSkillIds([]);
    return;
  }

  setIsLoading(true);
  try {
    const response = await api.scanSkills();
    setScanResult(response);

    let nextError: string | null = null;

    try {
      const state = await api.getSkillState();
      setDisabledSkillIds(state.disabledSkillIds);
    } catch (error) {
      setDisabledSkillIds([]);
      nextError = error instanceof Error ? error.message : String(error);
    }

    setSelectedSkill((currentSkill) => {
      if (!currentSkill) {
        return null;
      }

      const refreshedSkill =
        response.skills.find((skill) => skill.id === currentSkill.id) ?? null;
      if (!refreshedSkill) {
        setSelectedDocument(null);
      }
      return refreshedSkill;
    });

    setErrorMessage(nextError);
  } catch (error) {
    setErrorMessage(error instanceof Error ? error.message : String(error));
  } finally {
    setIsLoading(false);
  }
}, [repoPath]);
```

- [ ] **Step 3: Reset skill-state when the repo path changes and add the toggle action**

Update `saveRepoPath` and add `setSkillEnabled`:

```tsx
const saveRepoPath = useCallback(async (nextPath: string) => {
  setIsSavingPath(true);
  try {
    const savedPath = await api.setRepoPath(nextPath);
    setRepoPath(savedPath);
    setSelectedSkill(null);
    setSelectedDocument(null);
    setDisabledSkillIds([]);
    setActiveView("skills");
    setErrorMessage(null);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    setErrorMessage(message);
    throw error instanceof Error ? error : new Error(message);
  } finally {
    setIsSavingPath(false);
  }
}, []);

const setSkillEnabled = useCallback(async (skillId: string, enabled: boolean) => {
  setUpdatingSkillId(skillId);
  try {
    const snapshot = await api.setSkillEnabled(skillId, enabled);
    setDisabledSkillIds(snapshot.disabledSkillIds);
    setErrorMessage(null);
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    setErrorMessage(message);
    throw error instanceof Error ? error : new Error(message);
  } finally {
    setUpdatingSkillId(null);
  }
}, []);
```

- [ ] **Step 4: Publish the new values from the provider**

Update the `value` object returned by `useMemo`:

```tsx
const value = useMemo<AppContextValue>(
  () => ({
    activeView,
    repoPath,
    scanResult,
    selectedSkill,
    selectedDocument,
    disabledSkillIds,
    isLoading,
    isSavingPath,
    updatingSkillId,
    errorMessage,
    setActiveView,
    refreshSkills,
    saveRepoPath,
    selectSkill,
    setSkillEnabled,
  }),
  [
    activeView,
    repoPath,
    scanResult,
    selectedSkill,
    selectedDocument,
    disabledSkillIds,
    isLoading,
    isSavingPath,
    updatingSkillId,
    errorMessage,
    refreshSkills,
    saveRepoPath,
    selectSkill,
    setSkillEnabled,
  ],
);
```

- [ ] **Step 5: Build once before changing any browser UI**

Run: `npm run build`  
Expected: PASS with the context type changes fully wired and no missing consumer props yet if UI stubs still compile.

### Task 4: Add Status Filters, Row Toggles, And Status Metadata To The Browser UI

**Files:**
- Modify: `src/lib/skills/filters.ts`
- Modify: `src/components/skills/SkillFilters.tsx`
- Modify: `src/components/skills/SkillList.tsx`
- Modify: `src/components/skills/SkillDetailPanel.tsx`
- Modify: `src/views/SkillsView.tsx`
- Modify: `src/i18n/en.json`
- Modify: `src/i18n/zh.json`

- [ ] **Step 1: Extend the pure filter helpers with skill-status support**

Replace `src/lib/skills/filters.ts` with:

```ts
import type { SkillSummary } from "../tauri";

export type SourceFilter = "all" | "custom" | "external";
export type SkillStatusFilter = "all" | "enabled" | "disabled";

export interface SourceSummary {
  key: SourceFilter;
  count: number;
}

export interface StatusSummary {
  key: SkillStatusFilter;
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

export function buildStatusSummaries(
  skills: SkillSummary[],
  disabledSkillIds: ReadonlySet<string>,
): StatusSummary[] {
  const disabled = skills.filter((skill) => disabledSkillIds.has(skill.id)).length;
  const enabled = skills.length - disabled;

  return [
    { key: "all", count: skills.length },
    { key: "enabled", count: enabled },
    { key: "disabled", count: disabled },
  ];
}

export function filterSkills(
  skills: SkillSummary[],
  search: string,
  sourceFilter: SourceFilter,
  statusFilter: SkillStatusFilter,
  disabledSkillIds: ReadonlySet<string>,
) {
  const lowered = search.trim().toLowerCase();

  return skills.filter((skill) => {
    if (sourceFilter !== "all" && skill.sourceType !== sourceFilter) {
      return false;
    }

    const isDisabled = disabledSkillIds.has(skill.id);
    if (statusFilter === "enabled" && isDisabled) {
      return false;
    }
    if (statusFilter === "disabled" && !isDisabled) {
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

- [ ] **Step 2: Update the filter controls to render status chips**

Update `src/components/skills/SkillFilters.tsx`:

```tsx
import { useTranslation } from "react-i18next";
import type {
  SkillStatusFilter,
  SourceFilter,
  SourceSummary,
  StatusSummary,
} from "../../lib/skills/filters";

interface SkillFiltersProps {
  search: string;
  onSearchChange: (value: string) => void;
  sourceFilter: SourceFilter;
  onSourceFilterChange: (value: SourceFilter) => void;
  statusFilter: SkillStatusFilter;
  onStatusFilterChange: (value: SkillStatusFilter) => void;
  summaries: SourceSummary[];
  statusSummaries: StatusSummary[];
  onRefresh: () => Promise<void>;
  isRefreshing: boolean;
}

export function SkillFilters(props: SkillFiltersProps) {
  const {
    isRefreshing,
    onRefresh,
    onSearchChange,
    onSourceFilterChange,
    onStatusFilterChange,
    search,
    sourceFilter,
    statusFilter,
    summaries,
    statusSummaries,
  } = props;
  const { t } = useTranslation();

  return (
    <div className="space-y-4 rounded-2xl border border-slate-800 bg-slate-900 p-4">
      <div className="flex gap-3">
        <input
          className="flex-1 rounded-xl border border-slate-700 bg-slate-950 px-4 py-3 text-sm text-slate-100 outline-none focus:border-sky-400"
          onChange={(event) => onSearchChange(event.target.value)}
          placeholder={t("skills.search")}
          value={search}
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

      <div className="space-y-2">
        <p className="text-xs font-semibold uppercase tracking-wide text-slate-500">
          {t("skills.filters.source")}
        </p>
        <div className="flex flex-wrap gap-2">
          {summaries.map((summary) => (
            <button
              key={summary.key}
              className={`rounded-full px-3 py-1.5 text-sm ${
                sourceFilter === summary.key
                  ? "bg-sky-400 text-slate-950"
                  : "bg-slate-800 text-slate-200"
              }`}
              onClick={() => onSourceFilterChange(summary.key)}
              type="button"
            >
              {t(`skills.source.${summary.key}`)} ({summary.count})
            </button>
          ))}
        </div>
      </div>

      <div className="space-y-2">
        <p className="text-xs font-semibold uppercase tracking-wide text-slate-500">
          {t("skills.filters.status")}
        </p>
        <div className="flex flex-wrap gap-2">
          {statusSummaries.map((summary) => (
            <button
              key={summary.key}
              className={`rounded-full px-3 py-1.5 text-sm ${
                statusFilter === summary.key
                  ? "bg-emerald-400 text-slate-950"
                  : "bg-slate-800 text-slate-200"
              }`}
              onClick={() => onStatusFilterChange(summary.key)}
              type="button"
            >
              {t(`skills.status.${summary.key}`)} ({summary.count})
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}
```

- [ ] **Step 3: Add row-level status rendering and toggle actions**

Update `src/components/skills/SkillList.tsx`:

```tsx
import { useTranslation } from "react-i18next";
import type { SkillSummary } from "../../lib/tauri";
import { truncateDescription } from "../../lib/skills/filters";

interface SkillListProps {
  title: string;
  skills: SkillSummary[];
  selectedSkillId: string | null;
  disabledSkillIds: ReadonlySet<string>;
  updatingSkillId: string | null;
  onSelect: (skill: SkillSummary) => void;
  onToggleEnabled: (skillId: string, enabled: boolean) => Promise<void>;
}

export function SkillList(props: SkillListProps) {
  const {
    onSelect,
    onToggleEnabled,
    selectedSkillId,
    skills,
    title,
    disabledSkillIds,
    updatingSkillId,
  } = props;
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
        {skills.map((skill) => {
          const isDisabled = disabledSkillIds.has(skill.id);
          const isUpdating = updatingSkillId === skill.id;

          return (
            <article
              key={skill.id}
              className={`rounded-xl border p-4 transition ${
                selectedSkillId === skill.id
                  ? "border-sky-400 bg-sky-400/10"
                  : "border-slate-800 bg-slate-950"
              } ${isDisabled ? "opacity-70" : ""}`}
            >
              <div className="flex items-start justify-between gap-3">
                <button
                  className="min-w-0 flex-1 text-left"
                  onClick={() => onSelect(skill)}
                  type="button"
                >
                  <div className="flex flex-wrap items-center gap-2">
                    <strong className="text-sm font-semibold text-slate-100">{skill.name}</strong>
                    <span className="rounded-full bg-slate-800 px-2 py-1 text-xs text-slate-300">
                      {skill.sourceType === "custom"
                        ? t("skills.source.custom")
                        : t("skills.source.external")}
                    </span>
                    <span
                      className={`rounded-full px-2 py-1 text-xs ${
                        isDisabled
                          ? "bg-amber-500/15 text-amber-200"
                          : "bg-emerald-500/15 text-emerald-200"
                      }`}
                    >
                      {isDisabled
                        ? t("skills.status.disabled")
                        : t("skills.status.enabled")}
                    </span>
                  </div>
                  <p className="mt-2 text-sm text-slate-400">
                    {skill.description
                      ? truncateDescription(skill.description)
                      : t("skills.noDescription")}
                  </p>
                  <p className="mt-2 text-xs text-slate-500">{skill.relativePath}</p>
                </button>

                <button
                  className="rounded-lg border border-slate-700 bg-slate-900 px-3 py-2 text-xs text-slate-100 disabled:opacity-60"
                  disabled={isUpdating}
                  onClick={() => void onToggleEnabled(skill.id, isDisabled)}
                  type="button"
                >
                  {isDisabled ? t("skills.toggle.enable") : t("skills.toggle.disable")}
                </button>
              </div>
            </article>
          );
        })}
      </div>
    </section>
  );
}
```

- [ ] **Step 4: Surface status in the detail panel and wire the view-level filter state**

Update `src/components/skills/SkillDetailPanel.tsx`:

```tsx
import { useTranslation } from "react-i18next";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import type { SkillDocument, SkillSummary } from "../../lib/tauri";

interface SkillDetailPanelProps {
  skill: SkillSummary | null;
  document: SkillDocument | null;
  isEnabled: boolean;
}

export function SkillDetailPanel({ document, isEnabled, skill }: SkillDetailPanelProps) {
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
        <p className="text-sm text-slate-400">
          {skill.description || t("skills.noDescription")}
        </p>
        <dl className="grid grid-cols-[96px_1fr] gap-2 text-sm text-slate-300">
          <dt className="text-slate-500">{t("skills.detail.sourceLabel")}</dt>
          <dd>
            {skill.sourceType === "custom"
              ? t("skills.source.custom")
              : t("skills.source.external")}
          </dd>
          <dt className="text-slate-500">{t("skills.detail.statusLabel")}</dt>
          <dd>{isEnabled ? t("skills.status.enabled") : t("skills.status.disabled")}</dd>
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

Replace `src/views/SkillsView.tsx` with:

```tsx
import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { SkillDetailPanel } from "../components/skills/SkillDetailPanel";
import { SkillFilters } from "../components/skills/SkillFilters";
import { SkillList } from "../components/skills/SkillList";
import { useAppContext } from "../context/AppContext";
import {
  buildSourceSummaries,
  buildStatusSummaries,
  filterSkills,
  groupSkills,
  type SkillStatusFilter,
  type SourceFilter,
} from "../lib/skills/filters";

export function SkillsView() {
  const { t } = useTranslation();
  const {
    disabledSkillIds,
    errorMessage,
    isLoading,
    refreshSkills,
    repoPath,
    scanResult,
    selectSkill,
    selectedDocument,
    selectedSkill,
    setSkillEnabled,
    updatingSkillId,
  } = useAppContext();
  const [search, setSearch] = useState("");
  const [sourceFilter, setSourceFilter] = useState<SourceFilter>("all");
  const [statusFilter, setStatusFilter] = useState<SkillStatusFilter>("all");

  const disabledSkillIdSet = useMemo(
    () => new Set(disabledSkillIds),
    [disabledSkillIds],
  );
  const filteredSkills = useMemo(
    () =>
      filterSkills(
        scanResult.skills,
        search,
        sourceFilter,
        statusFilter,
        disabledSkillIdSet,
      ),
    [scanResult.skills, search, sourceFilter, statusFilter, disabledSkillIdSet],
  );
  const grouped = useMemo(() => groupSkills(filteredSkills), [filteredSkills]);
  const summaries = useMemo(
    () => buildSourceSummaries(scanResult.skills),
    [scanResult.skills],
  );
  const statusSummaries = useMemo(
    () => buildStatusSummaries(scanResult.skills, disabledSkillIdSet),
    [scanResult.skills, disabledSkillIdSet],
  );
  const selectedSkillEnabled = selectedSkill
    ? !disabledSkillIdSet.has(selectedSkill.id)
    : true;

  return (
    <>
      {!repoPath ? (
        <section className="rounded-2xl border border-dashed border-slate-700 bg-slate-900 p-8 text-center">
          <h2 className="text-xl font-semibold text-slate-100">
            {t("skills.unconfigured")}
          </h2>
          <p className="mt-3 text-sm text-slate-400">{t("skills.unconfiguredBody")}</p>
        </section>
      ) : (
        <div className="grid gap-6 xl:grid-cols-[1.25fr_1.25fr_1fr]">
          <div className="space-y-6 xl:col-span-2">
            <SkillFilters
              isRefreshing={isLoading}
              onRefresh={refreshSkills}
              onSearchChange={setSearch}
              onSourceFilterChange={setSourceFilter}
              onStatusFilterChange={setStatusFilter}
              search={search}
              sourceFilter={sourceFilter}
              statusFilter={statusFilter}
              summaries={summaries}
              statusSummaries={statusSummaries}
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
                disabledSkillIds={disabledSkillIdSet}
                onSelect={(skill) => void selectSkill(skill)}
                onToggleEnabled={setSkillEnabled}
                selectedSkillId={selectedSkill?.id ?? null}
                skills={grouped.custom}
                title={t("skills.section.custom")}
                updatingSkillId={updatingSkillId}
              />
              <SkillList
                disabledSkillIds={disabledSkillIdSet}
                onSelect={(skill) => void selectSkill(skill)}
                onToggleEnabled={setSkillEnabled}
                selectedSkillId={selectedSkill?.id ?? null}
                skills={grouped.external}
                title={t("skills.section.external")}
                updatingSkillId={updatingSkillId}
              />
            </div>
          </div>

          <SkillDetailPanel
            document={selectedDocument}
            isEnabled={selectedSkillEnabled}
            skill={selectedSkill}
          />
        </div>
      )}
    </>
  );
}
```

- [ ] **Step 5: Add the phase-two UI copy**

Merge these keys into `src/i18n/en.json`:

```json
{
  "skills.filters.source": "Source",
  "skills.filters.status": "Status",
  "skills.status.all": "All",
  "skills.status.enabled": "Enabled",
  "skills.status.disabled": "Disabled",
  "skills.toggle.enable": "Enable",
  "skills.toggle.disable": "Disable",
  "skills.detail.statusLabel": "Status"
}
```

Merge these keys into `src/i18n/zh.json`:

```json
{
  "skills.filters.source": "来源",
  "skills.filters.status": "状态",
  "skills.status.all": "全部",
  "skills.status.enabled": "已启用",
  "skills.status.disabled": "已禁用",
  "skills.toggle.enable": "启用",
  "skills.toggle.disable": "禁用",
  "skills.detail.statusLabel": "状态"
}
```

- [ ] **Step 6: Run the browser-facing verification pass**

Run: `npm run build`  
Expected: PASS with the new status filter props, row toggle props, and i18n keys wired correctly.

### Task 5: Update Module Docs And Run The Full Repository Gate

**Files:**
- Create: `docs/modules/tauri/core/skills/state.md`
- Modify: `docs/modules/tauri/core/skills/mod.md`
- Modify: `docs/modules/tauri/commands/skills.md`
- Modify: `docs/modules/tauri/lib.md`
- Modify: `docs/modules/frontend/context/AppContext.md`
- Modify: `docs/modules/frontend/lib/tauri.md`
- Modify: `docs/modules/frontend/lib/skills/filters.md`
- Modify: `docs/modules/frontend/views/SkillsView.md`
- Modify: `docs/modules/frontend/components/skills/SkillFilters.md`
- Modify: `docs/modules/frontend/components/skills/SkillList.md`
- Modify: `docs/modules/frontend/components/skills/SkillDetailPanel.md`

- [ ] **Step 1: Add the new module doc for the Rust skill-state store**

Create `docs/modules/tauri/core/skills/state.md`:

````md
# Skill State Store

> **Source**: `src-tauri/src/core/skills/state.rs`
> **Status**: [REVIEW]

## Overview

Persists repo-scoped skill enable/disable state as app-local JSON keyed by a normalized repository path.

## Import Relationships

```text
Upstream: src-tauri/src/commands/skills.rs
Downstream: serde_json, std::fs
```

## Public Surface

| Export | Purpose |
|---|---|
| `SkillStateSnapshot` | Serializable disabled-ID snapshot returned to the frontend. |
| `SkillStateStore` | Loads and updates repo-scoped persisted skill state. |
| `build_repo_state_key` | Produces a normalized repo bucket key. |
| `normalize_repo_path` | Converts repo paths to forward-slash normalized strings. |

## Core Logic

The store keeps a JSON map of repository buckets and persists only disabled skill IDs, treating all discovered skills as enabled by default.

## Data Flow

Tauri commands resolve the configured repo path, then delegate load and mutation operations into this store.

## Interactions

Must stay aligned with the stable skill IDs produced by `src-tauri/src/core/skills/scan.rs`.

## Configuration

Writes `skill-state.json` under the app config directory.

## Change Notes

Keep scene/project overrides out of this module until those features exist.
````

- [ ] **Step 2: Update the existing module docs so they mention phase-two responsibilities**

Apply these focused content changes:

```md
docs/modules/tauri/core/skills/mod.md
- Add `state.rs` to the exported module list and mention repo-scoped local state persistence.

docs/modules/tauri/commands/skills.md
- Expand the overview and public surface to include `get_skill_state` and `set_skill_enabled`.
- Update change notes to reinforce that persistence rules stay in `core/skills/state.rs`.

docs/modules/tauri/lib.md
- Update the registered-command list to include the two new skills-state commands.

docs/modules/frontend/context/AppContext.md
- Expand overview/core logic to mention disabled-ID state, refresh joining, and row-level toggle persistence.

docs/modules/frontend/lib/tauri.md
- Add `SkillStateSnapshot`, `getSkillState`, and `setSkillEnabled` to the public surface.

docs/modules/frontend/lib/skills/filters.md
- Add status summaries and status filtering to the core logic description.

docs/modules/frontend/views/SkillsView.md
- Add status-filter orchestration and the joined disabled-ID set to the overview/data flow sections.

docs/modules/frontend/components/skills/SkillFilters.md
- Mention source chips plus status chips.

docs/modules/frontend/components/skills/SkillList.md
- Mention row badges, toggle button, and disabled visual state.

docs/modules/frontend/components/skills/SkillDetailPanel.md
- Mention read-only enabled/disabled metadata display.
```

- [ ] **Step 3: Run the full repo gate**

Run: `npm run verify`  
Expected: PASS with module-doc coverage, module-doc diff checks, architecture checks, frontend build, `cargo check`, and `cargo test`.

Run: `cargo test --manifest-path src-tauri/Cargo.toml`  
Expected: PASS with the new `state.rs` tests plus the existing settings, metadata, scan, and document tests.

- [ ] **Step 4: Do a final manual scope check before implementation handoff**

Confirm these are still true:

- no agent-sync code was added
- no project-directory writes were added
- `settings.rs` still only owns `repo_path`
- `scan.rs` still owns discovery only
- the new persistence lives in `src-tauri/src/core/skills/state.rs`
- module docs exist for every touched source file

## Self-Review Checklist

- Spec coverage: this plan covers repo-scoped local persistence, stable-ID join, frontend toggle/filter wiring, module-doc updates, and full verification.
- Disallowed-marker scan: no banned placeholder markers remain.
- Type consistency: the plan uses `SkillStateSnapshot`, `disabledSkillIds`, `setSkillEnabled`, and `updatingSkillId` consistently across Rust and TypeScript tasks.

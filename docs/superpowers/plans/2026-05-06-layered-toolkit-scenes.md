# Layered Toolkit Scenes Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement layered toolkit scenes so project assignments inherit global agent scenes/direct skills, then add project scenes/direct skills and project-local exclusions.

**Architecture:** Keep scenes as reusable toolkit definitions in `scene-config.json`. Extend project assignment storage from flat project-level skill/agent lists into per-agent project entries, then share one resolution contract for global and project targets. Keep filesystem writes behind explicit global/project apply commands and use target-scoped apply metadata to show stale/current state.

**Tech Stack:** Tauri 2, Rust, serde JSON config stores, React 19, TypeScript, Vite, module-doc guard scripts, existing target manifest/sync helpers.

---

## Scope And Sequencing

This is a structural migration. Do not implement it as a single patch. The safe order is:

1. Add resolution diagnostics and tests without changing UI.
2. Migrate project storage to per-agent entries with backward compatibility.
3. Switch project sync to layered resolution and configured sync mode.
4. Remove the old scene-as-active-setup apply path from normal UI behavior.
5. Update Projects UI to edit per-agent project layers.
6. Add apply staleness metadata and preview labels.

Before editing any Rust symbol named below, follow `AGENTS.md` and run GitNexus impact analysis for that symbol. If impact returns HIGH or CRITICAL, stop and report the blast radius before editing.

## File Map

- Modify: `src-tauri/src/core/agents/selection.rs`
  - Own shared global and project skill resolution, source labels, missing-reference diagnostics, and hash input.
- Modify: `docs/modules/tauri/core/agents/selection.md`
  - Document the expanded resolution contract.
- Modify: `src-tauri/src/core/agents/sync.rs`
  - Consume the updated global resolution result.
- Modify: `src-tauri/src/core/agents/sync_tests.rs`
  - Preserve existing global sync behavior against the new result type.
- Modify: `src-tauri/src/core/projects/store.rs`
  - Add per-agent project entries and legacy flat-shape migration.
- Modify: `src-tauri/src/core/projects/store_tests.rs`
  - Cover migration, normalization, stale unsupported agents, and round trips.
- Modify: `docs/modules/tauri/core/projects/store.md`
  - Document the new project config shape and migration rule.
- Modify: `src-tauri/src/core/projects/sync.rs`
  - Resolve project targets from inherited global layer plus project layer, use configured sync mode, and persist target apply metadata.
- Modify: `src-tauri/src/core/projects/sync_tests.rs`
  - Cover inheritance, project additions, project exclusions, global disabled filtering, and global/project target isolation.
- Modify: `docs/modules/tauri/core/projects/sync.md`
  - Document layered resolution and apply metadata.
- Modify: `src-tauri/src/core/scenes/manager.rs`
  - Stop using scene apply as a hidden agent-config rewrite path.
- Modify: `src-tauri/src/core/scenes/config.rs`
  - Keep loading `active_scene_id` for compatibility, but stop treating it as a resolution input.
- Modify: `src-tauri/src/commands/scenes.rs`
  - Keep CRUD commands. Reframe or remove direct `apply_scene` use from frontend after compatibility tests are added.
- Modify: `docs/modules/tauri/core/scenes/manager.md`
  - Document that scenes are toolkit definitions, not active global state.
- Modify: `src/lib/projects.ts`
  - Add `ProjectAgentAssignment` types and update Tauri command payloads.
- Modify: `src/lib/project-draft.ts`
  - Replace flat draft lists with per-agent project-layer drafts and effective preview helpers.
- Modify: `src/views/ProjectsView.tsx`
  - Orchestrate the new project-layer workbench without owning resolution rules.
- Modify: `src/components/projects/ProjectAssignmentEditor.tsx`
  - Edit per-agent project scenes/direct skills/exclusions.
- Modify: `src/components/projects/ProjectAssignmentSummary.tsx`
  - Show inherited global sources, project sources, exclusions, stale refs, disabled refs, and target status.
- Modify: `src/components/projects/ProjectCard.tsx`
  - Summarize per-agent scene/skill counts and apply status.
- Modify: `docs/modules/frontend/lib/projects.md`
- Modify: `docs/modules/frontend/lib/project-draft.md`
- Modify: `docs/modules/frontend/views/ProjectsView.md`
- Modify: `docs/modules/frontend/components/projects/ProjectAssignmentEditor.md`
- Modify: `docs/modules/frontend/components/projects/ProjectAssignmentSummary.md`
- Modify: `docs/modules/frontend/components/projects/ProjectCard.md`
  - Keep module docs one-to-one with changed frontend modules.
- Modify or add focused scripts under `scripts/`
  - Update project-draft and project-assignment layout tests if UI helper contracts change.

## Task 1: Shared Resolution Contract

**Files:**
- Modify: `src-tauri/src/core/agents/selection.rs`
- Modify: `docs/modules/tauri/core/agents/selection.md`
- Test: `src-tauri/src/core/agents/sync_tests.rs`

- [ ] **Step 1: Run impact checks for resolution symbols**

Run GitNexus impact analysis for:

```text
load_skill_selection_context
resolve_agent_skills
SkillSelectionContext
```

Expected: direct callers include agent sync tests and agent sync orchestration. If project or scene apply callers appear, include them in the task notes before editing.

- [ ] **Step 2: Add test fixture helpers for resolution tests**

In `src-tauri/src/core/agents/sync_tests.rs`, add this helper near the existing free helper functions if it does not already exist:

```rust
struct SyncFixture {
    config_dir: tempfile::TempDir,
    repo_dir: tempfile::TempDir,
    target_root: tempfile::TempDir,
}

impl SyncFixture {
    fn new() -> Self {
        Self {
            config_dir: tempfile::tempdir().unwrap(),
            repo_dir: tempfile::tempdir().unwrap(),
            target_root: tempfile::tempdir().unwrap(),
        }
    }

    fn create_skill(&self, relative_path: &str) {
        create_skill(self.repo_dir.path(), relative_path);
    }

    fn create_scene(&self, scene_id: &str, skill_ids: Vec<&str>) {
        let scene_store = SceneConfigStore::new(self.config_dir.path().to_path_buf());
        scene_store.create_scene(scene_id, scene_id, "").unwrap();
        scene_store
            .set_scene_skills(
                scene_id,
                skill_ids.into_iter().map(str::to_string).collect(),
            )
            .unwrap();
    }

    fn configure_agent(
        &self,
        key: &str,
        skill_ids: Vec<String>,
        scene_ids: Vec<String>,
        excluded_ids: Vec<String>,
    ) {
        let target_dir = self.target_root.path().join(format!("{key}-skills"));
        configure_agent(
            self.config_dir.path(),
            key,
            &target_dir,
            skill_ids.iter().map(String::as_str).collect(),
            scene_ids.iter().map(String::as_str).collect(),
            excluded_ids.iter().map(String::as_str).collect(),
        );
    }

    fn load_selection_context(&self) -> SkillSelectionContext {
        load_skill_selection_context(self.config_dir.path(), self.repo_dir.path()).unwrap()
    }

    fn agent_inventory_item(&self, key: &str) -> AgentInventoryItem {
        load_agent_inventory(self.config_dir.path(), &test_system_dirs(self.target_root.path()))
            .unwrap()
            .agents
            .into_iter()
            .find(|agent| agent.key == key)
            .unwrap()
    }

    fn resolve_agent(&self, key: &str) -> SkillResolutionResult {
        let context = self.load_selection_context();
        let agent = self.agent_inventory_item(key);
        resolve_agent_skill_selection(&agent, &context)
    }
}
```

If this helper needs imports, add only the specific imports required from `agents::discovery` and `agents::selection`.

- [ ] **Step 3: Add failing tests for diagnostics and global result shape**

In `src-tauri/src/core/agents/sync_tests.rs`, add tests that exercise the new result shape through the public sync path or a test-only helper:

```rust
#[test]
fn global_resolution_reports_missing_scene_references() {
    let fixture = SyncFixture::new();
    fixture.create_skill("custom/alpha");
    fixture.configure_agent(
        "codex",
        vec![],
        vec!["missing-scene".to_string()],
        vec![],
    );

    let context = fixture.load_selection_context();
    let agent = fixture.agent_inventory_item("codex");
    let result = resolve_agent_skill_selection(&agent, &context);

    assert!(result.skills.is_empty());
    assert_eq!(result.missing_scene_ids, vec!["missing-scene".to_string()]);
}
```

Also add:

```rust
#[test]
fn global_resolution_labels_direct_and_scene_sources() {
    let fixture = SyncFixture::new();
    fixture.create_skill("custom/direct");
    fixture.create_skill("custom/scene");
    fixture.create_scene("focus", vec!["custom:scene"]);
    fixture.configure_agent(
        "codex",
        vec!["custom:direct".to_string()],
        vec!["focus".to_string()],
        vec![],
    );

    let result = fixture.resolve_agent("codex");

    assert!(result.source_labels_for("custom:direct").contains("globalDirect"));
    assert!(result.source_labels_for("custom:scene").contains("globalScene"));
}
```

Expected: tests fail because `resolve_agent_skill_selection`, source labels, and missing-scene diagnostics do not exist yet.

- [ ] **Step 4: Replace bare vector resolution with a structured result**

In `selection.rs`, introduce:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SkillSourceLabel {
    GlobalDirect,
    GlobalScene { scene_id: String, scene_name: String },
    ProjectDirect,
    ProjectScene { scene_id: String, scene_name: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedSkillEntry {
    pub skill: SkillSummary,
    pub sources: Vec<SkillSourceLabel>,
    pub excluded: bool,
    pub globally_disabled: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct SkillResolutionDiagnostics {
    pub missing_scene_ids: Vec<String>,
    pub missing_skill_ids: Vec<String>,
    pub globally_disabled_references: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct SkillResolutionResult {
    pub entries: Vec<ResolvedSkillEntry>,
    pub diagnostics: SkillResolutionDiagnostics,
}
```

Add `resolve_agent_skill_selection(agent, context) -> SkillResolutionResult`. Keep `resolve_agent_skills(agent, context) -> Vec<SkillSummary>` as a compatibility wrapper that filters `result.entries` to non-excluded and non-disabled skills.

- [ ] **Step 5: Run focused Rust tests**

Run:

```powershell
cargo test --manifest-path src-tauri/Cargo.toml agents::sync_tests -- --nocapture
```

Expected: new and existing agent sync tests pass.

- [ ] **Step 6: Update module docs and commit**

Update `docs/modules/tauri/core/agents/selection.md` to describe:

- direct plus scene resolution
- source labels
- stale missing-scene diagnostics
- compatibility wrapper for old callers

Run:

```powershell
node scripts/check-module-doc-coverage.mjs
git add src-tauri/src/core/agents/selection.rs src-tauri/src/core/agents/sync_tests.rs docs/modules/tauri/core/agents/selection.md
git commit -m "refactor: structure skill resolution diagnostics"
```

Expected: module docs coverage passes and the commit includes only Task 1 files.

## Task 2: Project Per-Agent Store Migration

**Files:**
- Modify: `src-tauri/src/core/projects/store.rs`
- Modify: `src-tauri/src/core/projects/store_tests.rs`
- Modify: `docs/modules/tauri/core/projects/store.md`

- [ ] **Step 1: Run impact checks for project store symbols**

Run GitNexus impact analysis for:

```text
ProjectAssignment
ProjectConfigStore::load
ProjectConfigStore::add_project
ProjectConfigStore::update_project
```

Expected: affected callers include project commands, frontend DTOs, project sync, and project store/sync tests.

- [ ] **Step 2: Add failing migration and round-trip tests**

In `store_tests.rs`, add:

```rust
#[test]
fn loads_legacy_flat_project_as_per_agent_entries() {
    let temp = tempfile::tempdir().unwrap();
    let config_dir = temp.path().join("config");
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(
        config_dir.join("project-config.json"),
        r#"{
  "projects": {
    "C:/repo/app": {
      "projectPath": "C:/repo/app",
      "displayName": "App",
      "skillIds": ["custom:alpha"],
      "agentKeys": ["codex", "unknown-agent"]
    }
  }
}"#,
    ).unwrap();

    let snapshot = ProjectConfigStore::new(config_dir).load().unwrap();
    let project = snapshot.projects.get("C:/repo/app").unwrap();

    assert_eq!(project.agents["codex"].selected_skill_ids, vec!["custom:alpha"]);
    assert_eq!(project.agents["codex"].selected_scene_ids, Vec::<String>::new());
    assert_eq!(project.agents["codex"].excluded_skill_ids, Vec::<String>::new());
    assert_eq!(project.unsupported_agent_keys, vec!["unknown-agent"]);
}
```

Add a second test:

```rust
#[test]
fn saves_per_agent_project_entries_sorted_and_deduped() {
    let temp = tempfile::tempdir().unwrap();
    let store = ProjectConfigStore::new(temp.path().join("config"));

    store.add_project_with_agents(
        "C:/repo/app",
        "App",
        BTreeMap::from([(
            "codex".to_string(),
            ProjectAgentAssignment {
                selected_skill_ids: vec!["custom:beta".into(), "custom:alpha".into(), "custom:alpha".into()],
                selected_scene_ids: vec!["focus".into()],
                excluded_skill_ids: vec!["custom:beta".into()],
            },
        )]),
    ).unwrap();

    let entry = &store.load().unwrap().projects["C:/repo/app"].agents["codex"];
    assert_eq!(entry.selected_skill_ids, vec!["custom:alpha", "custom:beta"]);
    assert_eq!(entry.selected_scene_ids, vec!["focus"]);
    assert_eq!(entry.excluded_skill_ids, vec!["custom:beta"]);
}
```

Expected: tests fail because the per-agent shape and `add_project_with_agents` do not exist.

- [ ] **Step 3: Implement per-agent project model**

In `store.rs`, replace flat fields with:

```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectAgentAssignment {
    #[serde(default)]
    pub selected_skill_ids: Vec<String>,
    #[serde(default)]
    pub selected_scene_ids: Vec<String>,
    #[serde(default)]
    pub excluded_skill_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectAssignment {
    pub project_path: String,
    pub display_name: String,
    #[serde(default)]
    pub agents: BTreeMap<String, ProjectAgentAssignment>,
    #[serde(default)]
    pub unsupported_agent_keys: Vec<String>,
}
```

Use a custom deserialization helper or intermediate legacy struct so old JSON with `skillIds` and `agentKeys` migrates on load. Normalize all ID lists with the same sorted/deduped helper used by agent config.

- [ ] **Step 4: Keep compatibility command helpers**

Keep current `add_project(project_path, display_name, skill_ids, agent_keys)` and `update_project(... skill_ids, agent_keys)` as wrappers during migration. They should build per-agent entries by copying `skill_ids` into every supported `agent_key`. Add new methods:

```rust
pub fn add_project_with_agents(
    &self,
    project_path: &str,
    display_name: &str,
    agents: BTreeMap<String, ProjectAgentAssignment>,
) -> Result<ProjectConfigSnapshot>
```

and:

```rust
pub fn update_project_agents(
    &self,
    project_path: &str,
    display_name: Option<&str>,
    agents: Option<BTreeMap<String, ProjectAgentAssignment>>,
) -> Result<ProjectConfigSnapshot>
```

- [ ] **Step 5: Run focused store tests**

Run:

```powershell
cargo test --manifest-path src-tauri/Cargo.toml projects::store_tests -- --nocapture
```

Expected: all project store tests pass.

- [ ] **Step 6: Update docs and commit**

Update `docs/modules/tauri/core/projects/store.md` with the legacy-to-per-agent migration rule.

Run:

```powershell
node scripts/check-module-doc-coverage.mjs
git add src-tauri/src/core/projects/store.rs src-tauri/src/core/projects/store_tests.rs docs/modules/tauri/core/projects/store.md
git commit -m "refactor: store project assignments per agent"
```

## Task 3: Layered Project Sync

**Files:**
- Modify: `src-tauri/src/core/projects/sync.rs`
- Modify: `src-tauri/src/core/projects/sync_tests.rs`
- Modify: `src-tauri/src/core/settings.rs` only if existing sync-mode loading helper is not reusable
- Modify: `docs/modules/tauri/core/projects/sync.md`

- [ ] **Step 1: Run impact checks for project sync symbols**

Run GitNexus impact analysis for:

```text
apply_project_assignments
apply_for_project
select_project_skills
ProjectSyncLedger
```

Expected: affected callers include project commands, CLI project apply, and project sync tests.

- [ ] **Step 2: Add project sync fixture helpers**

In `src-tauri/src/core/projects/sync_tests.rs`, add this helper near the existing helper functions:

```rust
struct ProjectSyncFixture {
    temp: tempfile::TempDir,
    config_dir: std::path::PathBuf,
    repo_dir: std::path::PathBuf,
    project_dir: std::path::PathBuf,
}

impl ProjectSyncFixture {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let config_dir = temp.path().join("config");
        let repo_dir = temp.path().join("repo");
        let project_dir = temp.path().join("project");
        std::fs::create_dir_all(&project_dir).unwrap();
        Self {
            temp,
            config_dir,
            repo_dir,
            project_dir,
        }
    }

    fn create_skill(&self, relative_path: &str) -> String {
        create_skill(&self.repo_dir, relative_path, relative_path)
    }

    fn create_scene(&self, scene_id: &str, skill_ids: Vec<&str>) {
        let scene_store = SceneConfigStore::new(self.config_dir.clone());
        scene_store.create_scene(scene_id, scene_id, "").unwrap();
        scene_store
            .set_scene_skills(
                scene_id,
                skill_ids.into_iter().map(str::to_string).collect(),
            )
            .unwrap();
    }

    fn configure_global_agent(
        &self,
        agent_key: &str,
        skill_ids: Vec<&str>,
        scene_ids: Vec<&str>,
        excluded_ids: Vec<&str>,
    ) {
        AgentConfigStore::new(self.config_dir.clone())
            .set_agent_configuration(
                agent_key,
                true,
                Some(self.temp.path().join(format!("{agent_key}-global")).to_string_lossy().as_ref()),
                skill_ids.into_iter().map(str::to_string).collect(),
                scene_ids.into_iter().map(str::to_string).collect(),
                excluded_ids.into_iter().map(str::to_string).collect(),
            )
            .unwrap();
    }

    fn configure_project_agent(
        &self,
        agent_key: &str,
        skill_ids: Vec<&str>,
        scene_ids: Vec<&str>,
        excluded_ids: Vec<&str>,
    ) {
        let agents = BTreeMap::from([(
            agent_key.to_string(),
            ProjectAgentAssignment {
                selected_skill_ids: skill_ids.into_iter().map(str::to_string).collect(),
                selected_scene_ids: scene_ids.into_iter().map(str::to_string).collect(),
                excluded_skill_ids: excluded_ids.into_iter().map(str::to_string).collect(),
            },
        )]);
        let store = ProjectConfigStore::new(self.config_dir.clone());
        let project_path = self.project_dir.to_string_lossy();
        if store.load().unwrap().projects.contains_key(project_path.as_ref()) {
            store
                .update_project_agents(project_path.as_ref(), Some("Project"), Some(agents))
                .unwrap();
        } else {
            store
                .add_project_with_agents(project_path.as_ref(), "Project", agents)
                .unwrap();
        }
    }

    fn apply_projects(&self) -> ApplyProjectAssignmentsResponse {
        apply_project_assignments(
            &self.config_dir,
            &self.repo_dir,
            &AgentSystemDirs {
                home_dir: self.temp.path().join("home"),
                config_dir: Some(self.temp.path().join("xdg")),
            },
        )
        .unwrap()
    }

    fn project_target(&self, agent_key: &str) -> std::path::PathBuf {
        let agent = find_agent(agent_key).unwrap();
        self.project_dir.join(project_skills_dir_rule(agent))
    }

    fn global_target(&self, agent_key: &str) -> std::path::PathBuf {
        self.temp.path().join(format!("{agent_key}-global"))
    }
}
```

If exact project-local directories differ from `project_skills_dir_rule`, update `project_target` to call `find_agent` plus `project_skills_dir_rule` rather than hardcoding paths.

- [ ] **Step 3: Add failing inheritance tests**

In `sync_tests.rs`, add:

```rust
#[test]
fn project_apply_inherits_global_agent_scenes_and_adds_project_scene() {
    let fixture = ProjectSyncFixture::new();
    fixture.create_skill("custom/global");
    fixture.create_skill("custom/project");
    fixture.create_scene("global-scene", vec!["custom:global"]);
    fixture.create_scene("project-scene", vec!["custom:project"]);
    fixture.configure_global_agent("codex", vec![], vec!["global-scene"], vec![]);
    fixture.configure_project_agent(
        "codex",
        vec![],
        vec!["project-scene"],
        vec![],
    );

    fixture.apply_projects();

    assert!(fixture.project_target("codex").join("global/SKILL.md").exists());
    assert!(fixture.project_target("codex").join("project/SKILL.md").exists());
}
```

Add:

```rust
#[test]
fn project_exclusion_removes_inherited_skill_only_for_that_project() {
    let fixture = ProjectSyncFixture::new();
    fixture.create_skill("custom/global");
    fixture.configure_global_agent("codex", vec!["custom:global"], vec![], vec![]);
    fixture.configure_project_agent(
        "codex",
        vec![],
        vec![],
        vec!["custom:global"],
    );

    fixture.apply_global();
    fixture.apply_projects();

    assert!(fixture.global_target("codex").join("global/SKILL.md").exists());
    assert!(!fixture.project_target("codex").join("global").exists());
}
```

Expected: tests fail because project sync does not inherit global layer or support project exclusions.

- [ ] **Step 4: Add project-layer resolution helper**

In `selection.rs`, add:

```rust
pub(crate) fn resolve_project_agent_skill_selection(
    global_result: &SkillResolutionResult,
    project_agent: &ProjectAgentAssignment,
    context: &SkillSelectionContext,
) -> SkillResolutionResult
```

The helper must:

1. start with non-excluded, non-disabled global entries
2. add project direct skills with `ProjectDirect`
3. add project scene skills with `ProjectScene`
4. remove project exclusions from the final write set
5. keep diagnostics for missing project scenes and skills
6. keep global disabled references in diagnostics

- [ ] **Step 5: Use layered resolution in project sync**

In `projects/sync.rs`:

- load agent config and scene/skill context once
- for each project-agent entry, find the matching global agent entry
- compute global result with `resolve_agent_skill_selection`
- compute project result with `resolve_project_agent_skill_selection`
- build desired entries from final non-excluded entries
- write only to project-local `project_skills_dir_rule`

Replace hardcoded `SyncMode::Copy` with the configured `AgentSyncMode`, using the same mapping pattern as `scenes/manager.rs::load_sync_mode`.

- [ ] **Step 6: Run focused sync tests**

Run:

```powershell
cargo test --manifest-path src-tauri/Cargo.toml projects::sync_tests -- --nocapture
cargo test --manifest-path src-tauri/Cargo.toml agents::sync_tests -- --nocapture
```

Expected: project sync and global sync tests pass.

- [ ] **Step 7: Update docs and commit**

Update `docs/modules/tauri/core/projects/sync.md` with:

- inherited global layer
- project additions
- project-local exclusions
- configured copy/symlink mode
- unmanaged target preservation

Run:

```powershell
node scripts/check-module-doc-coverage.mjs
git add src-tauri/src/core/agents/selection.rs src-tauri/src/core/projects/sync.rs src-tauri/src/core/projects/sync_tests.rs docs/modules/tauri/core/projects/sync.md docs/modules/tauri/core/agents/selection.md
git commit -m "feat: resolve layered project skill assignments"
```

## Task 4: Scene Apply Semantics Cleanup

**Files:**
- Modify: `src-tauri/src/core/scenes/manager.rs`
- Modify: `src-tauri/src/core/scenes/config.rs`
- Modify: `src-tauri/src/commands/scenes.rs`
- Modify: `src/views/ScenesView.tsx`
- Modify: `src/components/scenes/SceneCard.tsx`
- Modify: scene module docs under `docs/modules/tauri/core/scenes/` and `docs/modules/frontend/components/scenes/SceneCard.md`

- [ ] **Step 1: Run impact checks for scene apply symbols**

Run GitNexus impact analysis for:

```text
apply_scene
apply_agent_state_for_scene
set_active_scene
SceneConfigSnapshot
```

Expected: affected callers include scene command wrappers, ScenesView, CLI scene commands, and scene tests.

- [ ] **Step 2: Add failing test that scene editing does not mutate agent config**

In `src-tauri/src/core/scenes/manager.rs` tests, replace old apply expectations with:

```rust
#[test]
fn scene_toolkit_apply_does_not_rewrite_agent_selection() {
    let temp = tempfile::tempdir().unwrap();
    let config_dir = temp.path().join("config");
    let repo_dir = temp.path().join("repo");

    create_skill(&repo_dir, "custom/alpha");
    SceneConfigStore::new(config_dir.clone())
        .create_scene("focus", "Focus", "")
        .unwrap();

    let agent_store = AgentConfigStore::new(config_dir.clone());
    agent_store
        .set_agent_selection(
            "codex",
            vec!["custom:existing".to_string()],
            vec![],
            vec![],
        )
        .unwrap();

    let before = agent_store.load().unwrap();
    let result = describe_scene_toolkit(&config_dir, &repo_dir, "focus").unwrap();
    let after = agent_store.load().unwrap();

    assert_eq!(result.scene_id, "focus");
    assert_eq!(before, after);
}
```

Expected: fails until the manager separates scene description/config from filesystem apply.

- [ ] **Step 3: Reframe scene manager**

Replace `apply_scene` with a non-mutating helper such as:

```rust
pub fn describe_scene_toolkit(
    config_dir: &Path,
    repo_path: &Path,
    scene_id: &str,
) -> Result<SceneToolkitSummary>
```

Keep the old Tauri command only if needed for backward compatibility, but do not call it from the frontend. If kept, return an error message that tells users to apply via Agents or Projects:

```rust
Err(anyhow::anyhow!(
    "Scenes are reusable toolkits. Apply them from Agents or Projects."
))
```

- [ ] **Step 4: Remove normal UI direct scene apply**

In `ScenesView.tsx` and `SceneCard.tsx`, remove or disable the Apply button. Replace it with neutral copy such as "Use this toolkit from Agents or Projects" through i18n keys in `src/i18n/en.json` and `src/i18n/zh.json`.

- [ ] **Step 5: Run focused tests**

Run:

```powershell
cargo test --manifest-path src-tauri/Cargo.toml scenes -- --nocapture
node --test scripts/agent-layout.test.mjs
npm run build
```

Expected: scene tests, focused layout test, and frontend build pass.

- [ ] **Step 6: Update docs and commit**

Update module docs for changed scene files and frontend component docs.

Run:

```powershell
node scripts/check-module-doc-coverage.mjs
git add src-tauri/src/core/scenes src-tauri/src/commands/scenes.rs src/views/ScenesView.tsx src/components/scenes/SceneCard.tsx src/i18n/en.json src/i18n/zh.json docs/modules
git commit -m "refactor: make scenes reusable toolkits"
```

## Task 5: Project UI Per-Agent Layer Editor

**Files:**
- Modify: `src/lib/projects.ts`
- Modify: `src/lib/project-draft.ts`
- Modify: `src/views/ProjectsView.tsx`
- Modify: `src/components/projects/ProjectAssignmentEditor.tsx`
- Modify: `src/components/projects/ProjectAssignmentSummary.tsx`
- Modify: `src/components/projects/ProjectCard.tsx`
- Modify: `src/i18n/en.json`
- Modify: `src/i18n/zh.json`
- Modify: corresponding frontend module docs
- Test: `scripts/project-draft.test.mjs`
- Test: `scripts/project-assignment-layout.test.mjs`

- [ ] **Step 1: Run impact checks for project frontend helpers**

Run GitNexus impact analysis for:

```text
ProjectDraft
buildProjectSummary
ProjectAssignmentEditor
ProjectAssignmentSummary
ProjectsView
```

Expected: affected callers are the Projects page, project-draft tests, and layout tests.

- [ ] **Step 2: Add project-draft test helpers**

In `scripts/project-draft.test.mjs`, add local helper builders near the other test helpers:

```js
function createProjectDraft() {
  return {
    mode: "create",
    sourceProjectPath: null,
    projectPath: "C:/repo/app",
    displayName: "App",
    displayNameManuallyEdited: true,
    agents: {},
    unsupportedAgentKeys: [],
  };
}

function skill(id, name) {
  return {
    id,
    name,
    description: "",
    sourceType: "custom",
    relativePath: id.replace(":", "/"),
    directoryPath: "",
    skillDocumentPath: "",
    managedSource: null,
  };
}

function agent(key) {
  return {
    key,
    displayName: key,
    installed: true,
    enabled: true,
    skillsDirRule: "",
    projectSkillsDirRule: ".codex/skills",
    selectedSkillIds: [],
    selectedSceneIds: [],
    excludedSkillIds: [],
    targetSkillEntries: [],
  };
}
```

- [ ] **Step 3: Add failing project-draft tests**

In `scripts/project-draft.test.mjs`, add tests for:

In `scripts/project-draft.test.mjs`, add tests for:

```js
test("project draft stores selected scenes per agent", () => {
  const draft = createProjectDraft();
  const next = toggleProjectAgentScene(draft, "codex", "obsidian-plugin-dev");
  assert.deepEqual(next.agents.codex.selectedSceneIds, ["obsidian-plugin-dev"]);
});

test("project summary marks inherited global skills and local exclusions", () => {
  const summary = buildProjectSummary({
    draft: toggleProjectAgentExclusion(createProjectDraft(), "codex", "custom:global"),
    globalPreview: {
      codex: {
        items: [
          {
            skill: skill("custom:global", "Global"),
            sourceLabels: ["globalDirect"],
            isExcludedByProject: false,
          },
        ],
      },
    },
    skills: [skill("custom:global", "Global")],
    scenes: {},
    agents: [agent("codex")],
    disabledSkillIds: [],
  });
  assert.equal(summary.previewItems[0].sourceLabels.includes("globalDirect"), true);
  assert.equal(summary.previewItems[0].isExcludedByProject, true);
});
```

Expected: fails until helper functions and summary shape are added.

- [ ] **Step 4: Update TypeScript DTOs**

In `src/lib/projects.ts`, define:

```ts
export interface ProjectAgentAssignment {
  selectedSkillIds: string[];
  selectedSceneIds: string[];
  excludedSkillIds: string[];
}

export interface ProjectAssignment {
  projectPath: string;
  displayName: string;
  agents: Record<string, ProjectAgentAssignment>;
  unsupportedAgentKeys: string[];
}
```

Keep legacy adapter helpers in `project-draft.ts` so existing config loaded from Rust can still be displayed during migration.

- [ ] **Step 5: Update project draft helpers**

In `project-draft.ts`, change `ProjectDraft` to:

```ts
export interface ProjectAgentDraft {
  selectedSkillIds: string[];
  selectedSceneIds: string[];
  excludedSkillIds: string[];
}

export interface ProjectDraft {
  mode: "create" | "edit";
  sourceProjectPath: string | null;
  projectPath: string;
  displayName: string;
  displayNameManuallyEdited: boolean;
  agents: Record<string, ProjectAgentDraft>;
  unsupportedAgentKeys: string[];
}
```

Add helpers:

```ts
export function ensureProjectAgentDraft(draft: ProjectDraft, agentKey: string): ProjectDraft
export function toggleProjectAgentSkill(draft: ProjectDraft, agentKey: string, skillId: string): ProjectDraft
export function toggleProjectAgentScene(draft: ProjectDraft, agentKey: string, sceneId: string): ProjectDraft
export function toggleProjectAgentExclusion(draft: ProjectDraft, agentKey: string, skillId: string): ProjectDraft
```

- [ ] **Step 6: Update editor UI**

`ProjectAssignmentEditor` should show a selected-agent list and edit one agent's project layer at a time:

- agent selector column
- direct skills checklist for selected project agent
- scene checklist for selected project agent
- exclusion checklist sourced from inherited global preview plus project-added skills

Use existing compact panels and no nested cards.

- [ ] **Step 7: Update summary UI**

`ProjectAssignmentSummary` should show per-agent preview groups:

- inherited global sources
- project scene sources
- project direct sources
- project exclusions
- globally disabled references
- stale scenes/skills
- expected project target path

- [ ] **Step 8: Run frontend focused checks**

Run:

```powershell
node --test scripts/project-draft.test.mjs
node --test scripts/project-assignment-layout.test.mjs
npm run build
```

Expected: project helper tests, layout tests, and build pass.

- [ ] **Step 9: Update frontend module docs and commit**

Run:

```powershell
node scripts/check-module-doc-coverage.mjs
git add src/lib/projects.ts src/lib/project-draft.ts src/views/ProjectsView.tsx src/components/projects src/i18n/en.json src/i18n/zh.json scripts/project-draft.test.mjs scripts/project-assignment-layout.test.mjs docs/modules/frontend
git commit -m "feat: edit project toolkit layers per agent"
```

## Task 6: Apply Status Metadata

**Files:**
- Modify: `src-tauri/src/core/projects/sync.rs`
- Modify: `src-tauri/src/core/agents/manifest.rs` or `src-tauri/src/core/agents/target_manifest.rs` only if reusing global target metadata belongs there
- Modify: `src-tauri/src/core/projects/sync_tests.rs`
- Modify: `src/lib/projects.ts`
- Modify: `src/components/projects/ProjectCard.tsx`
- Modify: project sync and frontend module docs

- [ ] **Step 1: Add failing stale/current tests**

In `projects/sync_tests.rs`, add:

```rust
#[test]
fn project_apply_marks_current_then_stale_after_config_change() {
    let fixture = ProjectSyncFixture::new();
    fixture.create_skill("custom/alpha");
    fixture.configure_project_agent("codex", vec!["custom:alpha"], vec![], vec![]);

    let first = fixture.apply_projects();
    assert_eq!(first.project_status("C:/repo/app", "codex"), ApplyFreshness::Current);

    fixture.create_skill("custom/beta");
    fixture.configure_project_agent("codex", vec!["custom:alpha", "custom:beta"], vec![], vec![]);

    let preview = fixture.inspect_project_status();
    assert_eq!(preview.project_status("C:/repo/app", "codex"), ApplyFreshness::Stale);
}
```

Expected: fails until the ledger stores and compares resolution hashes.

- [ ] **Step 2: Add target-scoped resolution hash**

In project sync ledger entries, add:

```rust
resolution_hash: String,
applied_at: i64,
```

Build the hash from stable JSON containing:

- project path
- agent key
- resolved skill IDs
- source labels
- exclusion IDs
- target directory
- sync mode

- [ ] **Step 3: Surface apply status to frontend**

Extend `ProjectAssignmentApplyResult` and project config/inspection DTOs with:

```ts
applyStatus: "current" | "stale" | "neverApplied";
lastAppliedAt: number | null;
```

Keep missing ledger entries as `neverApplied`.

- [ ] **Step 4: Update ProjectCard**

Show one compact status row per configured project agent:

- `current`
- `stale`
- `never applied`
- `unsupported`

Do not imply that viewing or editing a project applies it.

- [ ] **Step 5: Run verification for status slice**

Run:

```powershell
cargo test --manifest-path src-tauri/Cargo.toml projects::sync_tests -- --nocapture
node --test scripts/project-draft.test.mjs
npm run build
```

Expected: all pass.

- [ ] **Step 6: Update docs and commit**

Run:

```powershell
node scripts/check-module-doc-coverage.mjs
git add src-tauri/src/core/projects/sync.rs src-tauri/src/core/projects/sync_tests.rs src/lib/projects.ts src/components/projects/ProjectCard.tsx docs/modules
git commit -m "feat: track project apply freshness"
```

## Task 7: CLI, Docs, And Full Verification

**Files:**
- Modify: `src-tauri/src/cli/commands/projects.rs`
- Modify: `src-tauri/src/cli/commands/projects_tests.rs`
- Modify: `src-tauri/src/cli/commands/scenes.rs`
- Modify: `src-tauri/src/cli/commands/scenes_tests.rs`
- Modify: CLI module docs under `docs/modules/tauri/cli/commands/`
- Modify: command docs under `docs/modules/tauri/commands/`
- Modify: `AGENTS.md` only if the project goal text needs to reflect layered toolkit scenes

- [ ] **Step 1: Add CLI regression tests**

Add tests that assert:

- `projects apply` remains full-snapshot with no path argument unless a separate scoped command already exists
- project config output includes per-agent scene/direct/exclusion fields
- old scene apply wording does not promise global active-scene switching

Use existing CLI parser test style in `projects_tests.rs` and `scenes_tests.rs`.

- [ ] **Step 2: Update CLI command behavior and docs**

Make CLI project mutation commands accept the per-agent fields. Preserve old flat arguments only if existing tests require compatibility; map them to per-agent entries exactly like the Rust store wrappers.

- [ ] **Step 3: Run full verification gate**

Run:

```powershell
npm run check:module-docs
npm run check:architecture
npm run test:project-draft
npm run test:project-assignment-layout
npm run build
cargo check --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml
npm run verify
```

Expected: every command exits 0. If `npm run verify` repeats known warnings, capture them in the final task notes but do not ignore failures.

- [ ] **Step 4: Commit final integration**

Run:

```powershell
git status --short
git add src-tauri/src/cli docs/modules AGENTS.md
git commit -m "chore: align cli docs with layered toolkit scenes"
```

Only include `AGENTS.md` if it was actually changed.

## Self-Review Checklist

- [ ] Spec coverage: every product rule in `docs/superpowers/specs/2026-05-06-layered-toolkit-scenes-design.md` maps to a task above.
- [ ] Review findings coverage: OpenCode F1 maps to Task 2; F2/F3/F6 map to Task 4; F4/F5 map to Tasks 1 and 3; F7 maps to Task 6; F8 maps to Task 3; F9 is treated as acceptance criteria across tasks.
- [ ] No source file is added without a matching module doc.
- [ ] No command mutates target directories except explicit apply paths.
- [ ] Project exclusions stay project-local.
- [ ] Existing unmanaged target content remains protected by target manifests.
- [ ] Final verification includes `npm run verify`.

## Execution Choice

Plan complete and saved to `docs/superpowers/plans/2026-05-06-layered-toolkit-scenes.md`. Two execution options:

1. **Subagent-Driven (recommended)** - dispatch a fresh subagent per task, review between tasks, fast iteration.
2. **Inline Execution** - execute tasks in this session using executing-plans, batch execution with checkpoints.

Which approach?

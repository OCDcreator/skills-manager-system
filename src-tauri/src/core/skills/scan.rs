use anyhow::{anyhow, Result};
use serde::Serialize;
use std::fs;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

use crate::core::external_sources::{
    ExternalSourcesStore, ImportedExternalSkillRecord, ManagedSkillMirrorManifest,
};

use super::identity::{
    build_skill_id_from_relative_path, canonicalize_repo_relative_path, resolve_source_type,
};
use super::metadata::parse_skill_metadata;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ManagedSourceInfo {
    pub kind: String,
    pub import_id: String,
    pub repo_url: String,
    pub pinned_commit: String,
    pub agent_key: String,
    pub update_available: bool,
    pub integrity: Option<String>,
}

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
    pub managed_source: Option<ManagedSourceInfo>,
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

            skills.push(build_skill_summary(repo_root, &skill_dir)?);
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
                let skill_dir = entry.path().parent().expect("SKILL.md must have a parent");
                skills.push(build_skill_summary(repo_root, skill_dir)?);
            }
        }
    } else {
        warnings
            .push("Missing external/ directory; continuing with remaining sources.".to_string());
    }

    skills.sort_by(|left, right| left.id.cmp(&right.id));

    Ok(ScanSkillsResponse { skills, warnings })
}

pub fn scan_repo_skills_with_external_sources(
    repo_root: &Path,
    config_dir: &Path,
) -> Result<ScanSkillsResponse> {
    let mut response = scan_repo_skills(repo_root)?;
    let snapshot = ExternalSourcesStore::new(config_dir.to_path_buf()).load()?;

    for skill in &mut response.skills {
        if skill.source_type != "external" {
            continue;
        }

        let manifest_path = Path::new(&skill.directory_path).join(".skills-manager-source.json");
        if !manifest_path.exists() {
            continue;
        }

        match read_managed_manifest(&manifest_path) {
            Ok(manifest) => {
                let managed_source = build_managed_source_info(
                    skill,
                    &manifest,
                    snapshot
                        .imports
                        .iter()
                        .find(|record| record.skill_id == skill.id),
                );
                skill.managed_source = Some(managed_source);
            }
            Err(error) => {
                response.warnings.push(format!(
                    "Failed to parse managed mirror manifest at {}: {error}",
                    manifest_path.display()
                ));
            }
        }
    }

    Ok(response)
}

fn should_walk(entry: &DirEntry) -> bool {
    !IGNORED_DIRS
        .iter()
        .any(|ignored| entry.file_name() == *ignored)
}

fn build_skill_summary(repo_root: &Path, skill_dir: &Path) -> Result<SkillSummary> {
    let relative_path = normalize_relative_path(repo_root, skill_dir)?;
    let skill_md_path = skill_dir.join("SKILL.md");
    let metadata = parse_skill_metadata(&skill_md_path)?;
    let source_type = resolve_source_type(&relative_path)?;
    let default_name = skill_dir
        .file_name()
        .map(|value| value.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown-skill".to_string());

    Ok(SkillSummary {
        id: build_skill_id_from_relative_path(&relative_path)?,
        name: metadata.name.unwrap_or(default_name),
        description: metadata.description.unwrap_or_default(),
        source_type: source_type.to_string(),
        relative_path,
        directory_path: path_to_string(skill_dir),
        skill_document_path: path_to_string(&skill_md_path),
        managed_source: None,
    })
}

fn normalize_relative_path(repo_root: &Path, skill_dir: &Path) -> Result<String> {
    let relative = skill_dir.strip_prefix(repo_root)?;
    canonicalize_repo_relative_path(&path_to_string(relative))
}

fn path_to_string(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn read_managed_manifest(path: &Path) -> Result<ManagedSkillMirrorManifest> {
    let raw = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&raw)?)
}

fn build_managed_source_info(
    skill: &SkillSummary,
    manifest: &ManagedSkillMirrorManifest,
    import_record: Option<&ImportedExternalSkillRecord>,
) -> ManagedSourceInfo {
    let Some(import_record) = import_record else {
        return mismatch_managed_source(manifest);
    };

    if !manifest.managed
        || manifest.skill_id != skill.id
        || manifest.mirror_relative_path != skill.relative_path
        || manifest.import_id != import_record.import_id
        || manifest.source_id != import_record.external_source_id
        || manifest.agent_key != import_record.agent_key
        || manifest.variant_path != import_record.upstream_variant_path
        || manifest.pinned_commit != import_record.pinned_commit
    {
        return mismatch_managed_source(manifest);
    }

    ManagedSourceInfo {
        kind: "github_import".to_string(),
        import_id: import_record.import_id.clone(),
        repo_url: manifest.repo_url.clone(),
        pinned_commit: import_record.pinned_commit.clone(),
        agent_key: import_record.agent_key.clone(),
        update_available: import_record.update_available,
        integrity: None,
    }
}

fn mismatch_managed_source(manifest: &ManagedSkillMirrorManifest) -> ManagedSourceInfo {
    ManagedSourceInfo {
        kind: "github_import".to_string(),
        import_id: manifest.import_id.clone(),
        repo_url: manifest.repo_url.clone(),
        pinned_commit: manifest.pinned_commit.clone(),
        agent_key: manifest.agent_key.clone(),
        update_available: false,
        integrity: Some("mismatch".to_string()),
    }
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
        fs::write(
            repo.path().join("custom/searxng/SKILL.md"),
            "---\nname: searxng\n---",
        )
        .unwrap();
        fs::write(
            repo.path().join("custom/group/sub-skill/SKILL.md"),
            "---\nname: nested\n---",
        )
        .unwrap();

        let response = scan_repo_skills(repo.path()).unwrap();
        let ids: Vec<_> = response
            .skills
            .iter()
            .map(|skill| skill.id.as_str())
            .collect();

        assert!(ids.contains(&"custom:searxng"));
        assert!(!ids.contains(&"custom:group/sub-skill"));
    }

    #[test]
    fn scan_external_recurses_and_ignores_reference_only_sources() {
        let repo = tempdir().unwrap();
        fs::create_dir_all(repo.path().join("external/html-ppt-skill")).unwrap();
        fs::create_dir_all(repo.path().join("external/awesome-design-md/apple")).unwrap();
        fs::write(
            repo.path().join("external/html-ppt-skill/SKILL.md"),
            "---\nname: html-ppt\n---",
        )
        .unwrap();
        fs::write(
            repo.path()
                .join("external/awesome-design-md/apple/README.md"),
            "# reference",
        )
        .unwrap();

        let response = scan_repo_skills(repo.path()).unwrap();
        let ids: Vec<_> = response
            .skills
            .iter()
            .map(|skill| skill.id.as_str())
            .collect();

        assert!(ids.contains(&"external:html-ppt-skill"));
        assert!(!ids.iter().any(|id| id.contains("awesome-design-md")));
    }

    #[test]
    fn scan_ignores_noise_directories() {
        let repo = tempdir().unwrap();
        fs::create_dir_all(repo.path().join("external/node_modules/pkg")).unwrap();
        fs::create_dir_all(
            repo.path()
                .join("external/anthropics-skills/frontend-design"),
        )
        .unwrap();
        fs::write(
            repo.path().join("external/node_modules/pkg/SKILL.md"),
            "---\nname: nope\n---",
        )
        .unwrap();
        fs::write(
            repo.path()
                .join("external/anthropics-skills/frontend-design/SKILL.md"),
            "---\nname: frontend-design\n---",
        )
        .unwrap();

        let response = scan_repo_skills(repo.path()).unwrap();
        let ids: Vec<_> = response
            .skills
            .iter()
            .map(|skill| skill.id.as_str())
            .collect();

        assert!(!ids.iter().any(|id| id.contains("node_modules")));
        assert!(ids.contains(&"external:anthropics-skills/frontend-design"));
    }

    #[test]
    fn scan_external_managed_path_keeps_external_source_type() {
        let repo = tempdir().unwrap();
        fs::create_dir_all(repo.path().join("external/managed/github/owner__repo/codex/skill"))
            .unwrap();
        fs::write(
            repo.path()
                .join("external/managed/github/owner__repo/codex/skill/SKILL.md"),
            "---\nname: managed-skill\n---",
        )
        .unwrap();

        let response = scan_repo_skills(repo.path()).unwrap();
        let skill = response
            .skills
            .iter()
            .find(|skill| skill.relative_path == "external/managed/github/owner__repo/codex/skill")
            .unwrap();

        assert_eq!(skill.source_type, "external");
        assert_eq!(skill.id, "external:managed/github/owner__repo/codex/skill");
    }

    #[test]
    fn scan_enriches_managed_external_skill_when_manifest_and_record_match() {
        use crate::app_runtime::config_lock::acquire_config_lock;
        use crate::core::external_sources::models::{
            ExternalSourceRecord, ExternalSourcesSnapshot, ImportedExternalSkillRecord,
            ManagedSkillMirrorManifest,
        };

        let root = tempdir().unwrap();
        let repo_root = root.path().join("repo");
        let config_dir = root.path().join("config");
        let skill_relative_path = "external/managed/github/owner__repo/codex/impeccable";
        let skill_dir = repo_root.join(skill_relative_path);
        fs::create_dir_all(&skill_dir).unwrap();
        fs::write(
            skill_dir.join("SKILL.md"),
            "---\nname: impeccable\ndescription: imported\n---\n# Managed\n",
        )
        .unwrap();
        fs::write(
            skill_dir.join(".skills-manager-source.json"),
            serde_json::to_string_pretty(&ManagedSkillMirrorManifest {
                schema_version: 1,
                managed: true,
                import_id: "imp_01".to_string(),
                source_id: "src_01".to_string(),
                repo_url: "git@github.com:owner/repo.git".to_string(),
                agent_key: "codex".to_string(),
                variant_path: "dist/agents/.agents/skills/impeccable".to_string(),
                mirror_relative_path: skill_relative_path.to_string(),
                skill_id: "external:managed/github/owner__repo/codex/impeccable".to_string(),
                pinned_commit: "abc123".to_string(),
            })
            .unwrap(),
        )
        .unwrap();

        let store = ExternalSourcesStore::new(config_dir.clone());
        let guard = acquire_config_lock(&config_dir).unwrap();
        store
            .save(
                &guard,
                &ExternalSourcesSnapshot {
                    schema_version: 1,
                    sources: vec![ExternalSourceRecord {
                        id: "src_01".to_string(),
                        repo_url: "git@github.com:owner/repo.git".to_string(),
                        ..ExternalSourceRecord::default()
                    }],
                    imports: vec![ImportedExternalSkillRecord {
                        import_id: "imp_01".to_string(),
                        external_source_id: "src_01".to_string(),
                        agent_key: "codex".to_string(),
                        upstream_variant_path: "dist/agents/.agents/skills/impeccable"
                            .to_string(),
                        pinned_commit: "abc123".to_string(),
                        pinned_variant_fingerprint: Some("sha256:1234".to_string()),
                        skill_id: "external:managed/github/owner__repo/codex/impeccable"
                            .to_string(),
                        mirror_relative_path: skill_relative_path.to_string(),
                        last_checked_commit: Some("def456".to_string()),
                        imported_at: Some("2026-04-29T00:00:00Z".to_string()),
                        warnings: Vec::new(),
                        update_available: true,
                    }],
                },
            )
            .unwrap();
        drop(guard);

        let response = scan_repo_skills_with_external_sources(&repo_root, &config_dir).unwrap();
        let skill = response
            .skills
            .iter()
            .find(|skill| skill.id == "external:managed/github/owner__repo/codex/impeccable")
            .unwrap();

        assert_eq!(
            skill.managed_source,
            Some(ManagedSourceInfo {
                kind: "github_import".to_string(),
                import_id: "imp_01".to_string(),
                repo_url: "git@github.com:owner/repo.git".to_string(),
                pinned_commit: "abc123".to_string(),
                agent_key: "codex".to_string(),
                update_available: true,
                integrity: None,
            })
        );
    }

    #[test]
    fn scan_keeps_managed_enrichment_healthy_without_source_snapshot_entry() {
        use crate::app_runtime::config_lock::acquire_config_lock;
        use crate::core::external_sources::models::{
            ExternalSourcesSnapshot, ImportedExternalSkillRecord, ManagedSkillMirrorManifest,
        };

        let root = tempdir().unwrap();
        let repo_root = root.path().join("repo");
        let config_dir = root.path().join("config");
        let skill_relative_path = "external/managed/github/owner__repo/codex/impeccable";
        let skill_dir = repo_root.join(skill_relative_path);
        fs::create_dir_all(&skill_dir).unwrap();
        fs::write(
            skill_dir.join("SKILL.md"),
            "---\nname: impeccable\ndescription: imported\n---\n# Managed\n",
        )
        .unwrap();
        fs::write(
            skill_dir.join(".skills-manager-source.json"),
            serde_json::to_string_pretty(&ManagedSkillMirrorManifest {
                schema_version: 1,
                managed: true,
                import_id: "imp_01".to_string(),
                source_id: "src_01".to_string(),
                repo_url: "git@github.com:owner/repo.git".to_string(),
                agent_key: "codex".to_string(),
                variant_path: "dist/agents/.agents/skills/impeccable".to_string(),
                mirror_relative_path: skill_relative_path.to_string(),
                skill_id: "external:managed/github/owner__repo/codex/impeccable".to_string(),
                pinned_commit: "abc123".to_string(),
            })
            .unwrap(),
        )
        .unwrap();

        let store = ExternalSourcesStore::new(config_dir.clone());
        let guard = acquire_config_lock(&config_dir).unwrap();
        store
            .save(
                &guard,
                &ExternalSourcesSnapshot {
                    schema_version: 1,
                    sources: Vec::new(),
                    imports: vec![ImportedExternalSkillRecord {
                        import_id: "imp_01".to_string(),
                        external_source_id: "src_01".to_string(),
                        agent_key: "codex".to_string(),
                        upstream_variant_path: "dist/agents/.agents/skills/impeccable"
                            .to_string(),
                        pinned_commit: "abc123".to_string(),
                        pinned_variant_fingerprint: Some("sha256:1234".to_string()),
                        skill_id: "external:managed/github/owner__repo/codex/impeccable"
                            .to_string(),
                        mirror_relative_path: skill_relative_path.to_string(),
                        last_checked_commit: Some("def456".to_string()),
                        imported_at: Some("2026-04-29T00:00:00Z".to_string()),
                        warnings: Vec::new(),
                        update_available: true,
                    }],
                },
            )
            .unwrap();
        drop(guard);

        let response = scan_repo_skills_with_external_sources(&repo_root, &config_dir).unwrap();
        let skill = response
            .skills
            .iter()
            .find(|skill| skill.id == "external:managed/github/owner__repo/codex/impeccable")
            .unwrap();

        assert_eq!(
            skill.managed_source,
            Some(ManagedSourceInfo {
                kind: "github_import".to_string(),
                import_id: "imp_01".to_string(),
                repo_url: "git@github.com:owner/repo.git".to_string(),
                pinned_commit: "abc123".to_string(),
                agent_key: "codex".to_string(),
                update_available: true,
                integrity: None,
            })
        );
    }
}

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

const APP_ID: &str = "skills-manager-system";
const MANIFEST_FILE_NAME: &str = ".skills-manager-system-manifest.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AgentTargetManifest {
    pub entries: BTreeMap<String, AgentTargetManifestEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AgentTargetManifestEntry {
    pub skill_id: String,
    pub relative_path: String,
    #[serde(default)]
    pub preserve_existing: bool,
}

pub(crate) fn default_manifest() -> AgentTargetManifest {
    AgentTargetManifest {
        entries: BTreeMap::new(),
    }
}

pub(crate) fn load_manifest(target_dir: &Path, agent_key: &str) -> Result<AgentTargetManifest> {
    let path = manifest_path(target_dir);
    if !path.exists() {
        return Ok(default_manifest());
    }

    let raw = fs::read_to_string(&path).with_context(|| format!("Failed to read {:?}", path))?;
    let manifest = serde_json::from_str::<SerializedAgentTargetManifest>(&raw)
        .with_context(|| format!("Failed to parse {:?}", path))?;

    if manifest.app_id != APP_ID {
        return Err(anyhow!(
            "Managed manifest at {:?} belongs to a different app",
            path
        ));
    }
    if manifest.agent_key != agent_key {
        return Err(anyhow!(
            "Managed manifest at {:?} belongs to a different agent",
            path
        ));
    }

    Ok(AgentTargetManifest {
        entries: manifest.entries,
    })
}

pub(crate) fn save_manifest(
    target_dir: &Path,
    agent_key: &str,
    manifest: &AgentTargetManifest,
) -> Result<()> {
    let json = serde_json::to_string_pretty(&SerializedAgentTargetManifest {
        app_id: APP_ID.to_string(),
        agent_key: agent_key.to_string(),
        entries: manifest.entries.clone(),
    })?;
    fs::write(manifest_path(target_dir), json).context("Failed to write target manifest")?;
    Ok(())
}

pub(crate) fn save_or_remove_manifest(
    target_dir: &Path,
    agent_key: &str,
    manifest: &AgentTargetManifest,
) -> Result<()> {
    if manifest.entries.is_empty() {
        remove_manifest(target_dir)
    } else {
        save_manifest(target_dir, agent_key, manifest)
    }
}

pub(crate) fn remove_manifest(target_dir: &Path) -> Result<()> {
    let path = manifest_path(target_dir);
    if path.exists() {
        fs::remove_file(&path).with_context(|| format!("Failed to remove {:?}", path))?;
    }
    Ok(())
}

pub(crate) fn is_manifest_file_name(file_name: &std::ffi::OsStr) -> bool {
    file_name == MANIFEST_FILE_NAME
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct SerializedAgentTargetManifest {
    app_id: String,
    agent_key: String,
    entries: BTreeMap<String, AgentTargetManifestEntry>,
}

fn manifest_path(target_dir: &Path) -> PathBuf {
    target_dir.join(MANIFEST_FILE_NAME)
}

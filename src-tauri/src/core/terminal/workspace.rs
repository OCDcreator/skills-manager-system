use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::app_runtime::normalize_output_path;
use crate::core::settings::{AppSettings, SettingsStore};

use super::model::TerminalLauncherPreferences;

const CONFIG_GUIDE_FILE_NAME: &str = "agents.md";

const TERMINAL_AGENT_ROOTS: [(&str, &str); 4] = [
    ("codex", ".codex/skills"),
    ("claude_code", ".claude/skills"),
    ("opencode", ".opencode/skills"),
    ("kimi", ".kimi/skills"),
];

const CONFIG_GUIDE_SKILL_NAME: &str = "my-skills-repo-management";
const EXTERNAL_SOURCE_SKILL_NAME: &str = "external-source-skill-intake";

const CONFIG_GUIDE_CONTENT: &str = r#"# Skills Manager Config Workspace

This directory is the default working directory for the in-app assistant terminals.

## What lives here

- `settings.json`: the app settings file.
- `agents.md`: this quick guide for the config workspace.
- `.codex/skills`, `.claude/skills`, `.opencode/skills`, `.kimi/skills`: local project-style skill folders used when an assistant terminal runs from this directory.

## How to determine the local my-skills repository

1. Check the app Settings page first. The repository field is the source of truth.
2. If you need the raw value, open `settings.json` in this directory and inspect `repoPath`.
3. If `repoPath` is empty, the app falls back to the user's default local repo choice. On this machine that is usually `C:/Users/lt/Desktop/Write/custom-project/my-skills`, but the app setting wins whenever it is present.

## How to use this workspace

- Start Codex, Claude Code, OpenCode, or Kimi from this directory if you want them to read the built-in guidance skills below.
- Treat the hidden agent folders here as project-local config overlays, not as the user's global home-directory config.
- Keep skill-repo decisions aligned with the app settings and the actual `my-skills` checkout.
"#;

const CONFIG_GUIDE_SKILL_CONTENT: &str = r#"---
name: my-skills-repo-management
description: Explain how this Skills Manager System workspace determines and manages the local my-skills repository. Use this whenever the user asks where the managed my-skills repo is, how the repo path is configured, how custom versus external skills are organized, or how to maintain the repository used by this app.
---

# My Skills Repository Management

Use this skill when the user needs guidance about the `my-skills` repository managed by Skills Manager System.

## Response goals

- Explain the current repository location in a verifiable way.
- Distinguish configured state from machine-default assumptions.
- Keep answers aligned with this app's config workspace and `settings.json`.

## Workflow

1. Start with the app setting. The repository path configured in the Settings page is the primary source of truth.
2. If you can inspect files, read `settings.json` from the current config workspace and look for `repoPath`.
3. If `repoPath` is missing or empty, state that the app will fall back to the user's default local repo choice instead of pretending a custom path is configured.
4. When describing repository contents, explain the expected split:
   - `custom/` contains first-party skills maintained by the user.
   - `external/` contains mirrored or imported third-party sources.
   - helper scripts such as `update.*`, `pull.*`, and `push.*` operate on the repo as a whole.
5. If the user asks how to manage the repo, prefer practical guidance:
   - confirm the configured path
   - explain whether the task belongs in `custom/` or `external/`
   - mention Git sync expectations when relevant

## Guardrails

- Do not assume the default path is active unless the configured path is absent.
- Do not describe external-source mirrors as if they were custom first-party skills.
- If you are unsure whether the current workspace points at the right repo, say what should be verified.
"#;

const EXTERNAL_SOURCE_SKILL_CONTENT: &str = r#"---
name: external-source-skill-intake
description: Evaluate whether an external source should be managed as skills inside the my-skills repository. Use this whenever the user wants to add, mirror, import, refresh, or review an external source. Always determine whether the candidate content is actually a skill or a skill collection before recommending management.
---

# External Source Skill Intake

Use this skill when the user wants to bring a third-party source into the `my-skills` workflow.

## Core rule

First determine whether the candidate is actually a skill or a collection of skills. Manage it only if the answer is yes. If it is not a skill source, do not manage it as one.

## Intake workflow

1. Inspect the source structure before recommending import or sync.
2. Look for concrete skill evidence such as directories whose root contains `SKILL.md`.
3. Distinguish between:
   - actual skill directories
   - generic docs, prompts, notes, demos, templates, or unrelated code
4. If the source is a skill collection, explain how it should live under `external/` and which subset is relevant.
5. If the source is not a skill collection, say clearly that it should not be managed by Skills Manager System as a skill source.

## Decision rules

- Manage: repositories or folders that contain one or more real skills with usable `SKILL.md` roots.
- Do not manage: repos that only contain docs, loose prompts, articles, examples, or tooling without actual skill units.
- Mixed sources: only manage the parts that are real skills; exclude the rest from the recommendation.

## Response style

- Give the conclusion first: skill source or not.
- Briefly cite the evidence that led to the conclusion.
- If the answer is yes, describe the expected next step for importing or mirroring it.
- If the answer is no, explain why it falls outside skill management.
"#;

pub fn ensure_terminal_workspace(config_dir: &Path) -> Result<()> {
    fs::create_dir_all(config_dir)
        .with_context(|| format!("Failed to create {:?}", config_dir))?;

    write_text_if_changed(
        &config_dir.join(CONFIG_GUIDE_FILE_NAME),
        CONFIG_GUIDE_CONTENT,
    )?;

    for (_, skills_dir_rule) in TERMINAL_AGENT_ROOTS {
        let skills_dir = config_dir.join(skills_dir_rule);
        fs::create_dir_all(&skills_dir)
            .with_context(|| format!("Failed to create {:?}", skills_dir))?;
        write_skill_bundle(
            &skills_dir,
            CONFIG_GUIDE_SKILL_NAME,
            CONFIG_GUIDE_SKILL_CONTENT,
        )?;
        write_skill_bundle(
            &skills_dir,
            EXTERNAL_SOURCE_SKILL_NAME,
            EXTERNAL_SOURCE_SKILL_CONTENT,
        )?;
    }

    Ok(())
}

pub fn terminal_launcher_preferences(
    config_dir: &Path,
    settings: &AppSettings,
) -> TerminalLauncherPreferences {
    let default_working_directory = normalize_output_path(config_dir);
    let working_directory = settings
        .assistant_working_directory
        .clone()
        .filter(|path| !path.trim().is_empty())
        .unwrap_or_else(|| default_working_directory.clone());

    TerminalLauncherPreferences {
        default_working_directory,
        working_directory,
    }
}

pub fn load_terminal_launcher_preferences(
    config_dir: &Path,
) -> Result<TerminalLauncherPreferences> {
    ensure_terminal_workspace(config_dir)?;
    let settings = SettingsStore::new(config_dir.to_path_buf()).load()?;
    Ok(terminal_launcher_preferences(config_dir, &settings))
}

fn write_skill_bundle(skills_dir: &Path, skill_name: &str, content: &str) -> Result<()> {
    let skill_dir = skills_dir.join(skill_name);
    fs::create_dir_all(&skill_dir).with_context(|| format!("Failed to create {:?}", skill_dir))?;
    write_text_if_changed(&skill_dir.join("SKILL.md"), content)
}

fn write_text_if_changed(path: &Path, content: &str) -> Result<()> {
    match fs::read_to_string(path) {
        Ok(existing) if existing == content => return Ok(()),
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(error).with_context(|| format!("Failed to read {:?}", path));
        }
    }

    fs::write(path, content).with_context(|| format!("Failed to write {:?}", path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn launcher_preferences_default_to_config_directory() {
        let temp = tempdir().unwrap();
        let settings = AppSettings::default();

        let prefs = terminal_launcher_preferences(temp.path(), &settings);

        assert_eq!(
            prefs.default_working_directory,
            normalize_output_path(temp.path())
        );
        assert_eq!(prefs.working_directory, prefs.default_working_directory);
    }

    #[test]
    fn launcher_preferences_keep_saved_directory() {
        let temp = tempdir().unwrap();
        let mut settings = AppSettings::default();
        settings.assistant_working_directory = Some("C:/tmp/custom-assistant".into());

        let prefs = terminal_launcher_preferences(temp.path(), &settings);

        assert_eq!(prefs.working_directory, "C:/tmp/custom-assistant");
    }

    #[test]
    fn ensure_terminal_workspace_creates_agent_guides_and_skills() {
        let temp = tempdir().unwrap();

        ensure_terminal_workspace(temp.path()).unwrap();

        assert!(temp.path().join("agents.md").is_file());

        for (_, skills_dir_rule) in TERMINAL_AGENT_ROOTS {
            let skills_dir = temp.path().join(skills_dir_rule);
            assert!(skills_dir.is_dir());
            assert!(skills_dir.join(CONFIG_GUIDE_SKILL_NAME).join("SKILL.md").is_file());
            assert!(skills_dir
                .join(EXTERNAL_SOURCE_SKILL_NAME)
                .join("SKILL.md")
                .is_file());
        }
    }

    #[test]
    fn load_launcher_preferences_bootstraps_workspace_files() {
        let temp = tempdir().unwrap();

        let prefs = load_terminal_launcher_preferences(temp.path()).unwrap();

        assert_eq!(prefs.working_directory, prefs.default_working_directory);
        assert!(temp.path().join("agents.md").is_file());
    }
}

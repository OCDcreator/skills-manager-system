use super::*;

#[test]
fn detect_generated_agent_bundle_prefers_built_agent_target() {
    let temp = tempdir().unwrap();
    let repo_dir = temp.path();

    create_skill_dir(repo_dir, "source/skills/impeccable");
    create_skill_dir(repo_dir, "dist/agents/.agents/skills/impeccable");

    let result = detect_external_source_variants(repo_dir, "src_test").unwrap();

    assert_eq!(result.kind, "generated_agent_bundle");
    assert_eq!(result.variants.len(), 1);
    assert_eq!(result.variants[0].agent_key, "codex");
    assert_eq!(
        result.variants[0].variant_path,
        "dist/agents/.agents/skills/impeccable"
    );
    assert_eq!(
        result.variants[0].source_of_truth_path.as_deref(),
        Some("source/skills/impeccable")
    );
    assert!(result.warnings.is_empty());
}

#[test]
fn detect_unknown_agent_variant_emits_warning() {
    let temp = tempdir().unwrap();
    let repo_dir = temp.path();

    create_skill_dir(repo_dir, "source/skills/impeccable");
    create_skill_dir(repo_dir, "dist/agents/.mystery/skills/impeccable");

    let result = detect_external_source_variants(repo_dir, "src_test").unwrap();

    assert_eq!(result.kind, "skill_repository");
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "skill_repository"
            && variant.variant_path == "dist/agents/.mystery/skills/impeccable"
    }));
    assert!(result
        .warnings
        .iter()
        .any(|warning| warning.code == "unsupported_agent_variant"));
}

#[test]
fn detect_unknown_agent_variant_emits_warning_under_known_agent_root() {
    let temp = tempdir().unwrap();
    let repo_dir = temp.path();

    create_skill_dir(repo_dir, "source/skills/impeccable");
    create_skill_dir(repo_dir, "dist/agents/.agents/skills/impeccable");
    create_skill_dir(repo_dir, "dist/agents/.agents/skills/group/impeccable");

    let result = detect_external_source_variants(repo_dir, "src_test").unwrap();

    assert_eq!(result.kind, "generated_agent_bundle");
    assert_eq!(result.variants.len(), 2);
    assert!(result.warnings.iter().any(|warning| {
        warning.code == "unsupported_agent_variant"
            && warning
                .message
                .contains("dist/agents/.agents/skills/group/impeccable")
    }));
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "skill_repository"
            && variant.variant_path == "dist/agents/.agents/skills/group/impeccable"
    }));
}

#[test]
fn detect_generated_agent_bundle_supports_root_hidden_agent_roots() {
    let temp = tempdir().unwrap();
    let repo_dir = temp.path();

    create_skill_dir(repo_dir, "source/skills/impeccable");
    create_skill_dir(repo_dir, ".agents/skills/impeccable");
    create_skill_dir(repo_dir, ".claude/skills/impeccable");
    create_skill_dir(repo_dir, ".opencode/skills/impeccable");
    create_skill_dir(repo_dir, ".cursor/skills/impeccable");
    create_skill_dir(repo_dir, ".gemini/skills/impeccable");
    create_skill_dir(repo_dir, ".github/skills/impeccable");
    create_skill_dir(repo_dir, ".kiro/skills/impeccable");

    let result = detect_external_source_variants(repo_dir, "src_test").unwrap();

    assert_eq!(result.kind, "generated_agent_bundle");
    assert_eq!(result.variants.len(), 7);
    assert!(result
        .variants
        .iter()
        .any(|variant| variant.agent_key == "codex"
            && variant.variant_path == ".agents/skills/impeccable"));
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "claude_code" && variant.variant_path == ".claude/skills/impeccable"
    }));
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "opencode" && variant.variant_path == ".opencode/skills/impeccable"
    }));
    assert!(result
        .variants
        .iter()
        .any(|variant| variant.agent_key == "cursor"
            && variant.variant_path == ".cursor/skills/impeccable"));
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "gemini_cli" && variant.variant_path == ".gemini/skills/impeccable"
    }));
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "github_copilot" && variant.variant_path == ".github/skills/impeccable"
    }));
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "kilo_code" && variant.variant_path == ".kiro/skills/impeccable"
    }));
}

#[test]
fn detect_generated_agent_bundle_supports_dist_variant_roots_for_aligned_agents() {
    let temp = tempdir().unwrap();
    let repo_dir = temp.path();

    create_skill_dir(repo_dir, "source/skills/impeccable");
    create_skill_dir(repo_dir, "dist/cursor/.cursor/skills/impeccable");
    create_skill_dir(repo_dir, "dist/gemini/.gemini/skills/impeccable");
    create_skill_dir(repo_dir, "dist/github/.github/skills/impeccable");
    create_skill_dir(repo_dir, "dist/kiro/.kiro/skills/impeccable");

    let result = detect_external_source_variants(repo_dir, "src_test").unwrap();

    assert_eq!(result.kind, "generated_agent_bundle");
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "cursor"
            && variant.variant_path == "dist/cursor/.cursor/skills/impeccable"
    }));
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "gemini_cli"
            && variant.variant_path == "dist/gemini/.gemini/skills/impeccable"
    }));
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "github_copilot"
            && variant.variant_path == "dist/github/.github/skills/impeccable"
    }));
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "kilo_code"
            && variant.variant_path == "dist/kiro/.kiro/skills/impeccable"
    }));
}

#[test]
fn detect_generated_agent_bundle_supports_codex_alias_roots() {
    let temp = tempdir().unwrap();
    let repo_dir = temp.path();

    create_skill_dir(repo_dir, "source/skills/impeccable");
    create_skill_dir(repo_dir, ".codex/skills/impeccable");
    create_skill_dir(repo_dir, "dist/codex/.codex/skills/portable");

    let result = detect_external_source_variants(repo_dir, "src_test").unwrap();

    assert_eq!(result.kind, "generated_agent_bundle");
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "codex" && variant.variant_path == ".codex/skills/impeccable"
    }));
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "codex"
            && variant.variant_path == "dist/codex/.codex/skills/portable"
    }));
}

#[test]
fn detect_generated_agent_bundle_remains_preferred_over_generic_skills() {
    let temp = tempdir().unwrap();
    let repo_dir = temp.path();

    fs::write(repo_dir.join("SKILL.md"), "# Root skill\n").unwrap();
    create_skill_dir(repo_dir, ".agents/skills/generated");

    let result = detect_external_source_variants(repo_dir, "src_test").unwrap();

    assert_eq!(result.kind, "generated_agent_bundle");
    assert_eq!(result.variants.len(), 2);
    assert_eq!(result.variants[0].variant_path, ".agents/skills/generated");
    assert!(result
        .variants
        .iter()
        .any(|variant| variant.agent_key == "skill_repository" && variant.variant_path == "."));
}

use super::*;

#[test]
fn detect_supported_layouts_still_include_recursive_generic_skill_candidates() {
    let temp = tempdir().unwrap();
    let repo_dir = temp.path();

    create_skill_dir(repo_dir, ".agents/skills/generated");
    create_skill_dir(repo_dir, "packages/fallback/manual-one");
    create_skill_dir(repo_dir, "packages/fallback/group/manual-two");

    let result = detect_external_source_variants(repo_dir, "src_test").unwrap();

    assert_eq!(result.kind, "generated_agent_bundle");
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "codex" && variant.variant_path == ".agents/skills/generated"
    }));
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "skill_repository"
            && variant.variant_path == "packages/fallback/manual-one"
    }));
    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "skill_repository"
            && variant.variant_path == "packages/fallback/group/manual-two"
    }));
}

#[test]
fn detect_recursive_generic_candidates_skip_noise_directories() {
    let temp = tempdir().unwrap();
    let repo_dir = temp.path();

    create_skill_dir(repo_dir, "packages/skills/visible");
    create_skill_dir(repo_dir, "node_modules/pkg/hidden");
    create_skill_dir(repo_dir, ".git/hooks/hidden");
    create_skill_dir(repo_dir, "target/debug/hidden");

    let result = detect_external_source_variants(repo_dir, "src_test").unwrap();

    assert!(result.variants.iter().any(|variant| {
        variant.agent_key == "skill_repository"
            && variant.variant_path == "packages/skills/visible"
    }));
    assert!(!result
        .variants
        .iter()
        .any(|variant| variant.variant_path == "node_modules/pkg/hidden"));
    assert!(!result
        .variants
        .iter()
        .any(|variant| variant.variant_path == ".git/hooks/hidden"));
    assert!(!result
        .variants
        .iter()
        .any(|variant| variant.variant_path == "target/debug/hidden"));
}

#[test]
fn detect_generic_skill_repo_root_exports_root_variant() {
    let temp = tempdir().unwrap();
    let repo_dir = temp.path();

    fs::write(repo_dir.join("SKILL.md"), "# Root skill\n").unwrap();

    let result = detect_external_source_variants(repo_dir, "src_test").unwrap();

    assert_eq!(result.kind, "skill_repository");
    assert_eq!(result.variants.len(), 1);
    assert_eq!(result.variants[0].agent_key, "skill_repository");
    assert_eq!(result.variants[0].variant_path, ".");
    assert_eq!(result.variants[0].metadata_path.as_deref(), Some("SKILL.md"));
    assert!(result.warnings.is_empty());
}

#[test]
fn detect_generic_skill_repo_subpath_lists_direct_child_skills() {
    let temp = tempdir().unwrap();
    let repo_dir = temp.path();

    create_skill_dir(repo_dir, "packages/skills/alpha");
    create_skill_dir(repo_dir, "packages/skills/beta");
    create_skill_dir(repo_dir, "packages/skills/group/nested");

    let result = detect_external_source_variants_at_ref_in_root(
        repo_dir,
        "HEAD",
        "src_test",
        Some("packages/skills"),
    )
    .or_else(|_| {
        Ok::<_, anyhow::Error>(detect_external_source_variants_from_worktree_in_root(
            repo_dir,
            "src_test",
            Some("packages/skills"),
        )?)
    })
    .unwrap();

    assert_eq!(result.kind, "skill_repository");
    assert_eq!(
        result
            .variants
            .iter()
            .map(|variant| variant.variant_path.as_str())
            .collect::<Vec<_>>(),
        vec![
            "packages/skills/alpha",
            "packages/skills/beta",
            "packages/skills/group/nested",
        ]
    );
}

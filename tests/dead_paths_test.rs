use agent_memgc::engine::lint_content;
use agent_memgc::domain::RuleCode;
use tempfile::tempdir;

#[test]
fn test_dead_path_detection() {
    let dir = tempdir().expect("Failed to create tempdir");
    let root = dir.path();

    // Create one valid file
    let valid_file = root.join("src").join("main.rs");
    std::fs::create_dir_all(valid_file.parent().unwrap()).unwrap();
    std::fs::write(&valid_file, "fn main() {}").unwrap();

    let content = r#"# Agent Guidelines
- To check code, look at `src/main.rs`.
- Run tests using `scripts/deploy_v1.sh` before PR.
- Legacy controller lives at `src/controllers/old_auth.py`.
"#;

    let report = lint_content(content, "AGENTS.md", root).expect("Linting should succeed");

    let dead_path_findings: Vec<_> = report
        .findings
        .iter()
        .filter(|f| f.rule == RuleCode::MGC001)
        .collect();

    assert_eq!(dead_path_findings.len(), 2);
    assert!(dead_path_findings.iter().any(|f| f.target == "scripts/deploy_v1.sh"));
    assert!(dead_path_findings.iter().any(|f| f.target == "src/controllers/old_auth.py"));
}

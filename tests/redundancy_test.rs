use agent_memgc::engine::lint_content;
use agent_memgc::domain::RuleCode;
use std::path::Path;

#[test]
fn test_redundancy_and_duplicate_detection() {
    let content = r#"# Engineering Directives
- Always run automated test suite before opening PR.
- Keep commits strictly atomic and signed.
- Always run automated test suite before opening PR.
"#;

    let report = lint_content(content, "AGENTS.md", Path::new(".")).expect("Linting should succeed");

    let duplicates: Vec<_> = report
        .findings
        .iter()
        .filter(|f| f.rule == RuleCode::MGC004)
        .collect();

    assert_eq!(duplicates.len(), 1);
    assert_eq!(duplicates[0].line, 4);
}

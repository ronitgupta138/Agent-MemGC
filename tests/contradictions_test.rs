use agent_memgc::engine::lint_content;
use agent_memgc::domain::RuleCode;
use std::path::Path;

#[test]
fn test_semantic_contradiction_detection() {
    let content = r#"# Formatting Rules
- Always use tabs for indentation in all source files.
- Ensure all components are written as synchronous functions.
- Every file must be formatted with 2 spaces.
"#;

    let report = lint_content(content, "CLAUDE.md", Path::new(".")).expect("Linting should succeed");

    let contradictions: Vec<_> = report
        .findings
        .iter()
        .filter(|f| f.rule == RuleCode::MGC002)
        .collect();

    assert!(!contradictions.is_empty());
    assert!(contradictions[0].message.contains("Indentation"));
}

use agent_memgc::engine::lint_content;
use agent_memgc::domain::RuleCode;
use std::path::Path;

#[test]
fn test_cache_layout_volatile_prefix_detection() {
    let content = r#"# Project Memory
Session-ID: 2026-10-18-run-9128
Last-Updated: 2026-10-18 14:22:01

## Architecture Rules
- Use pure functions where possible.
"#;

    let report = lint_content(content, "MEMORY.md", Path::new(".")).expect("Linting should succeed");

    assert!(!report.cache_clean);
    let cache_findings: Vec<_> = report
        .findings
        .iter()
        .filter(|f| f.rule == RuleCode::MGC003)
        .collect();

    assert!(!cache_findings.is_empty());
}

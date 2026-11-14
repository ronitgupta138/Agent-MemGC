use agent_memgc::rules::markdown_fences::check_markdown_fences;
use agent_memgc::domain::RuleCode;

#[test]
fn test_unclosed_fence_detection() {
    let text = "# Section\n```rust\nfn main() {}\n";
    let findings = check_markdown_fences(text);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule, RuleCode::MGC006);
    assert_eq!(findings[0].line, 2);
}

#[test]
fn test_closed_fence_passes() {
    let text = "# Section\n```rust\nfn main() {}\n```\n";
    let findings = check_markdown_fences(text);
    assert!(findings.is_empty());
}

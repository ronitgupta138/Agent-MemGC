use agent_memgc::rules::token_density::check_token_density;
use agent_memgc::domain::RuleCode;

#[test]
fn test_filler_phrase_detection() {
    let text = "# Agent Guidelines\nPlease remember to format all code blocks.\nAlways run tests.\n";
    let findings = check_token_density(text);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule, RuleCode::MGC005);
    assert_eq!(findings[0].line, 2);
}

#[test]
fn test_clean_imperative_instructions() {
    let text = "# Agent Guidelines\nFormat all code blocks.\nRun test suite.\n";
    let findings = check_token_density(text);
    assert!(findings.is_empty());
}

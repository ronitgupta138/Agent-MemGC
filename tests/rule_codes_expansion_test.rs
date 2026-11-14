use agent_memgc::domain::RuleCode;

#[test]
fn test_new_rule_codes_string_representation() {
    assert_eq!(RuleCode::MGC005.as_str(), "MGC005");
    assert_eq!(RuleCode::MGC006.as_str(), "MGC006");
    assert_eq!(RuleCode::MGC005.title(), "Low Information Density / Filler Phrase");
    assert_eq!(RuleCode::MGC006.title(), "Malformed or Unclosed Code Fence");
}

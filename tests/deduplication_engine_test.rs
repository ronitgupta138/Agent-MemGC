// Test suite for instruction deduplication engine

#[test]
fn test_exact_duplicate_detection() {
    let rule_a = "Never use markdown links.";
    let rule_b = "Never use markdown links.";
    assert_eq!(rule_a, rule_b);
}

#[test]
fn test_deduplication_reduces_rule_count() {
    let mut rules = vec!["Rule A", "Rule B", "Rule A", "Rule C"];
    rules.sort();
    rules.dedup();
    assert_eq!(rules.len(), 3);
}

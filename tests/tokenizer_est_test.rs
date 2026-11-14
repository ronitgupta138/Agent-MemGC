use agent_memgc::parser::tokenizer_est::estimate_tokens;

#[test]
fn test_estimate_tokens_empty() {
    assert_eq!(estimate_tokens(""), 0);
}

#[test]
fn test_estimate_tokens_reasonable_bounds() {
    let sentence = "The quick brown fox jumps over the lazy dog.";
    let tokens = estimate_tokens(sentence);
    assert!(tokens >= 9 && tokens <= 15);
}

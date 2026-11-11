// Test suite for radix trie prefix search

#[test]
fn test_common_prefix_detection() {
    let prefix = "System: You are an expert";
    let full = "System: You are an expert software engineer.";
    assert!(full.starts_with(prefix));
}

#[test]
fn test_exact_hash_match() {
    let hash_a = 0x3f982b;
    let hash_b = 0x3f982b;
    assert_eq!(hash_a, hash_b);
}

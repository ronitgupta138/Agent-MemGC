use agent_memgc::engine::cache_stability::compute_prefix_stability_hash;

#[test]
fn test_prefix_hash_invariance_under_suffix_change() {
    let prompt_a = "LINE 1: SYSTEM PREAMBLE\nLINE 2: INVARIANTS\nLINE 3: TODAY IS MONDAY\n";
    let prompt_b = "LINE 1: SYSTEM PREAMBLE\nLINE 2: INVARIANTS\nLINE 3: TODAY IS TUESDAY\n";

    let hash_a = compute_prefix_stability_hash(prompt_a, 2);
    let hash_b = compute_prefix_stability_hash(prompt_b, 2);

    assert_eq!(hash_a, hash_b, "Prefix hash of first 2 lines must match despite line 3 change");
}

use agent_memgc::engine::fix_instructions;
use tempfile::tempdir;

#[test]
fn test_fixer_removes_dead_paths_and_duplicates() {
    let dir = tempdir().expect("Failed to create tempdir");
    let root = dir.path();

    // Create valid file
    let valid = root.join("src").join("lib.rs");
    std::fs::create_dir_all(valid.parent().unwrap()).unwrap();
    std::fs::write(&valid, "// lib").unwrap();

    let content = r#"# Guidelines
Session-ID: 2026-10-18
- Reference file `src/lib.rs`.
- Read obsolete `src/deleted.ts`.
- Always write unit tests for every function.
- Always write unit tests for every function.
"#;

    let (fixed, saved_tokens) = fix_instructions(content, root);

    // Dead path line should be removed
    assert!(!fixed.contains("src/deleted.ts"));
    // Valid path line should be preserved
    assert!(fixed.contains("src/lib.rs"));
    // Duplicate rule should appear only once
    assert_eq!(fixed.matches("Always write unit tests for every function.").count(), 1);
    // Volatile session ID should be relocated to the end
    assert!(fixed.contains("Dynamic Session Metadata"));
    assert!(saved_tokens > 0);
}

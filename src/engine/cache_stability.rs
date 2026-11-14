use sha2::{Digest, Sha256};

/// Compute a deterministic SHA-256 hash of the prompt's immutable prefix lines.
pub fn compute_prefix_stability_hash(content: &str, prefix_lines: usize) -> String {
    let mut hasher = Sha256::new();
    for line in content.lines().take(prefix_lines) {
        hasher.update(line.trim().as_bytes());
        hasher.update(b"\n");
    }
    hex::encode(hasher.finalize())
}

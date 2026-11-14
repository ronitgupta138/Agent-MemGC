/// Approximate token count estimator based on character heuristics (~4 chars/token).
pub fn estimate_tokens(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }
    let words = text.split_whitespace().count();
    let char_len = text.chars().count();
    let est = (char_len as f64 / 3.85).ceil() as usize;
    est.max(words)
}

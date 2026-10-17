use crate::domain::{InstructionBlock, LinterFinding, RuleCode, Severity};
use regex::Regex;

/// Check if volatile dynamic data sits at the prefix of instructions, breaking KV-cache prefix hits.
pub fn check_cache_layout(blocks: &[InstructionBlock]) -> Vec<LinterFinding> {
    let mut findings = Vec::new();

    // Inspect first 8 non-empty instruction blocks
    let volatile_patterns = [
        ("timestamp", r"\b\d{4}-\d{2}-\d{2}\b"),
        ("time", r"\b\d{2}:\d{2}:\d{2}\b"),
        ("session_id", r"(?i)session[_\s\-]?id"),
        ("last_updated", r"(?i)last[_\s\-]updated"),
        ("date_marker", r"(?i)date:\s*"),
    ];

    for block in blocks.iter().take(8) {
        let text = &block.text;
        // Skip title heading
        if text.starts_with("# ") {
            continue;
        }

        for (label, pattern) in &volatile_patterns {
            if let Ok(re) = Regex::new(pattern) {
                if re.is_match(text) {
                    findings.push(LinterFinding {
                        rule: RuleCode::MGC003,
                        severity: Severity::Info,
                        line: block.line_start,
                        target: format!("Volatile pattern: {label}"),
                        message: format!(
                            "Volatile dynamic content at line {} breaks prompt prefix caching.",
                            block.line_start
                        ),
                        suggestion: Some(
                            "Move dynamic timestamps, session IDs, and dates to the end of the prompt or into user messages so the static instruction prefix caches across turns."
                                .to_string(),
                        ),
                        fixable: true,
                    });
                    break;
                }
            }
        }
    }

    findings
}

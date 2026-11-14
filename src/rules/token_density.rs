use crate::domain::{LinterFinding, RuleCode, Severity};

const FILLER_PHRASES: &[&str] = &[
    "please remember to",
    "as an ai assistant",
    "it is important to note that",
    "feel free to",
    "in order to ensure that",
];

/// Scan content for low-density conversational filler phrases that waste context tokens.
pub fn check_token_density(content: &str) -> Vec<LinterFinding> {
    let mut findings = Vec::new();
    for (line_idx, line) in content.lines().enumerate() {
        let lower = line.to_lowercase();
        for phrase in FILLER_PHRASES {
            if lower.contains(phrase) {
                findings.push(LinterFinding {
                    rule: RuleCode::MGC005,
                    severity: Severity::Warning,
                    line: line_idx + 1,
                    target: phrase.to_string(),
                    message: format!("Low information density filler phrase detected: '{}'", phrase),
                    suggestion: Some("Convert to direct imperative command".to_string()),
                    fixable: true,
                });
            }
        }
    }
    findings
}

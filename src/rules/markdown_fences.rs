use crate::domain::{LinterFinding, RuleCode, Severity};

/// Check for unclosed or improperly formatted markdown code fences in instruction prompts.
pub fn check_markdown_fences(content: &str) -> Vec<LinterFinding> {
    let mut findings = Vec::new();
    let mut in_fence = false;
    let mut open_line = 0;

    for (line_idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            if in_fence {
                in_fence = false;
            } else {
                in_fence = true;
                open_line = line_idx + 1;
            }
        }
    }

    if in_fence {
        findings.push(LinterFinding {
            rule: RuleCode::MGC006,
            severity: Severity::Error,
            line: open_line,
            target: "```".to_string(),
            message: format!("Unclosed markdown code fence opened at line {}", open_line),
            suggestion: Some("Append closing ``` fence before end of prompt".to_string()),
            fixable: true,
        });
    }

    findings
}

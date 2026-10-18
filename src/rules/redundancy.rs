use crate::domain::{InstructionBlock, LinterFinding, RuleCode, Severity};
use std::collections::HashMap;

/// Detect redundant or verbatim duplicated instruction lines.
pub fn check_redundancy(blocks: &[InstructionBlock]) -> Vec<LinterFinding> {
    let mut findings = Vec::new();
    let mut seen: HashMap<String, usize> = HashMap::new();

    for block in blocks {
        let line = block.text.trim();
        // Skip headings, empty lines, and short bullet markers
        if line.starts_with('#') || line.is_empty() || line.len() < 12 {
            continue;
        }

        let normalized = line
            .trim_start_matches(['-', '*', '+', ' ', '\t'])
            .to_lowercase();

        if let Some(&first_line) = seen.get(&normalized) {
            findings.push(LinterFinding {
                rule: RuleCode::MGC004,
                severity: Severity::Warning,
                line: block.line_start,
                target: format!("Line {first_line} vs Line {}", block.line_start),
                message: format!(
                    "Duplicated instruction: \"{}\" was already stated on line {first_line}.",
                    block.text.trim()
                ),
                suggestion: Some(
                    "Remove redundant duplicate rules to conserve context window space and lower cost."
                        .to_string(),
                ),
                fixable: true,
            });
        } else {
            seen.insert(normalized, block.line_start);
        }
    }

    findings
}

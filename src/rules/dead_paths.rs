use crate::domain::{InstructionBlock, LinterFinding, RuleCode, Severity};
use std::path::Path;

/// Check for dead file paths or scripts cited in instruction blocks.
pub fn check_dead_paths(blocks: &[InstructionBlock], project_root: &Path) -> Vec<LinterFinding> {
    let mut findings = Vec::new();

    for block in blocks {
        for path_str in &block.backticked_paths {
            // Strip leading ./ or /
            let clean = path_str.trim_start_matches("./").trim_start_matches('/');
            let target_path = project_root.join(clean);

            if !target_path.exists() {
                // If it's a known generic example or wildcard, skip
                if clean.contains('*') || clean.contains('<') || clean.contains("...") || clean.contains("path/to") {
                    continue;
                }

                findings.push(LinterFinding {
                    rule: RuleCode::MGC001,
                    severity: Severity::Error,
                    line: block.line_start,
                    target: path_str.clone(),
                    message: format!("Referenced path `{path_str}` does not exist in workspace."),
                    suggestion: Some(format!("Verify if `{path_str}` was deleted or renamed; remove dead reference to prevent agent hallucination.")),
                    fixable: true,
                });
            }
        }
    }

    findings
}

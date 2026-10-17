use crate::domain::{InstructionBlock, LinterFinding, RuleCode, Severity};

struct ContradictionPair {
    category: &'static str,
    pattern_a: &'static str,
    pattern_b: &'static str,
    explanation: &'static str,
}

const CONTRADICTION_PAIRS: &[ContradictionPair] = &[
    ContradictionPair {
        category: "Indentation",
        pattern_a: "tab",
        pattern_b: "space",
        explanation: "Conflicting indentation directives: both tabs and spaces are enforced in separate instructions.",
    },
    ContradictionPair {
        category: "Module Format",
        pattern_a: "esm",
        pattern_b: "commonjs",
        explanation: "Conflicting module directives: both ESM (import) and CommonJS (require) are mandated.",
    },
    ContradictionPair {
        category: "Concurrency Paradigm",
        pattern_a: "synchronous",
        pattern_b: "async",
        explanation: "Conflicting execution constraints: synchronous execution and async pipelines both required for same component.",
    },
    ContradictionPair {
        category: "Testing Strategy",
        pattern_a: "no mock",
        pattern_b: "mock all",
        explanation: "Conflicting test guidelines: instruction says 'no mocks' while another mandates mock fixtures.",
    },
    ContradictionPair {
        category: "Identifier Casing",
        pattern_a: "camelcase",
        pattern_b: "snake_case",
        explanation: "Conflicting naming conventions: conflicting casing styles mandated without scope qualification.",
    },
];

/// Detect opposing semantic directives across instruction blocks.
pub fn check_contradictions(blocks: &[InstructionBlock]) -> Vec<LinterFinding> {
    let mut findings = Vec::new();

    for pair in CONTRADICTION_PAIRS {
        let mut match_a: Option<(usize, String)> = None;
        let mut match_b: Option<(usize, String)> = None;

        for block in blocks {
            let lower = block.text.to_lowercase();
            if lower.contains(pair.pattern_a) && match_a.is_none() {
                match_a = Some((block.line_start, block.text.clone()));
            }
            if lower.contains(pair.pattern_b) && match_b.is_none() {
                match_b = Some((block.line_start, block.text.clone()));
            }
        }

        if let (Some((line_a, text_a)), Some((line_b, text_b))) = (match_a, match_b) {
            if line_a != line_b {
                findings.push(LinterFinding {
                    rule: RuleCode::MGC002,
                    severity: Severity::Warning,
                    line: line_b,
                    target: format!("Line {line_a} vs Line {line_b}"),
                    message: format!(
                        "Contradiction [{}]: \"{}\" conflicts with line {line_a} \"{}\"",
                        pair.category,
                        text_b.trim(),
                        text_a.trim()
                    ),
                    suggestion: Some(pair.explanation.to_string()),
                    fixable: false,
                });
            }
        }
    }

    findings
}

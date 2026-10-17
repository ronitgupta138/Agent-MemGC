use crate::domain::finding::LinterFinding;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionBlock {
    pub line_start: usize,
    pub line_end: usize,
    pub heading: Option<String>,
    pub text: String,
    pub backticked_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinterReport {
    pub target_file: String,
    pub total_lines: usize,
    pub findings: Vec<LinterFinding>,
    pub errors_count: usize,
    pub warnings_count: usize,
    pub info_count: usize,
    pub estimated_wasted_tokens: usize,
    pub cache_clean: bool,
}

impl LinterReport {
    pub fn new(target_file: String, total_lines: usize, findings: Vec<LinterFinding>) -> Self {
        let mut errors = 0;
        let mut warnings = 0;
        let mut infos = 0;
        let mut wasted_tokens = 0;
        let mut cache_clean = true;

        for f in &findings {
            match f.severity {
                crate::domain::finding::Severity::Error => errors += 1,
                crate::domain::finding::Severity::Warning => warnings += 1,
                crate::domain::finding::Severity::Info => infos += 1,
            }

            match f.rule {
                crate::domain::finding::RuleCode::MGC001 => wasted_tokens += 25,
                crate::domain::finding::RuleCode::MGC002 => wasted_tokens += 60,
                crate::domain::finding::RuleCode::MGC003 => cache_clean = false,
                crate::domain::finding::RuleCode::MGC004 => wasted_tokens += 35,
            }
        }

        Self {
            target_file,
            total_lines,
            findings,
            errors_count: errors,
            warnings_count: warnings,
            info_count: infos,
            estimated_wasted_tokens: wasted_tokens,
            cache_clean,
        }
    }

    pub fn has_errors(&self) -> bool {
        self.errors_count > 0
    }
}

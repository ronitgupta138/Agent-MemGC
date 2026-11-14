use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleCode {
    /// Dead file path or missing script reference in agent instructions
    MGC001,
    /// Semantic contradiction between two conflicting directives
    MGC002,
    /// Volatile instruction header breaking prompt prefix cache
    MGC003,
    /// Redundant or duplicate instruction wasting agent context
    MGC004,
    /// Low information density or conversational filler phrase
    MGC005,
    /// Malformed or unclosed markdown code fence
    MGC006,
}

impl RuleCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            RuleCode::MGC001 => "MGC001",
            RuleCode::MGC002 => "MGC002",
            RuleCode::MGC003 => "MGC003",
            RuleCode::MGC004 => "MGC004",
            RuleCode::MGC005 => "MGC005",
            RuleCode::MGC006 => "MGC006",
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            RuleCode::MGC001 => "Dead Path Reference",
            RuleCode::MGC002 => "Semantic Directive Contradiction",
            RuleCode::MGC003 => "Prompt-Cache Prefix Invalidation",
            RuleCode::MGC004 => "Duplicate / Redundant Rule",
            RuleCode::MGC005 => "Low Information Density / Filler Phrase",
            RuleCode::MGC006 => "Malformed or Unclosed Code Fence",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinterFinding {
    pub rule: RuleCode,
    pub severity: Severity,
    pub line: usize,
    pub target: String,
    pub message: String,
    pub suggestion: Option<String>,
    pub fixable: bool,
}

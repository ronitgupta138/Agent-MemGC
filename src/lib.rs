//! agent-memgc: High-performance linter, cache-layout optimizer, and generational garbage collector for AI agent instruction files.

pub mod cli;
pub mod domain;
pub mod engine;
pub mod mcp;
pub mod parser;
pub mod rules;

pub use domain::{LinterFinding, LinterReport, RuleCode, Severity};
pub use engine::{fix_instructions, lint_content, lint_file};
pub use mcp::McpServer;

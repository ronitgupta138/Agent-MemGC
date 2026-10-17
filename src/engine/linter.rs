use crate::domain::LinterReport;
use crate::parser::parse_instruction_markdown;
use crate::rules::{
    check_cache_layout, check_contradictions, check_dead_paths, check_redundancy,
};
use std::path::Path;

/// Lints an instruction file and generates a structured report with actionable findings.
pub fn lint_file(file_path: &Path, project_root: &Path) -> anyhow::Result<LinterReport> {
    let content = std::fs::read_to_string(file_path)?;
    lint_content(&content, file_path.to_string_lossy().as_ref(), project_root)
}

/// Lints raw instruction content and returns findings.
pub fn lint_content(
    content: &str,
    target_name: &str,
    project_root: &Path,
) -> anyhow::Result<LinterReport> {
    let blocks = parse_instruction_markdown(content);
    let total_lines = content.lines().count();

    let mut findings = Vec::new();
    findings.extend(check_dead_paths(&blocks, project_root));
    findings.extend(check_contradictions(&blocks));
    findings.extend(check_cache_layout(&blocks));
    findings.extend(check_redundancy(&blocks));

    findings.sort_by_key(|f| f.line);

    Ok(LinterReport::new(
        target_name.to_string(),
        total_lines,
        findings,
    ))
}

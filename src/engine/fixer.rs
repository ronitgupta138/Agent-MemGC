use crate::engine::linter::lint_content;
use std::collections::HashSet;
use std::path::Path;

/// Automatically cleans, deduplicates, and optimizes instruction content for prompt caching.
pub fn fix_instructions(content: &str, project_root: &Path) -> (String, usize) {
    let report = match lint_content(content, "target.md", project_root) {
        Ok(r) => r,
        Err(_) => return (content.to_string(), 0),
    };

    let mut lines_to_drop: HashSet<usize> = HashSet::new();
    let mut volatile_lines: Vec<String> = Vec::new();

    for f in &report.findings {
        if f.fixable {
            match f.rule {
                crate::domain::RuleCode::MGC001 => {
                    // Dead path line
                    lines_to_drop.insert(f.line);
                }
                crate::domain::RuleCode::MGC004 => {
                    // Redundant duplicate line
                    lines_to_drop.insert(f.line);
                }
                crate::domain::RuleCode::MGC003 => {
                    // Volatile prefix line - extract to move down
                    lines_to_drop.insert(f.line);
                }
                _ => {}
            }
        }
    }

    let raw_lines: Vec<&str> = content.lines().collect();
    let mut clean_lines: Vec<String> = Vec::new();

    for (idx, line) in raw_lines.iter().enumerate() {
        let line_num = idx + 1;
        if lines_to_drop.contains(&line_num) {
            // Check if this was a volatile line we should preserve at the end
            let is_volatile = report
                .findings
                .iter()
                .any(|f| f.line == line_num && f.rule == crate::domain::RuleCode::MGC003);
            if is_volatile {
                volatile_lines.push(line.to_string());
            }
            continue;
        }
        clean_lines.push(line.to_string());
    }

    // Append volatile lines at the end under a dynamic context section if any were relocated
    if !volatile_lines.is_empty() {
        clean_lines.push(String::new());
        clean_lines.push("<!-- Dynamic Session Metadata (Moved to preserve KV-cache prefix) -->".to_string());
        for vl in volatile_lines {
            clean_lines.push(vl);
        }
    }

    let mut fixed = clean_lines.join("\n");
    if !fixed.ends_with('\n') {
        fixed.push('\n');
    }

    let tokens_saved = report.estimated_wasted_tokens;
    (fixed, tokens_saved)
}

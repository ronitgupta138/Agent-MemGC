use crate::domain::finding::Severity;
use crate::engine::{fix_instructions, lint_file};
use colored::*;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Color, Table};
use std::fs;
use std::path::Path;

pub fn cmd_check(path: &Path, project_root: &Path, json_mode: bool) -> anyhow::Result<bool> {
    let report = lint_file(path, project_root)?;

    if json_mode {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(!report.has_errors());
    }

    println!("\n{}", format!("=== agent-memgc Instruction Audit: {} ===", path.display()).cyan().bold());
    println!("Lines Analyzed: {} | Findings: {} (Errors: {}, Warnings: {}, Info: {})",
        report.total_lines,
        report.findings.len(),
        report.errors_count.to_string().red().bold(),
        report.warnings_count.to_string().yellow().bold(),
        report.info_count.to_string().blue().bold()
    );

    if !report.cache_clean {
        println!("{}", "⚠️  Prompt Prefix Cache: UNHEALTHY (Volatile content found at instruction prefix)".yellow().bold());
    } else {
        println!("{}", "✅ Prompt Prefix Cache: OPTIMIZED (Static rules pinned to instruction prefix)".green().bold());
    }

    if report.estimated_wasted_tokens > 0 {
        println!("Estimated Wasted Tokens: ~{} tokens/turn\n", report.estimated_wasted_tokens.to_string().yellow());
    } else {
        println!();
    }

    if report.findings.is_empty() {
        println!("{}", "✨ All instruction rules passed! Zero dead paths, contradictions, or cache bloat.\n".green().bold());
        return Ok(true);
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL).apply_modifier(UTF8_ROUND_CORNERS);
    table.set_header(vec![
        Cell::new("Rule").fg(Color::Cyan),
        Cell::new("Line").fg(Color::Yellow),
        Cell::new("Severity").fg(Color::Magenta),
        Cell::new("Message").fg(Color::White),
    ]);

    for f in &report.findings {
        let (sev_str, sev_color) = match f.severity {
            Severity::Error => ("ERROR", Color::Red),
            Severity::Warning => ("WARN", Color::Yellow),
            Severity::Info => ("INFO", Color::Blue),
        };

        table.add_row(vec![
            Cell::new(f.rule.as_str()).fg(Color::Cyan),
            Cell::new(f.line.to_string()).fg(Color::Yellow),
            Cell::new(sev_str).fg(sev_color),
            Cell::new(&f.message),
        ]);
    }

    println!("{}\n", table);

    println!("{}", "💡 Run `agent-memgc fix <file>` to automatically resolve dead paths and cache ordering.".cyan());
    Ok(!report.has_errors())
}

pub fn cmd_fix(path: &Path, project_root: &Path, dry_run: bool) -> anyhow::Result<()> {
    let content = fs::read_to_string(path)?;
    let (fixed_content, saved_tokens) = fix_instructions(&content, project_root);

    if content == fixed_content {
        println!("{}", "✨ File is already clean and optimized; no changes required.".green().bold());
        return Ok(());
    }

    if dry_run {
        println!("\n{}", format!("=== Dry-Run Fix Diff: {} ===", path.display()).cyan().bold());
        println!("{}", fixed_content);
        println!("\n{}", format!("Estimated Token Reduction: ~{} tokens/request", saved_tokens).green().bold());
        return Ok(());
    }

    fs::write(path, fixed_content)?;
    println!("{}", format!("✅ Successfully optimized {} (Saved ~{} tokens per agent turn).", path.display(), saved_tokens).green().bold());
    Ok(())
}

pub fn cmd_stats(path: &Path, project_root: &Path) -> anyhow::Result<()> {
    let report = lint_file(path, project_root)?;
    println!("\n{}", format!("=== Agent Instruction Quality Score: {} ===", path.display()).cyan().bold());

    let mut score: i32 = 100;
    score -= (report.errors_count as i32) * 20;
    score -= (report.warnings_count as i32) * 10;
    score -= (report.info_count as i32) * 5;
    let score = score.clamp(0, 100);

    let grade = match score {
        90..=100 => "A+ (Production Ready)",
        75..=89 => "B (Acceptable, minor bloat)",
        50..=74 => "C (Prone to agent hallucination)",
        _ => "F (Critical instruction decay)",
    };

    println!("Quality Score:  {}/100", score.to_string().bold());
    println!("Health Grade:   {}", grade.bold());
    println!("Token Footprint: ~{} lines / {} findings\n", report.total_lines, report.findings.len());
    Ok(())
}

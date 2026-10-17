use crate::domain::InstructionBlock;
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use std::collections::HashSet;

/// Parse raw markdown into structured instruction blocks and extract referenced code paths.
pub fn parse_instruction_markdown(content: &str) -> Vec<InstructionBlock> {
    let mut blocks = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    let mut current_heading: Option<String> = None;
    let mut in_heading = false;
    let mut heading_buf = String::new();

    let parser = Parser::new(content);
    let mut current_paths = HashSet::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { .. }) => {
                in_heading = true;
                heading_buf.clear();
            }
            Event::End(TagEnd::Heading(_)) => {
                in_heading = false;
                current_heading = Some(heading_buf.trim().to_string());
            }
            Event::Text(t) => {
                if in_heading {
                    heading_buf.push_str(&t);
                }
            }
            Event::Code(c) => {
                let s = c.trim();
                if is_plausible_file_path(s) {
                    current_paths.insert(s.to_string());
                }
            }
            _ => {}
        }
    }

    // Now scan lines directly to preserve line number fidelity
    for (idx, line) in lines.iter().enumerate() {
        let line_num = idx + 1;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let mut line_paths = Vec::new();
        // Extract backticked substrings
        let mut in_backtick = false;
        let mut current_buf = String::new();
        for ch in line.chars() {
            if ch == '`' {
                if in_backtick {
                    let candidate = current_buf.trim();
                    if is_plausible_file_path(candidate) {
                        line_paths.push(candidate.to_string());
                    }
                    current_buf.clear();
                    in_backtick = false;
                } else {
                    in_backtick = true;
                }
            } else if in_backtick {
                current_buf.push(ch);
            }
        }

        blocks.push(InstructionBlock {
            line_start: line_num,
            line_end: line_num,
            heading: current_heading.clone(),
            text: line.to_string(),
            backticked_paths: line_paths,
        });
    }

    blocks
}

/// Identifies if a string resembles a project file path, script, or directory.
pub fn is_plausible_file_path(s: &str) -> bool {
    let s = s.trim_matches(|c| c == '\'' || c == '"' || c == '`' || c == ',' || c == ':');
    if s.is_empty() || s.contains(' ') || s.starts_with("http://") || s.starts_with("https://") {
        return false;
    }

    // Must contain a directory slash or typical file extension
    let has_slash = s.contains('/') || s.contains('\\');
    let has_ext = s.ends_with(".rs")
        || s.ends_with(".ts")
        || s.ends_with(".tsx")
        || s.ends_with(".js")
        || s.ends_with(".jsx")
        || s.ends_with(".py")
        || s.ends_with(".json")
        || s.ends_with(".toml")
        || s.ends_with(".yaml")
        || s.ends_with(".yml")
        || s.ends_with(".md")
        || s.ends_with(".sh")
        || s.ends_with(".sql")
        || s.ends_with(".c")
        || s.ends_with(".cpp")
        || s.ends_with(".h");

    (has_slash || has_ext) && !s.starts_with('-') && !s.starts_with('$')
}

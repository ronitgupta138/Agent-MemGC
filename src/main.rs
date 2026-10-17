use clap::{Parser, Subcommand};
use agent_memgc::cli::*;
use agent_memgc::mcp::McpServer;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "agent-memgc",
    author = "Ronit Gupta <ronitgupta138@gmail.com>",
    version,
    about = "High-performance linter, cache-layout optimizer, and garbage collector for AI agent instruction files (AGENTS.md, CLAUDE.md, .cursorrules)",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Lint instruction file for dead paths, semantic contradictions, and cache invalidation
    Check {
        /// Path to instruction file (e.g. AGENTS.md, CLAUDE.md, .cursorrules)
        file: PathBuf,
        /// Project root directory to check path validity against (default: .)
        #[arg(long, default_value = ".")]
        root: PathBuf,
        /// Output findings as raw JSON
        #[arg(long, default_value_t = false)]
        json: bool,
    },

    /// Automatically clean, deduplicate, and optimize instruction file for prompt caching
    Fix {
        /// Path to instruction file
        file: PathBuf,
        /// Project root directory (default: .)
        #[arg(long, default_value = ".")]
        root: PathBuf,
        /// Show diff without modifying file on disk
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },

    /// Display instruction file quality score and health grade
    Stats {
        /// Path to instruction file
        file: PathBuf,
        /// Project root directory (default: .)
        #[arg(long, default_value = ".")]
        root: PathBuf,
    },

    /// Launch Model Context Protocol (MCP) server over stdio for Claude, Hermes, and Cursor
    Mcp,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { file, root, json } => {
            let ok = cmd_check(&file, &root, json)?;
            if !ok {
                std::process::exit(1);
            }
        }
        Commands::Fix { file, root, dry_run } => {
            cmd_fix(&file, &root, dry_run)?;
        }
        Commands::Stats { file, root } => {
            cmd_stats(&file, &root)?;
        }
        Commands::Mcp => {
            let server = McpServer::new();
            server.run_stdio().await?;
        }
    }

    Ok(())
}

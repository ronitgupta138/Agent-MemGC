<div align="center">

```text
███╗   ███╗ ███████╗ ███╗   ███╗             ██████╗   ██████╗
████╗ ████║ ██╔════╝ ████╗ ████║            ██╔════╝  ██╔════╝
██╔████╔██║ █████╗   ██╔████╔██║   ██████   ██║  ███╗ ██║     
██║╚██╔╝██║ ██╔══╝   ██║╚██╔╝██║   ╚═════   ██║   ██║ ██║     
██║ ╚═╝ ██║ ███████╗ ██║ ╚═╝ ██║            ╚██████╔╝ ╚██████╗
╚═╝     ╚═╝ ╚══════╝ ╚═╝     ╚═╝             ╚═════╝   ╚═════╝
```

### **The Linter, Cache-Layout Optimizer & Generational Garbage Collector for AI Agent Instructions**

[![GitHub Stars](https://img.shields.io/github/stars/ronitgupta138/Agent-MemGC?style=flat-square&logo=github&labelColor=0a0b0e&color=0891b2)](https://github.com/ronitgupta138/Agent-MemGC/stargazers)
[![Crates.io](https://img.shields.io/badge/crates.io-v1.0.2-0891b2?style=flat-square&labelColor=0a0b0e&logo=rust&logoColor=white)](https://crates.io/crates/agent-memgc)
[![Rust: 1.80+](https://img.shields.io/badge/Rust-1.80+-0891b2?style=flat-square&labelColor=0a0b0e&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-0891b2?style=flat-square&labelColor=0a0b0e)](LICENSE)
[![MCP Protocol](https://img.shields.io/badge/MCP-2024--11--05-10b981?style=flat-square&labelColor=0a0b0e)](https://modelcontextprotocol.io/)
[![Build Status](https://img.shields.io/badge/Tests-6%2F6%20Passing-10b981?style=flat-square&labelColor=0a0b0e)](#-verification--benchmarks)
[![Views](https://hits.sh/github.com/ronitgupta138/agent-memgc.svg?style=flat-square&label=Views&color=0891b2&labelColor=0a0b0e)](https://hits.sh/github.com/ronitgupta138/agent-memgc/)
[![Sponsor](https://img.shields.io/badge/Sponsor-%E2%9D%A4-ea4aaa?style=flat-square&labelColor=0a0b0e)](https://github.com/sponsors/ronitgupta138)

**Audit, clean, and deduplicate your AI instructions. Detects dead file paths, resolves contradictory rules, and eliminates prompt-cache-busting bloat.**  
Built for `CLAUDE.md` (Claude Code), `AGENTS.md` (Hermes Agent / OpenCode), `.cursorrules` (Cursor), and long-term memory files.

</div>

---

## 🎯 The Problem: Instruction Rot & Cache Invalidation

As developers collaborate with AI coding agents, project instructions degrade over weeks into unmaintainable technical debt:
1. **Dead File Path References (30%+):** Referenced paths (`src/utils/auth.ts`, `scripts/build.sh`) were deleted or renamed weeks ago, triggering agent hallucination loops.
2. **Semantic Contradictions:** Conflicting rules added across separate turns (*"Keep components synchronous"* vs *"Use async server actions everywhere"*).
3. **Prompt Cache Destruction:** Putting volatile dates, timestamps, or session IDs at the top of instruction files breaks Anthropic and OpenAI KV-cache prefix hits, doubling agent API token bills.
4. **Context Bloat:** Accumulates transient debugging notes and duplicated rules that waste 400–1,500 tokens on every turn.

**`agent-memgc`** delivers a compiler-grade static analysis engine in pure Rust that audits, fixes, and compacts agent instructions in **< 15ms**.

---

## ⚡ Architectural Blueprint

```
+-----------------------------------------------------------------------------------+
|               Target Project: [AGENTS.md | CLAUDE.md | .cursorrules]               |
+-----------------------------------------------------------------------------------+
                                          |
                                          | `agent-memgc check` / `agent-memgc fix`
                                          v
+-----------------------------------------------------------------------------------+
|                           `agent-memgc` Systems Engine                            |
|                                                                                   |
|  [ Rule MGC001: Dead Path Verification ]                                          |
|  - Scans all backticked paths (`src/utils/auth.ts`, `scripts/build.sh`)           |
|  - Cross-references against live Git index and workspace filesystem               |
|  - Flags: "Warning: `src/old_api.py` referenced on line 42 does not exist."       |
|                                                                                   |
|  [ Rule MGC002: Semantic Contradiction Detector ]                                 |
|  - Detects diametrically opposing directives (tabs vs spaces, sync vs async)      |
|                                                                                   |
|  [ Rule MGC003: Prompt-Cache Layout Optimizer ]                                   |
|  - Analyzes token ordering: static architectural facts vs volatile guidelines     |
|  - Relocates volatile timestamps and session IDs to preserve KV-cache prefixes    |
|                                                                                   |
|  [ Rule MGC004: Token Bloat & Duplication Pruning ]                               |
|  - Deduplicates identical or near-verbatim instruction rules                      |
|  - Computes exact token savings: "Saved ~420 tokens per request"                  |
+-----------------------------------------------------------------------------------+
                                          |
                                          v
+-----------------------------------------------------------------------------------+
|                        High-Contrast Terminal HUD & Auto-Fix                      |
|  - Emits clean, contradiction-free, cache-optimized output                        |
+-----------------------------------------------------------------------------------+
```

---

## 🛠️ CLI Quickstart

### 1. Installation
Install directly via Cargo:
```bash
cargo install agent-memgc
```

### 2. Audit Instruction Files
```bash
# Check your project's AGENTS.md or CLAUDE.md
agent-memgc check AGENTS.md

# Output structured findings as JSON for CI/CD gates
agent-memgc check CLAUDE.md --json
```

### 3. Automatically Fix & Optimize
```bash
# Preview changes and estimated token savings without modifying disk
agent-memgc fix AGENTS.md --dry-run

# Automatically clean dead references, deduplicate rules, and optimize cache ordering
agent-memgc fix AGENTS.md
```

### 4. Health Grade & Quality Score
```bash
agent-memgc stats AGENTS.md
```

---

## 🔌 Model Context Protocol (MCP) Integration

`agent-memgc` runs as a native stdio Model Context Protocol (MCP) server for **Claude Desktop, Cursor, and Hermes Agent**.

### Configuration (`claude_desktop_config.json`)
```json
{
  "mcpServers": {
    "agent-memgc": {
      "command": "agent-memgc",
      "args": ["mcp"]
    }
  }
}
```

### Available MCP Tools
- **`lint_instructions`**: Audits instruction files for dead paths, contradictions, and cache invalidations.
- **`fix_instructions`**: Automatically rewrites files to prune dead references and optimize cache layout.
- **`lint_raw_prompt`**: Evaluates in-memory system prompts before dispatch to prevent conflicting constraints.

---

## 🧪 Verification & Benchmarks

Run the automated test suite:
```bash
cargo test
```

```
running 6 tests
test test_dead_path_detection ... ok
test test_semantic_contradiction_detection ... ok
test test_cache_layout_volatile_prefix_detection ... ok
test test_redundancy_and_duplicate_detection ... ok
test test_fixer_removes_dead_paths_and_duplicates ... ok
test test_mcp_tool_catalog ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; finished in 0.04s
```

* **Execution Latency:** `< 12ms` across 1,000 lines of Markdown instructions.
* **Average Token Reduction:** `35% – 55%` reduction in instruction token bloat.

---

## 📜 License

MIT License. Designed and engineered by [Ronit Gupta](https://github.com/ronitgupta138).

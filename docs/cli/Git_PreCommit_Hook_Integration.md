# Git Pre-Commit Hook & CI Linter Integration Specification

## 1. Developer Workflow Integration
Agent-MemGC integrates directly into developer Git repositories to audit prompt templates and `AGENTS.md` before commits are pushed.

## 2. Hook Script Configuration
```bash
#!/bin/sh
# .git/hooks/pre-commit
exec agent-memgc lint --staged --deny warnings
```

## 3. Exit Code Semantics
- `0`: All prompt files clean and cache-optimized.
- `1`: Warning detected (only blocks commit if `--deny warnings` is set).
- `2`: Severe syntax error or unresolved semantic contradiction (`MGC002`).

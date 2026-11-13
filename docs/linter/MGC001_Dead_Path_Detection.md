# Static Rule Specification: MGC001 — Dead Instruction Path Detection

## 1. Rule Identification
- **Rule ID:** `MGC001`
- **Category:** Instruction Linter / Dead Code Elimination
- **Severity:** Warning
- **Target:** System prompt conditional clauses and tool calling triggers

## 2. Detection Heuristic
Detects instructions guarded by mutually exclusive preconditions or references to tools and roles that do not exist in the active agent configuration.
```markdown
<!-- Triggers MGC001: tool 'sql_exec' is absent from active MCP server list -->
If executing a database query, you must invoke the `sql_exec` tool.
```

## 3. Automated Remediation
Prunes the dead instruction subtree from the system prompt prior to LLM submission, eliminating redundant tokens and preventing hallucinated tool calls.

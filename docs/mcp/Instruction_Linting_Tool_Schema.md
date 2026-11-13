# Model Context Protocol (MCP) Instruction Linter Tool Schema Specification

## 1. Tool Declaration
Agent-MemGC exposes an MCP tool allowing autonomous agents to self-lint their system instructions:

```json
{
  "name": "memgc_lint_instructions",
  "description": "Lints system prompt instructions for dead code, cache invalidations, and contradictions",
  "inputSchema": {
    "type": "object",
    "properties": {
      "prompt_text": { "type": "string" },
      "active_tools": { "type": "array", "items": { "type": "string" } }
    },
    "required": ["prompt_text"]
  }
}
```

## 2. Output Diagnostics Envelope
Returns structured diagnostics including rule violation code, line numbers, character offsets, and auto-fix suggestions.

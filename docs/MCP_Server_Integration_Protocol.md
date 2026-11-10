# Model Context Protocol (MCP) Server Integration Protocol

## 1. Protocol Architecture
Agent-MemGC exposes a native Model Context Protocol (MCP) server over standard input/output (stdio) using JSON-RPC 2.0 messages. This allows seamless integration into agent runtimes like Hermes, Claude Code, and Cursor.

## 2. Core Tool Definitions
- `lint_prompt_context`:
  - **Inputs:** `context` (string), `model_target` (string, e.g. "claude-3-5-sonnet", "deepseek-v3")
  - **Outputs:** Lint diagnostics array containing rule violations, token waste metrics, and duplicate instruction offsets.
- `optimize_cache_layout`:
  - **Inputs:** `system_prompt` (string), `tools_schema` (string), `history` (array of messages)
  - **Outputs:** Reordered prompt structure optimized for prefix-hash caching on target provider architectures.
- `prune_transient_tokens`:
  - **Inputs:** `conversation_turns` (array), `target_token_budget` (integer)
  - **Outputs:** Pruned message history with Generation-0 tool noise safely truncated.

## 3. Error Handling & Protocol Compliance
- Returns standard JSON-RPC 2.0 error codes (`-32600 Invalid Request`, `-32602 Invalid params`).
- Zero unhandled panics on malformed input payloads.

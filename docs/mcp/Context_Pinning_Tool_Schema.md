# MCP Context Pinning Tool Schema and Immune Memory Tags

## 1. Tool Specification
- `pin_memory_block`:
  - **Inputs:** `block_id` (string), `reason` (string), `ttl_turns` (integer, optional)
  - **Behavior:** Marks the targeted prompt section with an immunity flag preventing garbage collection eviction during active conversation compaction passes.

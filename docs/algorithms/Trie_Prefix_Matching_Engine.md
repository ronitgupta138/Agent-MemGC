# Compact Radix Trie String Indexing for Prompt Prefix Matching

## 1. Overview
Agent-MemGC maintains a compact Radix Trie (PATRICIA tree) over all active system rules and cached prompt preambles:
- Reduces string matching complexity to $O(k)$ where $k$ is the prefix token length.
- Enables sub-millisecond lookup across thousands of known instruction patterns.

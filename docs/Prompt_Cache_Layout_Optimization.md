# Provider KV Prompt Cache Block Alignment Rules

## 1. Overview
Modern LLM inference engines (Anthropic, OpenAI, DeepSeek, Google) employ prefix KV cache reuse to dramatically lower latency and inference billing. Slight perturbations in the leading prompt tokens invalidate downstream cache pages.

## 2. Provider Block Boundary Mechanics
- **Anthropic Claude:**
  - Minimum cacheable prefix: 1,024 tokens.
  - Checkpoint granularity: Explicit `cache_control: {"type": "ephemeral"}` markers placed at boundary edges.
  - Reordering Rule: Place static system instructions first, followed by static tool declarations, keeping dynamic conversation turns strictly at the tail.
- **OpenAI / Codex:**
  - Automatic prefix caching for requests >= 1,024 tokens in 128-token increment pages.
  - Invariant: Dynamic dates or timestamps must NEVER appear at the prompt head.
- **DeepSeek V3 / R1:**
  - Server-side automatic 64-token block caching.
  - Invariant: Zero whitespace churn or parameter shuffling in tool call JSON schemas.

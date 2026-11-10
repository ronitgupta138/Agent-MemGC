# Generational Context Pruning Specification

## 1. Abstract
Agent-MemGC implements a two-generation memory garbage collection model for long-running LLM agent conversations. By partitioning system instructions and tool outputs into Young and Tenured generations, the collector safely reclaims transient tokens without breaking long-term semantic coherence.

## 2. Generational Partitioning
- **Generation 0 (Young / Ephemeral):**
  - Tool execution outputs, intermediate scratchpad deliberations, temporary directory listings, and raw API responses.
  - Collector Policy: Aggressive pruning on every N turns (default: 5 turns) using extractive summarization or bounded head/tail retention.
- **Generation 1 (Tenured / Durable):**
  - User identity rules, foundational system prompt constraints, active project invariants, and explicit memory items.
  - Collector Policy: Immutable throughout the active conversation; only subject to compaction during full context overflow recovery.

## 3. Pruning Invariants & Performance Bounds
- Total garbage collection parse and scan latency must not exceed 15ms for prompt contexts up to 128k tokens.
- Token reduction ratio target: 40%–60% reduction in transient tool noise while preserving 100% of tenured instruction blocks.

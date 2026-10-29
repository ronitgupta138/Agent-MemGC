# Prompt Cache Prefix Alignment Strategy

## Overview
Technical specification for **Agent-MemGC** systems engine.

## Specification Details
Aligning immutable system prompts to 1024-token boundaries to maximize Anthropic/OpenAI cache hits.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

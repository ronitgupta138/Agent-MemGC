# Sliding Window Context Eviction

## Overview
Technical specification for **Agent-MemGC** systems engine.

## Specification Details
FIFO eviction prioritizing retention of system rules, active tool state, and user invariants.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

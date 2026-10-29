# Agent Tool Schema Compaction Rules

## Overview
Technical specification for **Agent-MemGC** systems engine.

## Specification Details
Pruning redundant JSON schema descriptions to minimize tool catalog token consumption.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

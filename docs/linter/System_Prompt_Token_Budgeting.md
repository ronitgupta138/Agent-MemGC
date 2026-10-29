# System Prompt Token Budget Allocation

## Overview
Technical specification for **Agent-MemGC** systems engine.

## Specification Details
Deterministic budget partitioning reserving 20% for rules, 50% for context, and 30% for output.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

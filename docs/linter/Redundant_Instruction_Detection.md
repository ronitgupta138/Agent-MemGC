# Redundant Instruction Detection Rules

## Overview
Technical specification for **Agent-MemGC** systems engine.

## Specification Details
Static analysis rules finding repeated imperatives and conflicting formatting guidelines in prompts.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

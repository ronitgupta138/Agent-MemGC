# Compaction Pipeline Execution Stages

## Overview
Technical specification for **Agent-MemGC** systems engine.

## Specification Details
Three-stage pipeline: Tokenization -> AST Parse -> Rule Filter -> Linear Reassembly.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

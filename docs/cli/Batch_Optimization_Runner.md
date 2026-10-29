# Batch Optimization Runner CLI

## Overview
Technical specification for **Agent-MemGC** systems engine.

## Specification Details
CLI command batch-optimizing all SKILL.md and system instruction files across a repo.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

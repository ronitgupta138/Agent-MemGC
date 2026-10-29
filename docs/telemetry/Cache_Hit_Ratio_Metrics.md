# Cache Hit Ratio Tracking Metrics

## Overview
Technical specification for **Agent-MemGC** systems engine.

## Specification Details
Prometheus counter tracking prompt prefix cache hits vs misses across consecutive API turns.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

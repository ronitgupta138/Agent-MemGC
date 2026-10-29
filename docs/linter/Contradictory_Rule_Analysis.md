# Contradictory Rule Analysis Engine

## Overview
Technical specification for **Agent-MemGC** systems engine.

## Specification Details
Bipartite graph model detecting contradictory constraints across loaded skill definitions.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

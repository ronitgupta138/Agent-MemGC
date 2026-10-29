# Interactive Prompt Analyzer CLI

## Overview
Technical specification for **Agent-MemGC** systems engine.

## Specification Details
Terminal UI visualizing token heatmaps, duplicate instructions, and cache alignment markers.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

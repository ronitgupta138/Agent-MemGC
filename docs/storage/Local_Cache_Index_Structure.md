# Local Cache Index File Structure

## Overview
Technical specification for **Agent-MemGC** systems engine.

## Specification Details
Binary index header format mapping prompt hashes to cached token offsets on local disk.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

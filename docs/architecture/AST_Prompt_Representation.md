# Abstract Syntax Tree for Prompts

## Overview
Technical specification for **Agent-MemGC** systems engine.

## Specification Details
Typed AST nodes representing Markdown headers, lists, code blocks, and imperative directives.

## Invariants & Design Guarantees
- Zero-cost abstractions where possible.
- Safe Rust memory semantics without undefined behavior (`unsafe` strictly audited).
- Deterministic behavior under concurrent thread interleavings.

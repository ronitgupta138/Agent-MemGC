# Instruction Abstract Syntax Tree (AST) Memory Layout Specification

## 1. Tree Representation in Rust
Agent-MemGC converts unstructured Markdown prompts into a typed abstract syntax tree:

```rust
pub enum InstructionNode {
    Section { title: String, level: u8, children: Vec<InstructionNode> },
    Constraint { priority: u8, directive: String },
    ToolRequirement { tool_name: String, precondition: String },
    VariableInterpolation { key: String, dynamic: bool },
}
```

## 2. Memory Compactness
Allocates AST nodes using an arena allocator (`typed-arena`), achieving zero heap fragmentation and sub-microsecond traversal speeds across large prompts.

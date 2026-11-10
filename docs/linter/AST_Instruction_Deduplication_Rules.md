# AST-Based Instruction Deduplication and Contradiction Rules

## 1. Syntax Parsing
Parses Markdown and structured XML prompt tags (`<system>`, `<rules>`, `<instructions>`) into an Abstract Syntax Tree (AST).

## 2. Deduplication Heuristics
- Normalizes imperative directives (e.g. "Always respond in English" vs "Respond only in English").
- Identifies exact and semantic near-duplicates, collapsing multi-line redundant rules into canonical directives.

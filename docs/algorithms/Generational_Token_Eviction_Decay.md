# Generational Garbage Collection & Token Eviction Decay Specification

## 1. Generational Memory Hypothesis for LLM Agents
Like JVM heap objects, agent context elements exhibit bimodal lifespans: most intermediate tool results (e.g. massive directory listings) become obsolete immediately, while core objectives persist indefinitely.

## 2. Generational Partitioning
- **Gen0 (Ephemeral Scratch):** Raw stdout from shell commands, file search results. Evicted after 2 conversation turns.
- **Gen1 (Active Working Set):** Current task plan, recent user clarifications, modified file diffs. Evicted after task milestone completion.
- **Gen2 (Permanent Invariants):** Project conventions, repository rules, architectural design patterns. Retained across entire session.

## 3. Decay Scoring Equation
$$\text{Relevance}(t) = W_{\text{type}} \cdot e^{-\lambda (T_{\text{current}} - T_{\text{turn}})}$$

# Sliding Window Token Eviction Heuristics

## 1. Mathematical Scoring
Each conversational token block $B_i$ receives a dynamic retention score:
$$S(B_i) = W_{\text{role}} \cdot e^{-\lambda (T_{\text{now}} - T_i)} + \mu \cdot \text{CitationCount}(B_i)$$
- Blocks scoring below threshold $\theta_{\text{prune}}$ are compacted or replaced by semantic digest tokens.

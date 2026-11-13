# Prompt Cache Hit Ratio & Financial Savings Metrics Specification

## 1. Metric Instrumentation
Agent-MemGC exports metrics tracking prompt optimization efficacy:

- `memgc_tokens_pruned_total`: Cumulative input tokens eliminated by dead code rules.
- `memgc_cache_hit_rate`: Ratio of cached tokens to total prompt tokens.
- `memgc_estimated_cost_saved_usd`: Financial savings computed against target model rate card.

## 2. Verification Invariant
Prompts reorganized according to `MGC003` cache alignment must demonstrate $\ge 90\%$ steady-state KV cache hit rates on iterative multi-turn tasks.

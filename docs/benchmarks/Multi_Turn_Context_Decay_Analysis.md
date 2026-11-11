# Multi-Turn Context Decay and Prompt Bloat Benchmark Analysis

## 1. Empirical Findings (50-Turn Coding Workload)
- **Unmanaged Context:** Token volume grows monotonically from 2,000 to 86,000 tokens (43x bloat).
- **With Agent-MemGC:** Steady-state context fluctuates between 4,000 and 12,000 tokens (85% sustained token reduction).

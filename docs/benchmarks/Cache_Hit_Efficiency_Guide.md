# Cache Hit Ratio Benchmarks and TTFT Measurement Guide

## 1. Benchmarking Methodology
This document defines standard protocols for evaluating prompt cache efficiency across LLM agent workloads:
- **Metrics Tracked:**
  - Cache Read Tokens vs Cache Write Tokens.
  - Time To First Token (TTFT) reduction percentage.
  - Cost savings coefficient ($/1k tokens).

## 2. Experimental Results (100-Turn Coding Workload)
- **Baseline (Unoptimized Layout):**
  - Average TTFT: 1,840ms
  - Cache Hit Ratio: 12.4%
- **Optimized (Agent-MemGC Reordered Prefix):**
  - Average TTFT: 340ms (81.5% reduction)
  - Cache Hit Ratio: 91.8%

# Static Rule Specification: MGC003 — Prompt Cache Alignment Heuristics

## 1. Rule Identification
- **Rule ID:** `MGC003`
- **Category:** Prompt Caching / Cost Optimization
- **Severity:** Optimization Warning
- **Target:** Dynamic prefix mutations breaking KV cache hits

## 2. KV Cache Invalidation Mechanism
Frontier models (Anthropic, DeepSeek, OpenAI) cache prompt prefixes in 1024-token blocks. Injecting dynamic timestamps or volatile variables at the top of a prompt invalidates all subsequent cached tokens.

## 3. Structural Reordering Guideline
```markdown
[IMMUTABLE BASE PROMPT]       <-- Cached (100% Cache Hit)
[SYSTEM CAPABILITIES & TOOLS] <-- Cached (100% Cache Hit)
[CONVERSATION HISTORY]         <-- Rolling Cache
[DYNAMIC PER-TURN VARIABLES]   <-- Placed at absolute suffix
```

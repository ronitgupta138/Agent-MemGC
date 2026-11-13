# Static Rule Specification: MGC002 — Semantic Contradiction Analysis

## 1. Rule Identification
- **Rule ID:** `MGC002`
- **Category:** Behavioral Coherence / Semantic Conflict
- **Severity:** Error
- **Target:** Opposing directives in system instructions

## 2. Conflict Topology
Identifies diametric contradictions resulting from combining base instructions with ad-hoc agent skills:
- *Directive A:* "Always provide concise, one-sentence responses without explanation."
- *Directive B:* "Include exhaustive line-by-line code annotations and comprehensive reasoning."

## 3. Resolution Priority Hierarchy
1. Explicit active task instruction (Highest priority).
2. Domain-specific skill instructions.
3. Base agent persona instructions (Lowest priority).

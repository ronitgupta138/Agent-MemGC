# Static Rule Specification: MGC004 — Redundant Instruction Deduplication

## 1. Rule Identification
- **Rule ID:** `MGC004`
- **Category:** Token Economy / Deduplication
- **Severity:** Info / Auto-Fixable
- **Target:** Duplicate behavioral constraints repeated across multiple skills

## 2. Semantic Similarity Matching
Uses normalized Levenshtein and MinHash jaccard similarity across sentence vectors:

$$\text{Jaccard}(S_1, S_2) = \frac{|T_1 \cap T_2|}{|T_1 \cup T_2|} > 0.85$$

## 3. Deduplication Action
Collapses repeated rules (e.g. "Do not use markdown code blocks without language tags" appearing in 4 different skill files) into a single canonical entry in the system preamble.

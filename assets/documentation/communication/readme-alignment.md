# README Communication Alignment

> **Historical audit note:** this document records an earlier communication-alignment pass. Brand Bible v1.0 and live generated repository status supersede its volatile release snapshot. Do not treat version numbers below as current unless revalidated.

This document records recommended future README alignment. It intentionally does **not** rewrite `README.md`.

Audit basis: repository state inspected on **2026-09-14**.

## Already aligned

The current README already carries most of the canonical communication system well:

- public identity: **OX-DX**;
- descriptor: **Universal Ontology & Experience Engine**;
- implementation distinction: **Universal Ontology Engine**;
- evidence-first framing;
- “resolution spine over a graph” rather than mandatory hierarchy;
- native evidence and read-only discovery;
- provenance ≠ identity;
- observation ≠ ownership;
- representation ≠ meaning;
- Phase 1–6 hardened / Phase 7 gated / later phases planned;
- Quranic inspiration separated from technical specification.

No broad README rewrite is recommended.

## Recommended future alignment

### 1. Adopt the canonical one-line definition consistently

The opening question and current explanation are accurate. Future edits should converge on the one-line definition in `messaging-framework.md` / `one-liners.md` rather than creating new descriptions.

### 2. Keep “Experience” explicitly conceptual

The README already says the primary tagline is a direction, not a claim of complete implementation. Preserve this boundary in every future edit.

### 3. Keep the Quranic boundary concise and consistent

The existing section correctly says inspiration does not define the engineering contract. Future wording should remain compatible with `quranic-inspiration.md`: philosophically inspired, independently specified and testable.

### 4. Do not manually edit generated status blocks

The brand-system branch inherited an auto-generated block reporting stable `v0.1.3` and dev `v0.1.3-dev.9`.

During the final audit, GitHub Releases advanced again: stable **v0.1.4** was published, the latest preceding prerelease was **v0.1.4-dev.15**, and `dev/Cargo.toml` reports **0.1.4** after promotion.

This is a synchronization concern, not a prose-edit task. Let the repository automation regenerate the status block after branch synchronization/promotion. Do not hardcode a replacement in human-authored README text.

### 5. Avoid status duplication outside generated sections

Current phase/status tables are useful, but future changes should avoid adding additional hand-maintained copies of volatile release metadata. Prefer a single generated status source plus stable conceptual explanation.

### 6. Keep compiler/runtime claims bounded

Preserve these current distinctions:
- Rust AST observation uses `syn`;
- other supported languages provide deterministic syntax observation;
- cross-language adapters are not compiler-grade frontends;
- full runtime observation remains planned;
- representation readers remain planned.

### 7. Preserve source-of-truth order

README is a human entry point.

When wording conflicts:
1. normative ontology specification;
2. engineering standards;
3. implementation/tests;
4. generated current status;
5. README prose;
6. campaign/social copy.

## No changes recommended to

- repository name;
- crate names;
- ontology level names;
- CLI command names;
- graph edge kinds;
- phase definitions.

The README is already directionally strong. The communication system should reduce future drift, not trigger another rewrite.

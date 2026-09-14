# OX-DX Messaging Framework

This file defines the canonical hierarchy of public explanation. Use the shortest level that gives the audience enough truth.

The canonical voice remains `../brand-voice.md`. Canonical technical definitions remain in `terminology.md`.

## Level 1 — one line

**OX-DX is a Universal Ontology & Experience Engine that resolves heterogeneous evidence through a canonical ontology and typed graph without forcing native systems into one artificial shape.**

Use for repository descriptions, introductions, bios, and opening copy.

## Level 2 — short

OX-DX gives heterogeneous software evidence a shared ontology and typed graph while preserving native structure, provenance, and explicit relations. The current Rust engine focuses on deterministic, read-only discovery and syntax evidence; deeper semantic, runtime, and representation capabilities remain gated or planned.

Use when two or three sentences are available.

## Level 3 — standard

OX-DX is built around evidence-first resolution. The Universal Ontology provides 49 canonical types as a resolution spine over a graph; the graph distinguishes structural containment from projection, observation, representation, and other relations. Native evidence remains authoritative, provenance is not identity, and missing intermediate levels may remain unmaterialized. The Rust implementation currently validates the registry, enforces graph invariants, discovers software structure read-only, and records deterministic Rust and cross-language syntax evidence.

Use in README introductions, documentation overviews, project pages, and technical summaries.

## Level 4 — deep

For technical audiences, explain the layers in this order:

1. **Normative vocabulary** — `../../../specifications/universal-ontology-v1.0.md`.
2. **Engineering invariants** — `../../../standards/universal-ontology-engine.md`.
3. **Architecture** — `../../../docs/architecture.md`.
4. **Implementation** — seven Rust workspace crates under `../../../crates/`.
5. **CLI evidence surface** — `ontology-engine validate`, `levels`, `inspect`, `discover`, `parse`, and `self`.
6. **Repository state** — current release metadata and generated status, not static marketing prose.

Do not describe planned phases as current capability.

## Language of truth

**Never claim more than the evidence supports.**

Prefer: `observed`, `identified`, `represented`, `mapped`, `projected`, `connected`, `derived`, `supported`, `verified`, `traced`.

Avoid claims that the system “knows reality,” “proves everything,” “discovers ultimate truth,” “understands everything,” or “explains everything.”

## Language of evidence

A filesystem entry is evidence. A manifest is evidence. A source construct is evidence. A syntax observation is evidence. A runtime event can be evidence. A source span is provenance.

Evidence can support classification. Evidence cannot justify arbitrary invention.

### Good

> Rust AST observation identified a named function at this source span; the observation supports an ELEMENT/FUNCTION projection.

### Bad

> The engine knows this function's ultimate meaning because its file path contains `service`.

### Good

> The parser emitted deterministic syntax observations for the supported language.

### Bad

> OX-DX fully understands the program.

## Public-message rule

Every public message should answer four questions, explicitly or implicitly:

- What was observed?
- What distinction matters?
- What capability is implemented now?
- What remains bounded, gated, planned, or unknown?

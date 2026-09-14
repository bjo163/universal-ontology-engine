# Universal Ontology Engine Standard

This document defines the engineering rules that keep the reference engine aligned with **Universal Ontology v1.0**.

> **Quran Inspired ALLAH my Beloved**
>
> Philosophical inspiration only. The technical ontology is not presented as a Qur'anic prescription.

## 1. Scope

The engine is a Rust implementation for discovering, normalizing, relating, and validating resources against a canonical **7 zones × 7 levels = 49 levels** resolution spine.

The ontology is an interoperability vocabulary over a graph. It is **not** a universal filesystem tree, AST schema, compiler IR, or binary layout.

## 2. Normative separation

These rules are non-negotiable:

| Concept | Meaning |
|---|---|
| `TYPE` | Exactly one canonical ontology level |
| `KIND` | Specialization/native evidence owned by a canonical `TYPE`; never a new level |
| `parent` | Structural ownership only |
| `CONTAINS` | Explicit structural ownership relation; must agree with `parent` |
| `PROJECTS_TO` | Semantic projection; never structural containment |
| `REPRESENTED_AS` | Representation/encoding relation; never semantic ownership |
| `REFERENCES`, `CAUSES`, `OBSERVED_AT`, etc. | Graph relations; never implicit tree children |
| `SourceSpan` | Provenance/evidence; never identity |

A parser, directory layout, runtime, byte sequence, or external system may provide evidence, but none of them silently redefine the ontology.

## 3. Canonical resolution model

The 49 levels are a canonical **resolution spine**:

```text
01–07   EXISTENCE
08–14   CONTEXT
15–21   INTENT
22–28   STRUCTURE
29–35   SEMANTIC
36–42   DYNAMIC
43–49   REPRESENTATION
```

Every resource does not have to materialize every level. An omitted intermediate level is valid when evidence is insufficient or the native system has no meaningful equivalent.

## 4. Phase 1–6 certification contract

### Phase 1 — Core ontology

The compiled core MUST contain exactly 49 ordered `OntologyType` values with stable numeric positions and slugs. Core identity primitives MUST be semantic values; source coordinates remain provenance.

### Phase 2 — Registry and schema

The versioned registry and JSON Schema MUST agree on:

- version and title;
- 7 zones, 7 levels per zone, 49 total levels;
- canonical ordering and zone ownership;
- canonical edge classes;
- hard rules and kind ownership.

Registry loading MUST reject contract drift.

### Phase 3 — Typed graph

Graph insertion MUST enforce:

- non-empty unique node IDs;
- valid parent existence and ascending canonical level order;
- canonical `KIND` ownership where registered;
- declared edge kinds only;
- no self-edges or duplicate edges;
- `CONTAINS` agreement with `parent`;
- semantic relations independent from structural traversal;
- deterministic storage and traversal.

### Phase 4 — Discovery

Discovery MUST be read-only and observational.

It may identify ecosystem, project, repository, and source boundaries from explicit evidence. Generic directories such as `src/` MUST NOT manufacture `UNIT` or `MODULE` nodes. Those levels remain unmaterialized unless native evidence justifies them.

Source files may be represented as `COMPONENT`/`ELEMENT` observations as an implementation bridge into syntax adapters. This is an observation boundary, not a claim that all languages share the same physical structure.

`EXECUTION` is a dynamic observation. It MUST NOT be a structural child of an `ELEMENT`; when an observation targets an element, it is represented as:

```text
EXECUTION ──OBSERVED_AT──> ELEMENT
```

### Phase 5 — Rust syntax

The Rust adapter uses native `syn` syntax and exposes canonical mapping, native construct kind, semantic name, and source span.

AST projection identity MUST NOT depend on line/column coordinates. Named observations use a parent-scoped semantic key based on canonical type + native construct kind + name, with deterministic duplicate disambiguation. Anonymous observations use deterministic ordinals. Spans remain provenance.

Native syntax kinds stay in evidence/metadata unless a kind is explicitly owned by the registry.

### Phase 6 — Cross-language syntax

Phase 6 provides deterministic normalized syntax evidence for:

```text
TypeScript · JavaScript · Python · Go · Java · Kotlin
```

Adapters MUST preserve native construct kind and source-span evidence, map only to existing canonical types, remain deterministic, and avoid inventing levels or artificial structural folders.

Phase 6 is intentionally **not** compiler-grade semantic analysis. Symbol resolution, type inference, control/data flow, semantic linking, and meaning assignment belong to Phase 7.

## 5. Evidence and provenance

Discovered nodes should remain traceable to the evidence that produced them:

```text
Node
 └─ observation / provenance
    ├─ source path or URI
    ├─ repository revision, when available
    ├─ adapter/parser
    ├─ source span or byte range
    ├─ native syntax metadata
    └─ warning/error/confidence information
```

The engine MUST distinguish **what was observed** from **what was semantically projected**.

## 6. Determinism

For a fixed input revision and environment, discovery and syntax observation MUST be deterministic.

Filesystem enumeration is never semantic order. Implementations MUST use stable ordering and deterministic duplicate disambiguation. Paths, timestamps, and line numbers may be provenance but MUST NOT be the sole identity of a semantic node.

## 7. Representation boundary

The representation levels are a graph projection path, not a universal physical chain:

```text
INSTRUCTION → EXPRESSION → VALUE → DATA → TOKEN → CHARACTER → BIT
```

`CHARACTER` requires encoding context. `BIT` is the representation boundary and must not be treated as the semantic child of a source-level construct.

Future binary/encoding readers MUST support bounded inspection, malformed-input detection, streaming operation, explicit decoding metadata, and cancellation for large inputs.

## 8. Pre-Phase-7 certification gate

Phase 7 MUST NOT begin until CI proves all of the following:

```text
registry ↔ schema consistency
        AND
no synthetic UNIT/MODULE from generic directories
        AND
EXECUTION is non-structural + OBSERVED_AT
        AND
projection does not mutate parent/containment
        AND
AST identity survives line/whitespace movement
        AND
native syntax kind remains evidence/metadata
        AND
discovery/traversal is deterministic
        AND
malformed input remains observable
```

This gate is an engineering release boundary.

## 9. Phase 7 boundary

Phase 7 may consume the evidence emitted by Phases 4–6 and create explicit semantic projections. It may not retroactively redefine the structural or identity rules above.

## 10. Canonical sources

The normative source set is:

```text
specifications/universal-ontology-v1.0.json
schemas/universal-ontology.schema.json
specifications/universal-ontology-v1.0.md
standards/universal-ontology-engine.md
```

Implementation architecture is documented separately in `docs/architecture.md`; the repository README is the concise entry point, not a second normative specification.

# Engine Architecture

Universal Ontology Engine is the executable Rust reference implementation for **Universal Ontology v1.0**.

The architecture has one central rule: the ontology provides a canonical vocabulary, while native systems provide evidence. Structure, semantics, runtime observations, and physical representation must remain distinguishable.

## 1. System model

```text
                    CANONICAL ONTOLOGY
                    7 × 7 = 49 levels
                           │
            ┌──────────────┴──────────────┐
            │                             │
      STRUCTURAL GRAPH              RELATIONAL GRAPH
            │                             │
      parent / CONTAINS       PROJECTS_TO / REFERENCES / ...
            │                             │
            └──────────────┬──────────────┘
                           │
                        EVIDENCE
                           │
             filesystem / source / syntax
                           │
                runtime / encoding / bytes
```

The 49 levels are a **resolution spine over a graph**. They do not require a 49-deep tree and do not dictate a filesystem layout.

## 2. Layer responsibilities

```text
ontology-core
    canonical types, IDs, spans, edge kinds, errors

ontology-registry
    registry loading, canonical validation, kind ownership

ontology-graph
    typed graph, containment invariants, relation invariants, deterministic traversal

ontology-discovery
    read-only workspace / repository / source discovery

ontology-rust
    native Rust AST observation via syn

ontology-language
    normalized syntax observation for TS/JS/Python/Go/Java/Kotlin

ontology-cli
    command-line access to validation, discovery, and parsing
```

No adapter owns the ontology. Adapters only produce evidence that can later be projected into it.

## 3. Structural model

Structural ownership is represented only by `parent` and `CONTAINS`.

A generic source tree may therefore look like:

```text
UNIVERSE
  └─ ECOSYSTEM
      └─ PROJECT
          └─ REPOSITORY
              └─ SOURCE
                  └─ COMPONENT
                      └─ ELEMENT
```

`UNIT`, `MODULE`, `SUBSYSTEM`, or other intermediate canonical levels may be omitted. An implementation MUST NOT create a canonical level merely because a filesystem directory has a familiar name.

A native construct may justify a lower structural level; the evidence must be explicit and the native structure remains authoritative.

## 4. Relation model

Not every relationship is containment.

```text
ELEMENT ──PROJECTS_TO──> semantic observation
EXECUTION ──OBSERVED_AT──> observed target
NODE ──REFERENCES──> NODE
NODE ──CAUSES──> NODE
NODE ──REPRESENTED_AS──> representation
```

Relation edges do not automatically become tree children. In particular, adding `PROJECTS_TO`, `OBSERVED_AT`, or `REPRESENTED_AS` MUST NOT mutate a node's structural `parent`.

## 5. Discovery boundary

`ontology-discovery` is observational and read-only.

It identifies ecosystem/project/repository/source boundaries from explicit workspace evidence, detects source files, and optionally asks language adapters for syntax observations.

Generic source directories such as `src/`, `app/`, or `lib/` are not evidence of a universal `UNIT` or `MODULE`. When evidence is insufficient, the level is intentionally unmaterialized and reported as such.

File-level `COMPONENT` and `ELEMENT` observations are an implementation bridge into syntax. They are not a claim that every language has the same native architecture.

## 6. Rust AST identity and provenance

`ontology-rust` preserves:

```text
native_kind
semantic name, when present
canonical TYPE mapping
SourceSpan
```

`SourceSpan` is provenance only.

Projection identity is semantic and parent-scoped. Named items are keyed from canonical type + native construct kind + semantic name, with deterministic duplicate disambiguation. Anonymous items use deterministic ordinals.

Therefore moving a declaration to another line must not change its semantic node ID.

## 7. Native syntax vs canonical kind

A native construct such as `struct`, `fn`, `interface`, `method`, or `async_function` is syntax evidence.

It must not be copied into canonical `Node.kind` unless that exact kind is explicitly owned by the registry. This keeps language vocabularies from colliding with the canonical `KIND` namespace.

## 8. Cross-language adapter boundary

Phase 6 currently normalizes syntax for:

```text
TypeScript   JavaScript   Python
Go           Java         Kotlin
```

The adapters:

- retain language and native construct identity;
- map only to existing canonical `TYPE`s;
- preserve source spans where available;
- use deterministic ordering;
- recognize common declaration modifiers/prefixes;
- treat malformed or unsupported input as observable evidence/errors;
- do not create ontology levels.

These are syntax adapters, not complete compiler frontends.

## 9. Runtime boundary

Runtime information belongs to the dynamic zone and is observational rather than compile-time structure.

```text
PROGRAM / SYSTEM
       │
       └─ EXECUTION
            ├─ STATE
            ├─ EVENT
            ├─ PROCESS
            ├─ FLOW
            ├─ TRANSITION
            └─ ACTION
```

An execution observation is standalone. If it targets an element or other node, the relation is explicit:

```text
EXECUTION ──OBSERVED_AT──> TARGET
```

It must never be inserted beneath the target as a containment child.

## 10. Representation boundary

Representation is also a projection, not a universal physical inheritance chain:

```text
INSTRUCTION → EXPRESSION → VALUE → DATA → TOKEN → CHARACTER → BIT
```

The actual graph path depends on the encoding or representation being observed. `CHARACTER` requires decoding context; bytes alone do not establish character semantics.

`BIT` is the lowest representation boundary. Future readers must be bounded and streaming-capable rather than assuming that an entire input can be materialized safely.

## 11. Determinism

For a fixed repository revision and environment, discovery and syntax observation must be deterministic.

Implementation requirements:

- sort filesystem entries before interpretation;
- never use line/column as the sole semantic identity;
- use deterministic duplicate disambiguation;
- keep graph storage ordered;
- preserve provenance independently of identity.

## 12. Pipeline

```text
Canonical Registry
        ↓
Core Types + Graph Invariants
        ↓
Filesystem / Git Discovery
        ↓
Rust + Cross-language Syntax Evidence
        ↓
[ PRE-PHASE-7 GATE ]
        ↓
Semantic Projection
        ↓
Runtime Observation
        ↓
Representation / Encoding
        ↓
Bounded Bit Inspection
        ↓
Query + Certification
```

## 13. Pre-Phase-7 gate

Phase 7 is opened only when CI verifies:

```text
registry ↔ schema consistency
no synthetic UNIT/MODULE from generic directories
EXECUTION is non-structural + OBSERVED_AT
projection preserves structural parent
AST IDs survive source line movement
native syntax remains evidence/metadata
repeated discovery is deterministic
malformed input remains observable
```

The gate is both an architectural boundary and a release criterion.

## 14. Documentation hierarchy

To avoid conflicting definitions:

```text
Normative ontology
  → specifications/universal-ontology-v1.0.json
  → schemas/universal-ontology.schema.json
  → specifications/universal-ontology-v1.0.md

Engineering rules
  → standards/universal-ontology-engine.md

Implementation architecture
  → docs/architecture.md

Project entry point / concise status
  → README.md
```

The README summarizes the system; it does not redefine the normative contract.

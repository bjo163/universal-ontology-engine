# Engine Architecture

Universal Ontology Engine is the executable reference implementation for Universal Ontology v1.0.

## Pre-Phase-7 invariants

Before semantic projection begins, Phases 1–6 must satisfy these boundaries:

1. `TYPE` is exactly one of the 49 canonical ontology levels.
2. `KIND` is a specialization/evidence field and never creates a new level.
3. Node identity is semantic and parent-scoped; source spans are provenance, not identity.
4. `parent` and `CONTAINS` are reserved for structural ownership.
5. `PROJECTS_TO`, `REFERENCES`, `CAUSES`, `OBSERVED_AT`, and other relation classes never create tree children.
6. Intermediate canonical levels may remain unmaterialized when native evidence is insufficient.
7. Native language syntax remains authoritative for syntax; ontology mappings are evidence/projections.
8. Discovery and parsing are read-only and deterministic for a fixed input revision/environment.

## Resolution model

```text
STRUCTURAL CONTAINMENT
Universe → ... → Repository → Source → [native Unit/Module when evidenced] → Component → Element

SEMANTIC PROJECTION
Element → Symbol / Entity / Property / Relation / Operation / Function / Behavior

RUNTIME OBSERVATION
System / Program → Execution → State / Event / Process / Flow / Transition / Action

REPRESENTATION
Instruction → Expression → Value → Data → Token → Character → Bit
```

The 49 levels form a canonical resolution spine over a graph. They do not require every resource to materialize every level, and they do not define a universal filesystem tree.

## Discovery boundary

`ontology-discovery` is observational and read-only. It identifies ecosystem, project, repository, and source boundaries from explicit workspace/repository evidence. Generic source directories such as `src/` do **not** automatically become `UNIT` or `MODULE`, because directory grouping alone is insufficient semantic evidence.

`UNIT` or `MODULE` may be materialized later when a native construct provides evidence (for example a real language/package/module declaration). Otherwise the level is omitted and reported as unmaterialized. This is valid ontology resolution, not a discovery failure.

Source files may be represented as structural `COMPONENT` boundaries and file-level `ELEMENT` observations. This is an observational bridge into syntax adapters, not a claim that every language uses the same source-file structure.

Runtime `EXECUTION` observations are standalone dynamic nodes. When a file-level execution boundary is observed, the engine uses `EXECUTION -[OBSERVED_AT]-> ELEMENT`; execution is never a structural child of an element.

## Identity and provenance boundary

Stable identity is independent of source line/column movement. For Rust AST projections, named items use a parent-scoped semantic key built from canonical type, native construct kind, and name; deterministic occurrence suffixes disambiguate duplicates. Anonymous observations use deterministic ordinals. `SourceSpan` remains provenance only.

Native syntax kind remains evidence metadata rather than a canonical `Node.kind` unless that kind is explicitly owned by the registry. This prevents a language's syntax vocabulary from silently redefining the ontology kind namespace.

## Language adapter boundary

Language adapters convert native syntax into normalized evidence. Current Phase 6 adapters cover:

```text
Rust        → ontology-rust (syn)
TypeScript  ┐
JavaScript  │
Python      │
Go          ├→ ontology-language → NormalizedSyntaxEvidence
Java        │
Kotlin      ┘
```

An adapter MUST:

- preserve native language/construct kind;
- map only to existing canonical `TYPE`s;
- retain source spans where available;
- be deterministic for the same input;
- handle common modifiers/prefixes without losing construct identity;
- treat malformed/unsupported input as observable failure/evidence;
- avoid inventing ontology levels or artificial directories.

Phase 6 is intentionally a syntax-observation boundary, not a complete compiler frontend. Semantic resolution, type inference, symbol linking, control/data-flow analysis, and authoritative meaning assignment belong to Phase 7.

## Semantic projection boundary

```text
ELEMENT
   │
   ├── PROJECTS_TO → SYMBOL / ENTITY / PROPERTY / FUNCTION / ...
   └── REPRESENTED_AS → source / lexical / encoded representation
```

A parser observation does not become a semantic assertion automatically. Phase 7 combines syntax evidence with semantic sources and emits explicit graph projections.

## Bit boundary

`BIT` is a representation endpoint. The engine must not imply that a bit is a semantic child of a source-level construct. A projection/representation edge carries that relationship.

Future binary readers must support bounded ranges, malformed-input detection, encoding/endianness metadata, and cancellation without requiring whole-file materialization.

## Current pipeline and phase gate

```text
Registry
   ↓
Core Types + Graph Invariants
   ↓
Filesystem / Git Discovery
   ↓
Language Syntax Adapters
   ↓
[PRE-PHASE-7 CERTIFICATION GATE]
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

Phase 7 must not begin until the pre-gate test suite proves structural containment, identity stability, relation separation, deterministic discovery, native syntax evidence preservation, and registry/schema consistency.

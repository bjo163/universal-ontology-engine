# Universal Ontology Engine Architecture

## Purpose

The reference engine is a Rust implementation of Universal Ontology v1.0. It discovers, normalizes, relates, validates, queries, and renders resources across the 49-level canonical resolution spine.

The foundation contract defines vocabulary and invariants; the engine defines how those invariants are observed from real systems.

## Fundamental model

```text
                         UNIVERSAL ONTOLOGY
                                │
                    canonical 49-level spine
                                │
                ┌───────────────┴───────────────┐
                │                               │
           CONTAINMENT                       GRAPH
                │                               │
        structural ownership       references / projection / events
                │                               │
                └───────────────┬───────────────┘
                                │
                          REPRESENTATION
                                │
               source → syntax → runtime → binary
                                │
                                ▼
                               BIT
```

The canonical spine is a resolution vocabulary over a graph. It is not a universal filesystem hierarchy and does not require all 49 levels to materialize for every resource.

## Non-negotiable separation

The engine MUST NOT pretend that an AST is the ontology itself. A parser yields evidence from which ontology nodes and relations can be projected.

Likewise, bytes are evidence of representation; they do not determine semantic meaning without decoding context.

`TYPE` is a canonical level. `KIND` is a specialization/evidence field owned by a canonical type and never creates another level.

`parent` and `contains` represent structural ownership only. `projects_to`, `represented_as`, `references`, `causes`, `observed_at`, and the other graph edges never create implicit tree children.

## Phase 1–6 certification contract

### Phase 1 — core

The compiled `OntologyType` set MUST contain exactly 49 ordered canonical levels with stable slugs and stable numeric ordering. Core IDs MUST be semantic values; source spans are provenance.

### Phase 2 — registry

The versioned registry and schema MUST describe the same 7×7×49 shape, canonical ordering, zone ownership, edge classes, and hard rules. Registry loading MUST reject version/shape/type/zone/rule drift.

### Phase 3 — graph

Graph insertion MUST enforce non-empty unique node IDs, valid parent ordering, canonical kind ownership where known, declared edge kinds, explicit containment agreement with `parent`, no self edges, and deterministic storage/traversal.

### Phase 4 — discovery

Discovery MUST be observational and read-only. It may recognize ecosystem/project/repository/source boundaries from explicit workspace or repository evidence. A generic `src/` or similar directory MUST NOT automatically become `UNIT` or `MODULE`. Those levels remain unmaterialized unless native evidence justifies them.

Source-file boundaries may be represented as `COMPONENT`/`ELEMENT` observations as a bridge into syntax parsing. Runtime `EXECUTION` observations are standalone dynamic nodes linked with `OBSERVED_AT`; they MUST NOT be structural children.

### Phase 5 — Rust syntax

The Rust adapter uses the native `syn` AST and preserves `native_kind`, canonical mapping, names, and spans. Syntax observations are not semantic truth.

AST projection IDs MUST NOT use source line/column as identity. Named constructs use semantic parent-scoped keys based on canonical type + native kind + semantic name, with deterministic duplicate suffixes; anonymous constructs use deterministic ordinals. Spans remain provenance.

### Phase 6 — cross-language syntax

Adapters for TypeScript, JavaScript, Python, Go, Java, and Kotlin normalize common declaration syntax while preserving native kind and source span evidence. They MUST map only to existing canonical `TYPE`s and MUST remain syntax-boundary adapters.

Phase 6 does not claim compiler-grade semantic analysis. Type checking, symbol resolution, control/data flow, meaning assignment, and cross-reference projection belong to Phase 7.

## Evidence and provenance

Every discovered lower-level node should be traceable to evidence:

```text
Node
 └── Provenance / Observation
      ├── source URI/path
      ├── repository revision when available
      ├── parser/adapter
      ├── byte/character/source span
      ├── observation metadata
      └── confidence/error information
```

The engine reports what it observed and how it derived it. It must not silently promote an observation into an asserted semantic truth.

## Adapter architecture

```text
Rust       ─┐
TypeScript ─┤
JavaScript ├─> Syntax Adapter ─> NormalizedSyntaxEvidence ─> Phase 7 projection
Python     ─┤
Go         ─┤
Java       ─┘
Kotlin      ┘

ELF / PE / Mach-O / raw binary ─> BinaryAdapter ─> RepresentationProjection
UTF-8 / UTF-16 / bytes          ─> EncodingAdapter ─> Character/BIT projection
```

External parser/encoding ecosystems are inputs to adapters, not owners of the canonical ontology vocabulary.

## Runtime layer

Runtime inspection is observation, not compile-time structure.

```text
PROGRAM / SYSTEM
  ↓
EXECUTION
  ├── STATE snapshots
  ├── EVENT occurrences
  ├── PROCESS activity
  ├── FLOW movement
  └── TRANSITION changes
```

Runtime data should include timestamps and execution context where available. An observed state/event MUST NOT overwrite source-level structural identity.

## Binary and bit layer

The representation levels are projection paths, not a universal physical stack:

```text
INSTRUCTION → EXPRESSION → VALUE → DATA → TOKEN → CHARACTER → BIT
```

Different encodings and binaries can branch into different graph paths. The engine must not force every representation through one physical chain.

## Encoding rule

`CHARACTER` is context-sensitive. The engine MUST record the encoding used to derive character information; byte sequences alone do not establish character semantics.

## Bit inspection safety

The bit reader MUST support bounded inspection, streaming rather than unbounded loading, malformed-input detection, explicit endianness/encoding metadata, cancellation for large inputs, and passive inspection without executable interpretation.

## Query model

Queries operate over type, kind, identity, edges, and provenance. Query output must report omitted/unmaterialized canonical levels instead of fabricating nodes.

## Determinism

Discovery and syntax observation MUST be deterministic for a fixed repository revision and environment. Filesystem enumeration order is never semantic order; stable sorting and deterministic duplicate disambiguation are required.

A scan result should include ontology/engine versions, input revision when available, adapter versions, node/edge counts, warnings, and errors.

## Pre-Phase-7 gate

Phase 7 may start only after CI proves:

1. the canonical registry and schema are consistent;
2. no generic directory creates fake `UNIT`/`MODULE` nodes;
3. runtime observations do not become structural children;
4. `PROJECTS_TO` preserves structural `parent` boundaries;
5. AST IDs survive whitespace/line movement;
6. native syntax kinds remain evidence/metadata;
7. malformed input remains observable failure/evidence;
8. repeated discovery produces deterministic node and edge ordering.

This gate is a release boundary, not a documentation suggestion.

## Certification target

A compliant engine can discover topological boundaries, map native structures to canonical types without forced folders, retain stable identity/provenance, represent relations independently from containment, and emit deterministic machine-readable evidence suitable for the next semantic projection phase.

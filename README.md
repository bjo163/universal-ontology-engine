# Universal Ontology Engine

> **Quran Inspired ALLAH my Beloved**
>
> Philosophical inspiration only; the technical ontology is not presented as a Qur'anic prescription.

Rust reference implementation of **Universal Ontology v1.0.0**.

## Canonical model

**7 zones × 7 levels = 49 canonical levels.** The 49-level sequence is a resolution spine over a graph, not a mandatory filesystem tree.

```text
01 UNIVERSE       08 ECOSYSTEM      15 PURPOSE        22 REPOSITORY
02 CREATION       09 ORGANIZATION   16 MISSION        23 SOURCE
03 ORDER          10 COMMUNITY      17 OBJECTIVE      24 UNIT
04 REALITY       11 REGION         18 PROGRAM        25 MODULE
05 REALM          12 ENVIRONMENT    19 PROJECT        26 SUBSYSTEM
06 WORLD          13 NETWORK        20 PRODUCT        27 COMPONENT
07 DOMAIN         14 CONTEXT        21 SYSTEM         28 ELEMENT

29 SYMBOL         36 STATE          43 INSTRUCTION
30 ENTITY         37 EVENT          44 EXPRESSION
31 PROPERTY       38 PROCESS        45 VALUE
32 RELATION       39 FLOW           46 DATA
33 OPERATION      40 TRANSITION     47 TOKEN
34 FUNCTION       41 ACTION         48 CHARACTER
35 BEHAVIOR       42 EXECUTION      49 BIT
```

## Architecture

```text
NORMATIVE REGISTRY JSON
        ↓
ontology-registry
        ↓
49-level validated registry
        ↓
ontology-core + ontology-graph
        ↓
ontology-discovery
        ↓
ontology-rust + ontology-language
        ↓
[PRE-PHASE-7 GATE]
        ↓
semantic projection
        ↓
runtime observation
        ↓
representation / encoding
        ↓
bit-level inspection
```

`TYPE` is one canonical level. `KIND` is a specialization/evidence vocabulary owned by a canonical type and never creates a new level.

`CONTAINS` and `parent` are structural ownership only. `PROJECTS_TO`, `REPRESENTED_AS`, `REFERENCES`, `CAUSES`, `OBSERVED_AT`, and the other relation classes are graph relations and never become implicit tree children.

Intermediate canonical levels may be unmaterialized. Native language structures remain authoritative and are mapped into the universal vocabulary rather than forced into artificial directories.

## Rust workspace

```text
crates/
├── ontology-core         # canonical compiled ontology primitives
├── ontology-graph        # registry-aware typed graph and invariants
├── ontology-registry     # registry loader + canonical validation
├── ontology-discovery    # read-only filesystem/workspace discovery
├── ontology-rust         # syn-based Rust AST syntax adapter
├── ontology-language     # normalized syntax adapters for TS/JS/Python/Go/Java/Kotlin
└── ontology-cli          # ontology-engine command-line interface
```

## Phase 1–6 hardening contract

Phase 1–3 provide the canonical type/edge vocabulary, registry enforcement, stable IDs, explicit containment, deterministic traversal, and registry-backed graph invariants.

Phase 4 now refuses to fabricate `UNIT`/`MODULE` from generic source directories. Those levels are materialized only with native evidence; otherwise they are explicitly reported as unmaterialized. `EXECUTION` observations are standalone dynamic nodes connected with `OBSERVED_AT`, never nested below `ELEMENT`.

Phase 5 Rust AST observations preserve native syntax kind as evidence and retain source spans as provenance. AST projection node IDs no longer depend on line/column offsets; named constructs use semantic parent-scoped keys with deterministic duplicate disambiguation.

Phase 6 adapters for TypeScript, JavaScript, Python, Go, Java, and Kotlin provide deterministic normalized syntax evidence. Common declaration modifiers/prefixes, Python async functions, Go receiver functions, and non-void Java methods are covered. These adapters are intentionally syntax-boundary adapters, not semantic analyzers or compiler replacements.

## CLI examples

```bash
cargo run -p ontology-engine -- validate
cargo run -p ontology-engine -- levels
cargo run -p ontology-engine -- inspect 49
cargo run -p ontology-engine -- discover /path/to/workspace
cargo run -p ontology-engine -- discover /path/to/workspace --rust-ast
cargo run -p ontology-engine -- discover /path/to/workspace --rust-ast --include-files --max-depth 3

cargo run -p ontology-engine -- parse rust path/to/file.rs
cargo run -p ontology-engine -- parse typescript path/to/file.ts
cargo run -p ontology-engine -- parse javascript path/to/file.js
cargo run -p ontology-engine -- parse python path/to/file.py
cargo run -p ontology-engine -- parse go path/to/file.go
cargo run -p ontology-engine -- parse java path/to/file.java
cargo run -p ontology-engine -- parse kotlin path/to/file.kt
```

Discovery and parse commands emit deterministic machine-readable JSON where applicable, including ontology version, source language, native kind, canonical type, source span, and evidence.

## Phase roadmap

```text
Phase 1   Core ontology primitives                     ✅ hardened
Phase 2   Canonical registry + schema loading          ✅ hardened
Phase 3   Registry-aware typed graph + identity        ✅ hardened
Phase 4   Filesystem / Git discovery                   ✅ hardened
Phase 5   Rust source + AST adapter                    ✅ hardened
Phase 6   Cross-language syntax adapters               ✅ hardened
Phase 7   Semantic projection                          ⛔ blocked by gate until CI green
Phase 8   Runtime observation                          →
Phase 9   Binary / encoding readers                    →
Phase 10  Token / character representation             →
Phase 11  Bit-level bounded reader                     →
Phase 12  Query engine + certification                 →
```

### Pre-Phase-7 gate

The gate must prove all of the following:

```text
canonical 49-level registry + schema consistency
        AND
no synthetic UNIT/MODULE from generic directory grouping
        AND
EXECUTION is standalone + OBSERVED_AT relation
        AND
PROJECTS_TO never mutates structural parent
        AND
AST IDs survive line/whitespace movement
        AND
native syntax kind remains evidence/metadata
        AND
deterministic discovery/traversal
        AND
malformed input remains observable failure/evidence
```

Phase 7 begins only after the latest CI run is green against this gate.

## Safety boundaries

Discovery and parsing are observational and read-only. The engine must not rewrite a repository, infer semantic truth from filenames alone, or turn filesystem layout into the canonical ontology.

Semantic projection is explicit. A syntax observation may `PROJECTS_TO` a canonical semantic node, while structural ownership continues to use `parent`/`CONTAINS`. `KIND` never becomes a hidden 50th+ level.

Binary and bit inspection must be bounded and streaming-capable. Malformed input must remain an explicit observation/error rather than silently changing ontology meaning. External runtimes, encodings, binaries, parsers, and host systems are evidence sources, not ontology owners.

## Normative specification

```text
specifications/universal-ontology-v1.0.json
schemas/universal-ontology.schema.json
specifications/universal-ontology-v1.0.md
standards/universal-ontology-engine.md
docs/architecture.md
```

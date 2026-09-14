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
semantic projection
        ↓
runtime observation
        ↓
representation / encoding
        ↓
bit-level inspection
```

`TYPE` is one canonical level. `KIND` is a specialization owned by exactly one type. The Rust compiled type representation is checked against the registry at load time; the registry remains the source of truth for definitions, ordering, and ontology rules.

`CONTAINS` is structural containment only. `PROJECTS_TO`, `REPRESENTED_AS`, `REFERENCES`, `CAUSES`, and the other relation classes are graph edges and must not be interpreted as filesystem or parent-child containment.

The graph engine uses stable semantic node IDs, deterministic ordered storage/traversal, explicit typed edges, and registry-backed edge validation. Intermediate levels may be unmaterialized; native language structures remain authoritative and are mapped into the universal vocabulary rather than forced into artificial directories.

## Rust workspace

```text
crates/
├── ontology-core         # canonical compiled ontology primitives
├── ontology-graph        # registry-aware typed graph and invariants
├── ontology-registry     # registry loader + canonical validation
├── ontology-discovery    # read-only filesystem/workspace discovery
├── ontology-rust         # syn-based Rust AST semantic adapter
├── ontology-language     # normalized syntax adapters for TS/JS/Python/Go/Java/Kotlin
└── ontology-cli          # ontology-engine command-line interface
```

## Discovery and syntax adapters

The discovery layer is **read-only**. It recognizes `ecosystem-*` workspace boundaries, native repositories/manifests, source roots, source files, and observed execution boundaries. It never creates canonical directories or rewrites the discovered repository shape.

The Rust adapter uses `syn` and emits syntax-derived ontology observations with source spans. Rust declarations are not treated as the ontology itself: the adapter produces evidence that can be projected into canonical semantic levels.

The cross-language layer normalizes syntax evidence from:

```text
TypeScript  JavaScript  Python  Go  Java  Kotlin
     \          |         |      |    |      /
      └──────── ParserAdapter contract ──────┘
```

Each adapter preserves its native construct name/kind and maps it to an existing canonical `TYPE`. No language-specific parser is allowed to introduce a new ontology level. Syntax mappings are evidence, not assertions of semantic truth.

Malformed input is preserved as an explicit observation/error result rather than silently mutating the graph.

### CLI examples

```bash
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

Discovery/parse commands emit deterministic machine-readable JSON where applicable, including ontology version, source language, native kind, canonical type, source span, and evidence.

## Current commands

```bash
cargo run -p ontology-engine -- validate
cargo run -p ontology-engine -- levels
cargo run -p ontology-engine -- inspect 49
cargo run -p ontology-engine -- discover /path/to/workspace
cargo run -p ontology-engine -- discover /path/to/workspace --rust-ast
cargo run -p ontology-engine -- parse <language> <path>
cargo run -p ontology-engine -- --registry path/to/universal-ontology-v1.0.json levels
```

## Engine roadmap

```text
Phase 1   Core ontology primitives                     ✅
Phase 2   Canonical registry + schema loading          ✅
Phase 3   Registry-aware typed graph + identity        ✅
Phase 4   Filesystem / Git discovery                   ✅
Phase 5   Rust source + AST adapter                    ✅
Phase 6   Cross-language parsing adapters              ✅
Phase 7   Semantic projection                          →
Phase 8   Runtime observation                          →
Phase 9   Binary / encoding readers                    →
Phase 10  Token / character representation             →
Phase 11  Bit-level bounded reader                     →
Phase 12  Query engine + certification                 →
```

Phase 6 establishes the language boundary only. The next phase decides how normalized syntax evidence becomes semantic graph projections without conflating parser output with ontology truth.

## Safety boundaries

Discovery and parsing are observational and read-only. The engine must not rewrite a repository, infer semantic truth from filenames alone, or turn filesystem layout into the canonical ontology. AST parsing adds evidence and source spans; it does not redefine the canonical levels.

Semantic projection MUST remain explicit. A syntax observation may `PROJECTS_TO` a canonical semantic node, while structural ownership continues to use `parent`/`CONTAINS`. `KIND` remains a specialization namespace and never becomes a hidden 50th+ level.

Binary and bit inspection must be bounded and streaming-capable. Malformed input must remain an explicit observation/error rather than silently changing ontology meaning. External runtimes, encodings, binaries, parsers, and host systems are evidence sources, not ontology owners.

## Normative specification

The canonical registry is:

```text
specifications/universal-ontology-v1.0.json
```

The machine-validation schema is:

```text
schemas/universal-ontology.schema.json
```

The normative explanation is:

```text
specifications/universal-ontology-v1.0.md
```

The implementation architecture is documented in:

```text
standards/universal-ontology-engine.md
docs/architecture.md
```
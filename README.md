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
04 REALITY        11 REGION         18 PROGRAM        25 MODULE
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
ontology-rust (Rust AST)
        ↓
semantic → runtime → representation → bit
```

`TYPE` is one canonical level. `KIND` is a specialization owned by exactly one type. The Rust compiled type representation is checked against the registry at load time; the registry remains the source of truth for definitions, ordering, and ontology rules.

`CONTAINS` is structural containment only. `PROJECTS_TO`, `REPRESENTED_AS`, `REFERENCES`, `CAUSES`, and the other relation classes are graph edges and must not be interpreted as filesystem or parent-child containment.

The graph engine uses stable semantic node IDs, deterministic ordered storage/traversal, explicit typed edges, and registry-backed edge validation. Intermediate levels may be unmaterialized; native language structures remain authoritative and are mapped into the universal vocabulary rather than forced into artificial directories.

## Rust workspace

```text
crates/
├── ontology-core       # canonical compiled ontology primitives
├── ontology-graph      # registry-aware typed graph and invariants
├── ontology-registry   # registry loader + canonical validation
├── ontology-discovery  # read-only filesystem/workspace discovery
├── ontology-rust       # syn-based Rust AST semantic adapter
└── ontology-cli        # ontology-engine command-line interface
```

## Discovery

Phase 4 adds a **read-only** workspace discovery engine. It recognizes `ecosystem-*` workspace boundaries, native repositories/manifests, source directories, native units, directory groupings, source-file components, and observed execution boundaries. It never creates canonical directories and it leaves unsupported or intermediate ontology levels unmaterialized.

Phase 5 adds a **Rust AST adapter** backed by `syn`. Rust declarations and constructs are emitted as observations at canonical semantic levels, including `ENTITY`, `PROPERTY`, `FUNCTION`, `OPERATION`, `VALUE`, `MODULE`, and `INSTRUCTION`, with source spans. AST observations are connected from the filesystem `ELEMENT` through `PROJECTS_TO`; they are deliberately not encoded as filesystem containment.

Malformed Rust is preserved as an explicit `rust-ast-error` observation rather than mutating or corrupting the graph.

Examples:

```bash
cargo run -p ontology-engine -- discover /path/to/workspace --rust-ast
cargo run -p ontology-engine -- discover /path/to/workspace --rust-ast --include-files --max-depth 3
```

The command emits JSON with ontology version, node/edge counts, per-level counts, read-only status, and discovery observations.

## Current commands

```bash
cargo run -p ontology-engine -- validate
cargo run -p ontology-engine -- levels
cargo run -p ontology-engine -- inspect 49
cargo run -p ontology-engine -- discover /path/to/workspace
cargo run -p ontology-engine -- discover /path/to/workspace --rust-ast
cargo run -p ontology-engine -- --registry path/to/universal-ontology-v1.0.json levels
```

## Engine roadmap

```text
Phase 1   Core ontology primitives                  ✅
Phase 2   Canonical registry + schema loading       ✅
Phase 3   Registry-aware typed graph + identity     ✅
Phase 4   Filesystem / Git discovery                ✅
Phase 5   Rust source + AST adapters                ✅
Phase 6   Cross-language parsing adapters            →
Phase 7   Semantic projection                       →
Phase 8   Runtime observation                       →
Phase 9   Binary / encoding readers                 →
Phase 10  Token / character representation          →
Phase 11  Bit-level bounded reader                  →
Phase 12  Query engine + certification               →
```

## Safety boundaries

Discovery is observational and read-only. It must not rewrite a repository, infer semantic truth from filenames alone, or turn filesystem layout into the canonical ontology. AST parsing adds evidence and source spans; it does not redefine the canonical levels. Binary and bit inspection must be bounded and streaming-capable; malformed input must remain an explicit observation rather than silently changing ontology meaning.

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
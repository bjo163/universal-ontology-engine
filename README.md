# Universal Ontology Engine

> **Quran Inspired ALLAH my Beloved**
>
> Philosophical inspiration only. The technical ontology is not presented as a Qur'anic prescription.

Rust reference implementation of **Universal Ontology v1.0.0**.

## What it is

Universal Ontology Engine provides a canonical vocabulary for resolving software and other resources across **7 zones × 7 levels = 49 canonical levels**.

The 49 levels are a **resolution spine over a graph**. They are not a mandatory 49-level folder tree, AST, compiler IR, or binary layout.

<!-- AUTO:SELF_STATUS:START -->
## Self status

> Auto-generated from `.ontology/self.json`. Do not edit this block manually.

| Signal | Value |
|---|---:|
| Repository | `universal-ontology-engine` |
| Self health | **✅ HEALTHY** |
| Ontology | `1.0.0` |
| Canonical levels | **49** |
| Workspace crates | **7** |
| Rust sources | **8** |
| Read-only inspection | **YES** |

Self inspection command:

```bash
cargo run -p ontology-engine -- self --json
```
<!-- AUTO:SELF_STATUS:END -->

<!-- AUTO:REPO_STATUS:START -->
## Repository automation status

> Auto-generated. Human-authored sections remain outside this block.

| Signal | Value |
|---|---:|
| Health | **HEALTHY** |
| CI | `success` |
| Stable release | `v0.1.3` |
| Dev release | `v0.1.3-dev.9` |
| Main version | `0.1.3` |
| Dev version | `0.1.3` |
| Branch relation | `aligned` |
| Branch model | `feature/* -> dev -> main` |

See [`CHANGELOG.md`](CHANGELOG.md) for generated release history.
<!-- AUTO:REPO_STATUS:END -->

## Canonical levels

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

## Core rules

```text
TYPE  = one canonical ontology level
KIND  = specialization/native evidence owned by a TYPE

parent / CONTAINS     = structural ownership only
PROJECTS_TO           = semantic projection
REPRESENTED_AS        = representation relationship
REFERENCES / CAUSES  = graph relationships
OBSERVED_AT           = observation relationship

SourceSpan            = provenance, not identity
```

Intermediate canonical levels may be omitted when native evidence does not justify them. Native structures remain authoritative; the engine maps evidence into the universal vocabulary instead of forcing artificial folders.

## Architecture

```text
Registry
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

Workspace crates:

```text
crates/
├── ontology-core         # canonical types, IDs, spans, edge kinds
├── ontology-registry     # registry loading and validation
├── ontology-graph        # typed graph and invariants
├── ontology-discovery    # read-only workspace/repository discovery
├── ontology-rust         # Rust AST adapter via syn
├── ontology-language     # TS/JS/Python/Go/Java/Kotlin syntax adapters
└── ontology-cli          # ontology-engine CLI
```

## Phase 1–6 status

| Phase | Scope | Status |
|---|---|---|
| 1 | Core ontology primitives | ✅ Hardened |
| 2 | Canonical registry + schema | ✅ Hardened |
| 3 | Typed graph + identity | ✅ Hardened |
| 4 | Filesystem / Git discovery | ✅ Hardened |
| 5 | Rust syntax + AST adapter | ✅ Hardened |
| 6 | Cross-language syntax adapters | ✅ Hardened |
| 7 | Semantic projection | ⛔ Gate first |
| 8 | Runtime observation | → |
| 9 | Binary / encoding readers | → |
| 10 | Token / character representation | → |
| 11 | Bit-level bounded reader | → |
| 12 | Query + certification | → |

### Phase 1–6 hardening

Phase 4 does not manufacture `UNIT` or `MODULE` from generic directories such as `src/`. Those levels are materialized only when native evidence exists.

Phase 4 runtime observations are standalone `EXECUTION` nodes:

```text
EXECUTION ──OBSERVED_AT──> TARGET
```

They are never structural children of an `ELEMENT`.

Phase 5 AST identity is semantic and parent-scoped. Named observations use canonical type + native construct kind + name, with deterministic duplicate disambiguation. Source line/column is provenance only.

Phase 6 preserves native syntax evidence for TypeScript, JavaScript, Python, Go, Java, and Kotlin. It is a syntax boundary, not compiler-grade semantic analysis.

## Pre-Phase-7 gate

Phase 7 must not start until CI proves:

```text
registry ↔ schema consistency
        AND
no synthetic UNIT/MODULE
        AND
EXECUTION is non-structural + OBSERVED_AT
        AND
projection preserves parent/containment
        AND
AST IDs survive line/whitespace movement
        AND
native syntax kind stays evidence/metadata
        AND
discovery/traversal is deterministic
        AND
malformed input remains observable
```

## CLI

Validate and inspect the canonical registry:

```bash
cargo run -p ontology-engine -- validate
cargo run -p ontology-engine -- levels
cargo run -p ontology-engine -- inspect 49
```

Discover a workspace:

```bash
cargo run -p ontology-engine -- discover /path/to/workspace
cargo run -p ontology-engine -- discover /path/to/workspace --rust-ast
cargo run -p ontology-engine -- discover /path/to/workspace --rust-ast --include-files --max-depth 3
```

Normalize source syntax:

```bash
cargo run -p ontology-engine -- parse rust path/to/file.rs
cargo run -p ontology-engine -- parse typescript path/to/file.ts
cargo run -p ontology-engine -- parse javascript path/to/file.js
cargo run -p ontology-engine -- parse python path/to/file.py
cargo run -p ontology-engine -- parse go path/to/file.go
cargo run -p ontology-engine -- parse java path/to/file.java
cargo run -p ontology-engine -- parse kotlin path/to/file.kt
```

Discovery and parsing are read-only and deterministic for a fixed input revision/environment. Machine-readable output retains ontology version, language, native kind, canonical type, provenance, and evidence where available.

## Safety boundaries

The engine observes; it does not rewrite repositories or treat filenames alone as semantic truth.

Semantic projection is explicit. Runtime observations remain separate from source structure. Representation and binary inspection must remain bounded and streaming-capable. Malformed input is reported as evidence/error rather than silently changing ontology meaning.

## Source hierarchy

```text
Normative ontology
  specifications/universal-ontology-v1.0.json
  schemas/universal-ontology.schema.json
  specifications/universal-ontology-v1.0.md

Engineering rules
  standards/universal-ontology-engine.md

Implementation architecture
  docs/architecture.md

Entry point
  README.md
```

The README is a concise project guide. The normative registry and schema remain authoritative.

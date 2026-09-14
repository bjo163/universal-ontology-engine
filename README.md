# OX-DX

> **Universal Ontology & Experience Engine**
>
> **FROM STRUCTURE TO EXPERIENCE.**

**Universal Ontology Engine** is the Rust implementation inside OX-DX. The GitHub repository remains `universal-ontology-engine`.

```text
OX-DX
└── Universal Ontology & Experience Engine
    └── Universal Ontology Engine
        └── Rust implementation
```

Software leaves traces.

Directories. Manifests. Syntax. Relations. State. Events. Data. Bits.

Different systems describe those traces differently. OX-DX asks a narrower question: **how can heterogeneous evidence be resolved through one common vocabulary without destroying the native structure that produced it?**

The engine observes evidence, preserves provenance, maps what can be justified into a canonical ontology, and connects the result in a typed graph. It does not require reality to pretend it was born as one perfect tree.

**Not replacement. Projection.**  
**Not invention. Observation.**  
**Not one giant hierarchy. A graph.**

**OBSERVE. RESOLVE. UNDERSTAND.**

---

## Quranic Inspiration

> **Quran Inspired ALLAH my Beloved**

OX-DX carries a philosophical inspiration shaped by questions of creation, order, signs, knowledge, measure, relation, observation, manifestation, context, meaning, layers, and trace.

That inspiration gives the project its sense of scale. It does not define the engineering contract.

The ontology is independently specified, versioned, testable, and reviewable. The Qur'an is not presented as a technical specification for the 49 levels, and engineering decisions in this repository are not treated as religious doctrine.

Ancient questions can sharpen modern systems thinking. They do not replace evidence.

---

## Why OX-DX Exists

Software is messy. Reality is messier.

A Rust crate, a TypeScript package, a Python module, a Java class, a process, an event stream, and a binary representation do not share one native shape. Flattening them too early creates a clean diagram and a bad model.

**Bad assumptions create clean lies.**

OX-DX exists to provide a common resolution layer while keeping the evidence that came first. Native systems remain authoritative about their own structure. The engine adds a canonical language for comparison, traversal, projection, and later analysis.

The current implementation focuses on software evidence. The longer architecture moves from structural evidence toward semantics, runtime observation, representation, bounded low-level resolution, query, and certification.

**FROM STRUCTURE TO EXPERIENCE.** is therefore a direction, not a claim that every layer is already implemented.

---

## The Core Idea

```text
NATIVE EVIDENCE
      ↓
DISCOVERY
      ↓
UNIVERSAL ONTOLOGY
      ↓
TYPED GRAPH
      ↓
SEMANTIC PROJECTION
      ↓
RUNTIME OBSERVATION
      ↓
REPRESENTATION
      ↓
BOUNDED RESOLUTION
      ↓
QUERY / CERTIFICATION
```

**Native evidence** is what the system actually exposes: filesystem layout, manifests, source syntax, names, locations, and eventually runtime or representation evidence.

**Discovery** reads that evidence without rewriting the target repository.

**Universal ontology** provides the canonical vocabulary: 7 zones × 7 levels = 49 canonical levels.

**Typed graph** preserves ownership and non-structural relationships without pretending every relation is containment.

**Semantic projection** maps justified native evidence into canonical semantic types. Full Phase 7 remains gated.

**Runtime observation** is where state, events, flows, actions, and executions become dynamic evidence. The relation model exists; the full runtime engine is still planned.

**Representation** separates meaning from encoded form through relationships such as `REPRESENTED_AS`. Representation readers are not yet complete.

**Bounded resolution** is the future path toward token, character, and bit-level inspection without assuming arbitrary inputs can be materialized safely.

**Query / certification** is the later layer for reproducible questions and evidence-backed claims over the graph.

---

## Evidence First

**EVIDENCE FIRST.**

Structure is evidence.

A repository tells us something. Its manifests tell us something. Its filesystem tells us something. Its source syntax tells us something. A runtime event tells us something else.

OX-DX does not manufacture a universal structure when the native evidence does not justify one.

That has concrete consequences:

- a generic `src/` directory does **not** automatically become `UNIT` or `MODULE`;
- filenames alone are not semantic truth;
- native syntax stays native evidence even when it maps to a canonical `TYPE`;
- source location is provenance, not semantic identity;
- runtime `EXECUTION` is an observation, not structural ownership;
- intermediate canonical levels may remain unmaterialized.

Your repository does not need to pretend it was born with 49 folders.

**DON'T FORCE THE SHAPE.**

---

## From Universe to Bit

**FROM UNIVERSE TO BIT.**

The ontology defines a resolution spine from the broadest managed frame toward increasingly fine-grained semantic, dynamic, and representational forms.

The 49 levels are **not 49 boxes**. They are a **resolution spine over a graph**.

They are not a mandatory filesystem hierarchy, not an AST hierarchy, not a compiler IR, not a binary layout, and not a requirement that every resource pass through every level.

When native evidence skips an intermediate level, OX-DX can skip it too.

### 49-Level Resolution Spine

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

The canonical registry groups them into seven zones: existence, context, intent, structure, semantic, dynamic, and representation.

The sequence is stable. Materialization is evidence-driven.

---

## Ontology + Graph

The ontology names. The graph connects.

```text
TYPE
= one canonical ontology level

KIND
= specialization/native evidence owned by a TYPE

parent / CONTAINS
= structural ownership only

PROJECTS_TO
= semantic projection

REPRESENTED_AS
= representation relationship

REFERENCES / CAUSES
= graph relationships

OBSERVED_AT
= observation relationship

SourceSpan
= provenance, not identity
```

Not every relationship means "inside". A node may reference another node without owning it. An execution may be observed at a target without becoming its child. A semantic observation may be projected from an element without replacing the element that produced the evidence.

```text
ELEMENT ──PROJECTS_TO──> semantic observation
NODE ──REFERENCES──────> NODE
NODE ──CAUSES──────────> NODE
NODE ──REPRESENTED_AS──> representation
EXECUTION ──OBSERVED_AT──> TARGET
```

**Not hierarchy. Resolution.**

---

## Provenance Is Not Identity

A location tells us where we saw something. It does not tell us what that thing fundamentally is.

`SourceSpan` records provenance. Source line and column are not allowed to become the sole semantic identity.

For Rust AST observations, named identities are parent-scoped and derived from canonical type, native construct kind, and semantic name, with deterministic duplicate disambiguation. Anonymous items use deterministic ordinals.

Moving a declaration because somebody inserted blank lines should not automatically create a different semantic thing.

**Provenance is not identity.**

---

## Observation Is Not Ownership

Something happening is not the same as something being.

The dynamic model therefore treats runtime evidence differently from structural ownership. `EXECUTION` is an observation node. When it is associated with a target, the relation is explicit:

```text
EXECUTION ──OBSERVED_AT──> TARGET
```

It does not become a structural child merely because the execution happened there.

The current discovery layer preserves this invariant when file-boundary execution observations are requested. Full runtime observation remains a later phase.

**Observation is not ownership.**

---

## Representation Is Not Meaning

Meaning does not disappear when representation changes. But representation is not meaning.

```text
43 INSTRUCTION
44 EXPRESSION
45 VALUE
46 DATA
47 TOKEN
48 CHARACTER
49 BIT
```

`REPRESENTED_AS` keeps representation separate from semantic identity. Character interpretation requires decoding context; bytes do not automatically establish character semantics.

The vocabulary exists today. The deeper representation readers are planned.

---

## Architecture

OX-DX keeps one boundary sacred: **the ontology provides the canonical vocabulary; native systems provide the evidence.**

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

| Stage | Current state | Meaning |
|---|---|---|
| Registry | **HARDENED** | Canonical ontology registry and contract validation |
| Core types + graph invariants | **HARDENED** | Types, IDs, spans, edge classes, containment and relation rules |
| Filesystem / Git discovery | **HARDENED** | Read-only workspace, repository, source, component, and element discovery |
| Rust syntax / AST evidence | **HARDENED** | `syn`-based Rust observations with deterministic identity rules |
| Cross-language syntax evidence | **HARDENED** | Deterministic syntax observations for TS/JS/Python/Go/Java/Kotlin |
| Pre-Phase-7 gate | **GATED** | Proof boundary before full semantic projection advances |
| Semantic projection | **PLANNED / GATED** | Broader projection beyond the current hardened bridge |
| Runtime observation | **PLANNED** | Dynamic state/event/process/flow/action/execution evidence |
| Representation / encoding | **PLANNED** | Explicit mapping into encoded forms |
| Bounded bit inspection | **PLANNED** | Safe low-level readers |
| Query + certification | **PLANNED** | Evidence-backed graph queries and certification |

The presence of an ontology type or edge kind does not mean the full corresponding engine phase is complete.

---

## What Exists Today

The Rust implementation is at **Phase 1–6 hardened**, with **Phase 7 blocked behind an explicit gate**.

| Phase | Scope | Status |
|---|---|---|
| 1 | Core ontology primitives | ✅ **HARDENED** |
| 2 | Canonical registry + schema | ✅ **HARDENED** |
| 3 | Typed graph + identity | ✅ **HARDENED** |
| 4 | Filesystem / Git discovery | ✅ **HARDENED** |
| 5 | Rust syntax + AST adapter | ✅ **HARDENED** |
| 6 | Cross-language syntax adapters | ✅ **HARDENED** |
| 7 | Semantic projection | ⛔ **GATED** |
| 8 | Runtime observation | ◌ **PLANNED** |
| 9 | Binary / encoding readers | ◌ **PLANNED** |
| 10 | Token / character representation | ◌ **PLANNED** |
| 11 | Bit-level bounded reader | ◌ **PLANNED** |
| 12 | Query + certification | ◌ **PLANNED** |

Phase 4 intentionally does not manufacture `UNIT` or `MODULE` from generic source directories. Phase 5 uses Rust AST evidence through `syn`. Phase 6 provides normalized syntax observations for six additional languages, but those adapters are **not compiler-grade frontends**.

The ontology contract version and the Rust package version are separate concerns: **Universal Ontology is v1.0.0**, while repository implementation releases follow their own `0.x` release train.

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
| Rust sources | **12** |
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
| Stable release | `v0.1.7` |
| Dev release | `v0.1.7-dev.21` |
| Main version | `0.1.7` |
| Dev version | `0.1.7` |
| Branch relation | `aligned` |
| Branch model | `feature/* -> dev -> main` |

See [`CHANGELOG.md`](CHANGELOG.md) for generated release history.
<!-- AUTO:REPO_STATUS:END -->

---

## Workspace

The workspace currently contains seven Rust crates:

```text
crates/
├── ontology-core
├── ontology-registry
├── ontology-graph
├── ontology-discovery
├── ontology-rust
├── ontology-language
└── ontology-cli
```

| Crate | Responsibility |
|---|---|
| `ontology-core` | Canonical types, IDs, source spans, edge kinds, and shared errors |
| `ontology-registry` | Registry loading, validation, canonical definitions, and kind ownership |
| `ontology-graph` | Typed graph storage, structural invariants, relation invariants, deterministic traversal |
| `ontology-discovery` | Read-only workspace/repository/source discovery and evidence collection |
| `ontology-rust` | Native Rust AST observation using `syn` |
| `ontology-language` | Deterministic syntax observation for TypeScript, JavaScript, Python, Go, Java, and Kotlin |
| `ontology-cli` | CLI access to validation, inspection, discovery, syntax parsing, and self-inspection |

No language adapter owns the ontology. Adapters produce evidence. The registry remains canonical.

---

## Pre-Phase-7 Gate

**Before meaning comes the boundary. Before projection comes proof.**

Phase 7 does not advance until CI continues to prove:

```text
registry ↔ schema consistency
        AND
no synthetic UNIT/MODULE
        AND
EXECUTION is non-structural
        AND
OBSERVED_AT is used for runtime observation
        AND
projection preserves parent/containment
        AND
AST identities survive line/whitespace movement
        AND
native syntax kind remains evidence/metadata
        AND
discovery/traversal is deterministic
        AND
malformed input remains observable
```

These constraints prevent later semantic layers from being built on unstable identity, invented hierarchy, or mixed structural/runtime meaning.

**EVIDENCE FIRST.**

---

## CLI

The binary name is `ontology-engine`.

Validate and inspect the canonical registry:

```bash
cargo run -p ontology-engine -- validate
cargo run -p ontology-engine -- levels
cargo run -p ontology-engine -- inspect 49
```

Inspect this repository first:

```bash
cargo run -p ontology-engine -- self
cargo run -p ontology-engine -- self --json
```

Discover a workspace:

```bash
cargo run -p ontology-engine -- discover /path/to/workspace
cargo run -p ontology-engine -- discover /path/to/workspace --rust-ast
cargo run -p ontology-engine -- discover /path/to/workspace --rust-ast --include-files --max-depth 3
```

`--rust-ast` asks discovery to parse Rust repositories with the native `ontology-rust` adapter. `--include-files` adds explicit file-boundary execution observations through `OBSERVED_AT`; it does not turn execution into structural containment.

Parse cross-language syntax evidence:

```bash
cargo run -p ontology-engine -- parse typescript path/to/file.ts
cargo run -p ontology-engine -- parse javascript path/to/file.js
cargo run -p ontology-engine -- parse python path/to/file.py
cargo run -p ontology-engine -- parse go path/to/file.go
cargo run -p ontology-engine -- parse java path/to/file.java
cargo run -p ontology-engine -- parse kotlin path/to/file.kt
```

The `parse` command currently supports **TypeScript, JavaScript, Python, Go, Java, and Kotlin**. Rust syntax uses the separate AST path through `discover --rust-ast`.

Cross-language parsing is deterministic syntax observation, not complete compiler semantic analysis.

---

## Safety Boundaries

Read the evidence. Don't rewrite the world.

Discovery is read-only. Native structures remain authoritative. Generic directory names are not enough to establish universal semantic levels. Malformed inputs should remain observable rather than silently changing ontology meaning.

Structural ownership, semantic projection, runtime observation, and physical representation remain distinct relation classes.

Future low-level readers must be bounded. Representation must not be mistaken for meaning. Provenance must not be mistaken for identity.

---

## Non-Goals

OX-DX is not currently:

- a mandatory 49-folder hierarchy;
- a compiler;
- a replacement for native language semantics;
- a source-rewriting engine;
- a filename-based semantic oracle;
- an absolute truth engine;
- a system that forces every resource through every ontology level;
- a claim of complete semantic or runtime understanding.

Ambition is allowed. False certainty is not.

---

## Source of Truth

The README is the human entry point. It is not the normative ontology contract.

### Normative ontology

- [`specifications/universal-ontology-v1.0.json`](specifications/universal-ontology-v1.0.json) — machine-readable canonical registry
- [`schemas/universal-ontology.schema.json`](schemas/universal-ontology.schema.json) — validation schema
- [`specifications/universal-ontology-v1.0.md`](specifications/universal-ontology-v1.0.md) — normative human-readable ontology specification

### Engineering rules

- [`standards/universal-ontology-engine.md`](standards/universal-ontology-engine.md) — engineering invariants and implementation rules

### Implementation architecture

- [`docs/architecture.md`](docs/architecture.md) — layer boundaries, graph model, discovery, identity, runtime, and representation architecture

### Human entry point

- [`README.md`](README.md) — public identity, current status, and practical usage

If these layers disagree, the normative and engineering contracts win over README prose.

---

## Roadmap

The next architectural boundary is not "add more words." It is **prove the gate, then project meaning without breaking evidence**.

```text
PHASE 1–6   HARDENED
     ↓
PRE-PHASE-7 GATE
     ↓
PHASE 7     SEMANTIC PROJECTION
     ↓
PHASE 8     RUNTIME OBSERVATION
     ↓
PHASE 9     BINARY / ENCODING
     ↓
PHASE 10    TOKEN / CHARACTER
     ↓
PHASE 11    BOUNDED BIT READER
     ↓
PHASE 12    QUERY / CERTIFICATION
```

The graph grows only when the evidence and invariants are strong enough to justify the next layer.

**DON'T FORCE THE SHAPE.**

---

## Development

Repository work follows:

```text
feature/*
    ↓
dev
    ↓
main
```

The repository already maintains self-inspection, CI validation, generated repository status, generated changelog data, release channels, branch alignment, and a read-only self-steward.

The engine is expected to understand its own repository baseline before being trusted with external repositories.

```bash
git clone https://github.com/bjo163/universal-ontology-engine.git
cd universal-ontology-engine
cargo test --workspace --all-targets
cargo run -p ontology-engine -- self --json
```

---

## Project Information

- **Public identity:** OX-DX
- **Descriptor:** Universal Ontology & Experience Engine
- **Technical implementation:** Universal Ontology Engine
- **Repository:** `bjo163/universal-ontology-engine`
- **Ontology contract:** Universal Ontology v1.0.0
- **Implementation language:** Rust 2021
- **License:** [MIT](LICENSE)

OX-DX is intentionally unfinished.

Not because the boundary is vague, but because the boundary is large.

The work begins by refusing to confuse observation with ownership, provenance with identity, representation with meaning, or a clean diagram with reality.

**Reality leaves traces.**  
**The graph connects. The ontology names.**  
**FROM UNIVERSE TO BIT.**

**OBSERVE. RESOLVE. UNDERSTAND.**

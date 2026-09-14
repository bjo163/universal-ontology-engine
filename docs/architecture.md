# Engine Architecture

Universal Ontology Engine is the executable reference implementation for Universal Ontology v1.0.

## Invariants

1. `type` is exactly one of the 49 canonical ontology levels.
2. `kind` is a specialization owned by exactly one `type`.
3. Node identity is stable and independent from filesystem paths.
4. `contains` is distinct from `projects_to` and `represented_as`.
5. Missing physical layers are reported as unmaterialized rather than treated as invalid ontology.
6. Representation readers are bounded and streaming-capable.
7. Native language syntax remains authoritative for syntax; ontology mappings are evidence/projections.

## Resolution model

```text
STRUCTURAL CONTAINMENT
Universe → ... → Repository → Source → Unit → Module → Component → Element

SEMANTIC / DYNAMIC PROJECTION
Element → Symbol / Entity / Property / Relation / Operation / Function / Behavior

RUNTIME OBSERVATION
Program/System → Execution → State / Event / Process / Flow / Transition / Action

REPRESENTATION
Instruction → Expression → Value → Data → Token → Character → Bit
```

The 49 levels form a canonical resolution spine over a graph. They do not require every resource to materialize every level, and they do not define a universal filesystem tree.

## Discovery boundary

`ontology-discovery` is observational and read-only. It identifies native workspace, repository, source, unit, module, component, and element boundaries using repository/manifest/source evidence.

Filesystem layout is never promoted into semantic truth merely because a directory or filename has a familiar name. Intermediate levels may be absent. Stable IDs are semantic and parent-scoped; paths and offsets are provenance.

## Language adapter boundary

Language adapters convert native syntax into normalized evidence. The current Phase 6 adapters cover:

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

- preserve the native language/construct kind;
- map only to existing canonical `TYPE`s;
- retain source spans where available;
- be deterministic for the same input;
- treat malformed input as observable failure/evidence;
- avoid inventing ontology levels or artificial folders.

For example, a Java `class` and a Rust `struct` may both map to `ENTITY` at the canonical layer while retaining different native kinds (`class`, `struct`). That is interoperability vocabulary, not a claim that the two constructs are identical.

## Semantic projection boundary

Syntax and semantics are separate graph concerns.

```text
ELEMENT
   │
   ├── PROJECTS_TO → SYMBOL / ENTITY / PROPERTY / FUNCTION / ...
   └── REPRESENTED_AS → source/encoding/lexical representation
```

A parser observation does not become a semantic assertion automatically. Semantic projection is the next dedicated layer and may combine syntax evidence, declarations, symbol resolution, references, and other evidence sources.

`parent` and `CONTAINS` remain reserved for structural ownership. `PROJECTS_TO`, `REFERENCES`, `SPECIALIZES`, `DEPENDS_ON`, `INVOKES`, `CAUSES`, and related edge classes express relations and must not be interpreted as literal containment.

## Bit boundary

`BIT` is a representation endpoint. The engine must not imply that a bit is a semantic child of a source-level construct. A projection/representation edge carries that relationship.

Future binary readers must support bounded ranges, malformed-input detection, encoding/endianness metadata, and cancellation without requiring whole-file materialization.

## Current pipeline

```text
Registry
   ↓
Core Types + Graph Invariants
   ↓
Filesystem / Git Discovery
   ↓
Language Syntax Adapters
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

Each layer consumes explicit evidence from the previous layer and must not silently rewrite or reinterpret another layer's ownership.

# Universal Ontology Engine Architecture

## Purpose

The reference engine is a Rust implementation of Universal Ontology v1.0. It discovers, normalizes, relates, validates, queries, and renders resources across the 49-level canonical resolution spine.

The engine is a separate implementation concern from the foundation contract. The foundation defines vocabulary and invariants; the engine defines how those invariants are observed from real systems.

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
        ownership hierarchy       references / projection / events
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

## Non-negotiable separation

The engine MUST NOT pretend that an AST is the ontology itself. A parser yields evidence from which ontology nodes and relations can be discovered.

Likewise, bytes are evidence of representation; they do not directly determine semantic meaning without the required decoding context.

## Core crates

```text
ontology-core          canonical IDs, types, kinds, spans, provenance
ontology-registry      49-level registry and versioned vocabulary
ontology-graph         nodes, edges, indexes, graph traversal
ontology-discovery     filesystem, git, manifests, workspace discovery
ontology-parser        parser abstraction and normalized syntax evidence
ontology-semantic      symbols, entities, properties, relations, semantic projections
ontology-runtime       runtime observations, states, events, executions
ontology-representation bytes, encodings, lexical and binary views
ontology-bit           byte/bit traversal and bounded binary inspection
ontology-query         deterministic path/type/kind/relation queries
ontology-validate      conformance and invariant checking
ontology-cli            command-line interface
```

## Canonical node

A node should carry both identity and provenance:

```rust
pub struct OntologyNode {
    pub id: NodeId,
    pub ontology_type: OntologyType,
    pub kind: Option<OntologyKind>,
    pub name: Option<String>,
    pub parent: Option<NodeId>,
    pub provenance: Vec<Provenance>,
    pub span: Option<SourceSpan>,
}
```

`NodeId` must be stable for the same logical resource within the declared identity scope. File offsets and paths may participate in provenance but must not become the only identity mechanism.

## Evidence and provenance

Every discovered lower-level node should be traceable to evidence:

```text
Node
 └── Provenance
      ├── source URI/path
      ├── repository revision
      ├── parser/adapter
      ├── byte or character span
      ├── discovery timestamp
      └── confidence/observation metadata
```

The engine reports what it observed and how it derived the node. It must not silently promote an observation into an asserted semantic truth.

## Adapter architecture

Adapters convert native structures into ontology evidence.

```text
Rust       ─┐
TypeScript ─┤
JavaScript ├─> ParserAdapter ─> NormalizedSyntax ─> OntologyProjection
Python     ─┤
Go         ─┤
Java       ─┘

ELF / PE / Mach-O / raw binary ─> BinaryAdapter ─> RepresentationProjection
UTF-8 / UTF-16 / bytes          ─> EncodingAdapter ─> Character/BIT projection
```

A parser adapter may use Tree-sitter or a language-native parser. Tree-sitter is suitable for a broad incremental parsing layer because it produces concrete syntax trees, is designed to tolerate syntax errors, and has broad language grammar support. citeturn521606search0turn521606search2

For Rust specifically, the adapter should preserve the distinction between lexical tokens, syntactic productions, expressions, and values rather than flattening everything into one node type. The Rust Reference explicitly separates lexer-produced tokens from syntax productions and defines literal expressions as expressions consisting of a single token that denotes a value. citeturn521606search4turn521606search7

## Semantic graph

Semantic connections should use explicit edges rather than overloading the parent field.

```text
parent            = structural owner
contains          = containment edge
references        = reference edge
specializes       = specialization edge
projects_to       = ontology projection
represented_as    = encoding relation
causes            = dynamic relation
observed_at       = evidence relation
```

This is compatible with graph-based knowledge representations in which explicit statements connect resources through typed predicates. RDF 1.2, for example, models statements as subject-predicate-object triples and distinguishes the graph semantics from the source representation. Universal Ontology is not an RDF implementation, but the same separation supports clean relation semantics. citeturn521606search6turn521606search1

## Runtime layer

Runtime inspection must be treated as observation, not compile-time structure.

```text
PROGRAM
  ↓
EXECUTION
  ├── STATE snapshots
  ├── EVENT occurrences
  ├── PROCESS activity
  ├── FLOW movement
  └── TRANSITION changes
```

Runtime data should include timestamps, process/thread context where available, and provenance. An observed state/event MUST NOT overwrite source-level structural identity.

## Binary and bit layer

The final seven levels are representation-aware. They are not a universal claim about how every language/compiler internally works.

```text
INSTRUCTION
   ↓
EXPRESSION
   ↓
VALUE
   ↓
DATA
   ↓
TOKEN
   ↓
CHARACTER
   ↓
BIT
```

Different representations can branch. For example, a UTF-8 text buffer and a machine executable reach `BIT` through different decoding/projection paths. The engine should therefore represent these as graph paths, not force a single physical chain.

Binary debugging metadata may be used when available. DWARF provides standardized source-level debugging information and is architecture-independent, making it useful as an evidence source for source-to-machine mappings; DWARF 6 remains under development, so the engine must pin the supported format version explicitly. citeturn521606search3

## Encoding rule

`CHARACTER` is context-sensitive. The engine MUST record the encoding used to derive it. UTF-8, for example, maps Unicode scalar values/code points to encoded byte sequences; a byte sequence alone is not enough to assert a character without decoding context.

## Bit inspection safety

The bit reader MUST support bounded inspection:

- maximum byte/bit range;
- streaming rather than unbounded loading;
- cancellation for large binaries;
- malformed-input detection;
- explicit endianness/encoding metadata;
- no executable interpretation during passive inspection.

## Query model

Queries operate over type, kind, identity, edges, and provenance.

```text
ontology query --type FUNCTION
ontology query --kind rust-crate
ontology trace --from PROJECT --to BIT
ontology path <node-id>
ontology refs <node-id>
ontology explain <node-id>
ontology bytes <path> --start 0 --length 128
ontology bits <path> --start 0 --length 256
```

`trace` must report projections and omitted/unmaterialized levels instead of fabricating nodes.

## Determinism

Discovery should be deterministic for a fixed repository revision and environment. Ordering MUST use stable keys, never filesystem enumeration order.

A scan result should include:

```text
ontology_version
engine_version
input_revision
adapter_versions
node_count
edge_count
warnings
errors
```

## Certification target

A repository is compliant when the engine can:

1. discover its topological boundary;
2. map native structures into canonical types without forced folders;
3. retain stable identity and provenance;
4. represent relations independently from containment;
5. traverse semantic and runtime projections;
6. inspect representation down to bounded bits;
7. validate the result against the v1.0 registry;
8. produce machine-readable JSON output.

## External standards are inputs, not ownership

Tree-sitter, DWARF, ELF/PE/Mach-O readers, language parsers, Unicode decoding, Git, and runtime APIs are adapters/evidence sources. None becomes part of the canonical ontology vocabulary merely because the engine can consume it.

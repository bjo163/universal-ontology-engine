# OX-DX Canonical Terminology

This is the canonical communication glossary. When this file conflicts with the normative ontology specification or engineering standard, the normative technical source wins.

| Term | Canonical definition | Short form | Use when | Do not use as |
|---|---|---|---|---|
| **OX-DX** | Public brand and system identity around the Universal Ontology & Experience Engine. | Brand/system identity. | Naming the overall project and public experience. | A crate name, ontology type, or claim that every future capability exists today. |
| **Universal Ontology & Experience Engine** | Conceptual product descriptor for OX-DX, spanning evidence, ontology, graph, resolution, and the longer experience horizon. | Product descriptor. | Explaining what OX-DX aims to unify conceptually. | The name of a specific Rust crate or proof that “experience” is fully implemented. |
| **Universal Ontology** | Normative 49-type vocabulary, version 1.0.0, arranged as 7 zones × 7 levels and defined as a resolution spine over a graph. | Canonical vocabulary. | Referring to the ontology contract. | A mandatory filesystem tree or universal programming language. |
| **Universal Ontology Engine** | Rust implementation in this repository that validates and works with the Universal Ontology and software evidence. | Rust engine. | Referring to the implementation. | The entire OX-DX brand identity. |
| **Ontology** | A stable vocabulary and set of distinctions for describing modeled resources and their canonical types. | Vocabulary of types. | Discussing classification and canonical meaning. | A claim that physical resources must mirror ontology order. |
| **Resolution** | Movement across increasingly specific conceptual or representational scales while preserving relevant distinctions and provenance. | Change of descriptive scale. | Explaining “from universe to bit” or finer inspection. | Deletion, simplification by loss, or forced ownership. |
| **Resolution Spine** | Stable ordered sequence of the 49 canonical types used as a vocabulary for scale; resources may skip unmaterialized levels. | Ordered type spine. | Explaining canonical order. | A mandatory parent/child chain. |
| **Evidence** | Observable information that can support a claim, classification, relation, or projection. | Support for a claim. | Describing what a source exposes. | Absolute truth or authority beyond what was observed. |
| **Native Evidence** | Evidence emitted by the source system in its own structure, syntax, manifests, names, locations, or runtime/representation forms. | Source-native observations. | Distinguishing source facts from canonical projection. | A replacement ontology. |
| **Observation** | A recorded finding about a resource or event based on available evidence. | Recorded evidence. | Describing discovered syntax, files, spans, or runtime evidence. | Structural ownership. |
| **Runtime Observation** | Dynamic evidence about state, events, processes, flows, actions, or executions. Full runtime observation is planned beyond the current hardened phases. | Dynamic evidence. | Discussing future/current dynamic evidence boundaries accurately. | A currently complete runtime subsystem. |
| **Provenance** | Information about where, when, or from what source an observation was obtained; `SourceSpan` is a concrete source-provenance structure. | Where evidence came from. | Explaining source location and traceability. | Semantic identity. |
| **Identity** | Stable semantic identifier for a resource, not solely derived from paths, filenames, offsets, or source line numbers when a stable identifier can be established. | What the resource is addressed as. | Discussing stable nodes and deduplication. | A source span or path alone. |
| **Graph** | Relationship model connecting nodes through typed edges. | Connected model. | Discussing non-tree relations. | Synonym for hierarchy. |
| **Typed Graph** | Graph whose nodes have canonical ontology types and whose edges use defined relation kinds with structural invariants. | Graph with explicit types/edges. | Explaining OX-DX relation modeling. | An untyped network diagram. |
| **Projection** | Explicit description of a resource at another ontology level without replacing the originating resource. | Another justified view. | Mapping evidence into semantic/dynamic/representational views. | Rewriting native structure. |
| **Semantic Projection** | Projection from observed/native evidence into canonical semantic types when justified. Broader Phase 7 projection remains gated. | Evidence-backed semantic view. | Discussing Phase 7 and current bridge behavior. | A complete semantic-understanding engine. |
| **Representation** | How a resource or value is encoded or realized in a more concrete form, connected explicitly rather than treated as identity. | Encoded form. | Discussing DATA/TOKEN/CHARACTER/BIT or `REPRESENTED_AS`. | Meaning itself. |
| **Encoding** | Rules/context used to interpret a concrete representation, especially bytes/characters. | Interpretation context. | Discussing character/data boundaries. | Proof of semantic identity. |
| **Structure** | Observable or modeled organization and ownership of resources. | Organization. | Discussing repository/source/unit/module/component/element boundaries. | Every relation in the graph. |
| **Experience** | Brand-level horizon for how structured evidence, behavior, observation, and representation become interpretable to people or systems. | Interpretability horizon. | Brand storytelling and future product framing. | A current canonical ontology level or fully implemented subsystem. |
| **TYPE** | One of the 49 canonical ontology levels. | Canonical level. | Naming ontology classification. | Native specialization. |
| **KIND** | Specialization within a TYPE; each registered kind has one owning canonical type. | Native/specific specialization. | Preserving native language/framework distinctions. | A new ontology level. |
| **CONTAINS** | Structural ownership edge. | Parent/child ownership. | Only when one node structurally owns/contains another. | Reference, projection, observation, or representation. |
| **PROJECTS_TO** | Explicit edge from a resource to another justified ontology-level view. | Projection edge. | Semantic/dynamic projection. | Mutation or replacement of the source node. |
| **REPRESENTED_AS** | Edge connecting a resource/value to a concrete representation. | Representation edge. | Separating meaning from encoded form. | Structural containment. |
| **OBSERVED_AT** | Edge linking an observation/execution to the target/location where it was observed. | Observation linkage. | Dynamic or file-boundary observation relationships. | Ownership or containment. |

## Edge vocabulary

The implementation currently defines 11 edge kinds:

`CONTAINS`, `REFERENCES`, `SPECIALIZES`, `DEPENDS_ON`, `INVOKES`, `PRODUCES`, `CONSUMES`, `CAUSES`, `PROJECTS_TO`, `REPRESENTED_AS`, `OBSERVED_AT`.

Use these names when the technical relation matters. Do not invent decorative edge names in engineering prose.

## Version language

Keep these distinct:

- **Ontology contract:** Universal Ontology **v1.0.0**.
- **Implementation release train:** repository `0.x` releases.
- **Current stable release in the dated audit snapshot (2026-09-14 06:41 UTC):** **v0.1.4**.
- **Latest prerelease in the dated audit snapshot (2026-09-14 06:41 UTC):** **v0.1.4-dev.15**. After stable promotion, `dev/Cargo.toml` reports `0.1.4`.

For volatile release status, prefer generated repository/release metadata over hardcoded prose.

## Normative sources

- `../../../specifications/universal-ontology-v1.0.md`
- `../../../specifications/universal-ontology-v1.0.json`
- `../../../schemas/universal-ontology.schema.json`
- `../../../standards/universal-ontology-engine.md`

# OX-DX Canonical Responsibility Matrix

One capability should have one clear owner.

| Layer | Name | Owns | Does Not Own | Examples |
|---|---|---|---|---|
| Brand | **OX-DX** | Public identity, brand promise, visual identity, voice, approved taglines | Ontology semantics, schema truth, engine behavior | OX-DX name, logo, **EVIDENCE FIRST.** |
| Product / System | **Universal Ontology & Experience Engine** | Conceptual user-facing system boundary and product narrative | Low-level crate APIs, normative ontology definitions | Structure-to-experience product framing |
| Specification | **OX-DX Specification / Universal Ontology** | Canonical ontology vocabulary, normative definitions, compatibility contract | Runtime implementation, marketing claims | `universal-ontology-v1.0.json`, schema |
| Engine | **Universal Ontology Engine** | Computation implementing technical contracts | Brand identity, public website ownership, ontology redefinition | registry, graph, discovery, adapters |
| Tool | **OX-DX Tools** / current `ontology-engine` CLI | User/operator actions over engine capabilities | Normative semantics, product strategy | validate, inspect, discover, parse |
| Interface | **OX-DX Interfaces** | Presentation and interaction with capabilities | Redefining ontology, schema, or engine invariants | future Web, Explorer, Console |
| Documentation | **OX-DX Documentation / OX-DX Docs** | Explanation, guides, references, navigation to sources of truth | Changing schema/engine semantics by prose | in-repo docs; future public docs |
| Community | **OX-DX Community** | Contributor interaction, discussion, proposal culture | Unilateral technical authority | Discussions, future forum/chat |
| Assets | **OX-DX Brand Assets** | Logo, visual system, graphics, visual governance | Technical semantics or implementation behavior | `assets/brand/`, `assets/vector/` |

## Ownership rules

### Specification owns ontology definition

The website, CLI, README, docs, and marketing may explain the ontology.

They may not silently redefine it.

### Engine owns computation

The engine implements the technical contract.

A tool or interface may invoke it but should not duplicate its core semantic authority.

### Tools own interaction

CLI and utilities expose operations.

A command name must not become a new ontology concept merely because it is convenient.

### Interfaces own presentation

Web/UI/API/visualization surfaces present experience.

They cannot introduce competing definitions of TYPE, KIND, edge semantics, or version truth.

### Documentation owns explanation

Documentation can simplify progressively.

It cannot make normative changes by wording alone.

### Brand owns recognition

Brand assets and language make the system recognizable.

They do not prove technical correctness.

## Conflict rule

When two layers appear to own the same capability:

1. identify the normative/technical owner;
2. make the other layer a consumer, adapter, presentation, or documentation surface;
3. remove duplicate authority;
4. document the boundary.

**No duplicate ownership.**

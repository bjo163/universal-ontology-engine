# OX-DX Naming Conventions

This communication document is the concise naming checklist.

The canonical brand/naming governance source is:

**`../architecture/brand-architecture.md`**

Specialized policy lives in `../architecture/`.

## Canonical hierarchy

```text
OX-DX
  ↓
Universal Ontology & Experience Engine
  ↓
Universal Ontology Engine
  ↓
technical repositories / crates / tools
```

- **OX-DX** = umbrella public brand/system identity.
- **Universal Ontology & Experience Engine** = conceptual product/system descriptor.
- **Universal Ontology Engine** = current Rust technical engine.
- **`universal-ontology-engine`** = current repository; do not rename merely for brand consistency.
- Existing crates remain implementation-oriented names such as `ontology-core` and `ontology-graph`.
- The current CLI package/binary remains `ontology-engine`.

## Communication naming rule

Use the name for the layer you actually mean.

Do not use repository, crate, CLI, product, and brand names as arbitrary synonyms.

## Future public extensions

Default pattern when a real new public responsibility exists:

**OX-DX + clear noun**

Examples as naming direction only:
- OX-DX Web
- OX-DX Docs
- OX-DX CLI
- OX-DX SDK

Potential future repository namespace:

`ox-dx-<responsibility>`

Do not create repositories/products for symmetry.

## Technical names

The more foundational the component, the more precise the name should be.

```text
Brand:   OX-DX
System:  Universal Ontology & Experience Engine
Engine:  Universal Ontology Engine
Crate:   ontology-graph
File:    universal-ontology-v1.0.json
```

Do not force `ox-dx-` onto internal crates.

## Reserved technical vocabulary

Do not casually reuse ontology TYPE names, registered KIND names, graph edge names, schema identifiers, or normative specification terms as unrelated product names.

## Boundaries

- Quranic inspiration is philosophical, not a source of sacred technical product names.
- Gen-Z/grunge affects voice and presentation, not foundational naming clarity.
- Version numbers for brand, engine, ontology, and schema must not be conflated.
- Existing names are preserved until an explicit migration is approved.

## Before approving a new name

1. What responsibility does it own?
2. Does another component already own it?
3. Which layer is this: brand, product, engine, tool, interface, repository, package, or documentation?
4. Does it collide with existing ontology/engine vocabulary?
5. Does it imply unimplemented capability?
6. Does it require a new repository at all?
7. Is migration worth the compatibility cost?

For the full policy, use `../architecture/brand-architecture.md`.

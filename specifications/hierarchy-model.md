# Universal Hierarchy Model v0.3

This document defines how the single foundation represents entities from Universe to Implementation without imposing a physical repository layout.

## Canonical parent chain

```text
UNIVERSE
  ↓
ECOSYSTEM
  ↓
ORGANIZATION? 
  ↓
DOMAIN?
  ↓
PROJECT
  ↓
REPOSITORY
  ↓
SOURCE
  ↓
UNIT
  ↓
MODULE
  ↓
COMPONENT
  ↓
ELEMENT
  ↓
IMPLEMENTATION
```

## Node model

Every node has:

- `id`: stable semantic identifier within its parent scope.
- `type`: one canonical hierarchy level.
- `parent`: stable parent identifier, except the Universe root where it is `null`.
- `name`: human-readable label.
- `path`: optional operational location/reference.
- `native_type`: optional language/framework-specific mapping.
- `children`: optional references to direct descendants.
- `relationships`: optional non-ownership references.

The node schema is `schemas/foundation-node.schema.json`.

## Rules

A child MUST belong to exactly one canonical parent in the ownership tree.

`ORGANIZATION` and `DOMAIN` MAY be skipped. A project can therefore belong directly to an ecosystem, or to the deepest applicable optional layer.

`SOURCE` through `IMPLEMENTATION` are semantic levels. Native code layouts decide how they are physically represented.

Supporting concerns such as tests, documentation, tools, assets, configuration, infrastructure, and deployment are not hierarchy nodes unless they independently represent a canonical entity.

External systems may appear in `relationships` but are never implicitly owned children.

## Native mapping examples

```text
Rust
REPOSITORY → Cargo workspace/repository
SOURCE     → src/
UNIT       → crate
MODULE     → mod / module
COMPONENT  → cohesive subsystem/type grouping
ELEMENT    → function / method / type / trait
IMPLEMENTATION → concrete impl/body/logic

Node/TypeScript
REPOSITORY → package/workspace repository
SOURCE     → src/ / apps/ / packages/
UNIT       → package / application / library
MODULE     → module
COMPONENT  → component / service / class / feature
ELEMENT    → function / method / type
IMPLEMENTATION → concrete runtime/build logic
```

These are mappings only. The foundation MUST NOT require a repository to rename valid native structures.

## Discovery path

The expected discovery sequence is:

```text
universe.json
    → ecosystem node
    → project node(s)
    → repository node(s)
    → source node(s)
    → unit node(s)
    → module node(s)
    → component node(s)
    → element node(s)
    → implementation node(s)
```

A discovery tool may stop at any level and continue later. Partial discovery is valid, but discovered identities and parent references MUST remain stable.

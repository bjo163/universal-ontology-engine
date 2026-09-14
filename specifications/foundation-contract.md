# Complete Foundation Contract

Universe Foundation is the single normative foundation for every canonical layer:

```text
UNIVERSE → ECOSYSTEM → ORGANIZATION? → DOMAIN? → PROJECT → REPOSITORY → SOURCE → UNIT → MODULE → COMPONENT → ELEMENT → IMPLEMENTATION
```

`?` means optional. The chain is semantic; it does not require matching physical directories.

## Rules

1. One foundation contract governs the complete chain.
2. No `ecosystem-foundation` or lower-level foundation repository is required.
3. Every entity should have a stable identifier in its parent scope.
4. Paths and directory names are operational metadata unless explicitly declared as identity.
5. Native language/framework concepts are mapped to the universal vocabulary rather than replaced.
6. Supporting concerns such as tests, docs, config, assets, deployment, and infrastructure are horizontal concerns, not extra hierarchy levels.
7. External systems are relationships, not children of the ownership tree.

## Native implementation mapping

```text
UNIT
├── Rust      → crate
├── Node      → package / application / library
├── Go        → package / command / service
├── Python    → package / module
└── Java      → module / package / application
```

The lower concepts remain semantic:

```text
SOURCE → native implementation area
UNIT → native implementation container
MODULE → logical implementation grouping
COMPONENT → cohesive responsibility
ELEMENT → meaningful implementation construct
IMPLEMENTATION → concrete behavior/logic
```

A repository must not introduce `unit/`, `module/`, `component/`, `element/`, or `implementation/` directories solely for conformance.

## Conformance surface

The foundation contract is represented by:

- `governance/hierarchy.md`
- `specifications/universe-contract.md`
- `specifications/universe-contract.json`
- `specifications/universe-contract.instance.json`
- `specifications/foundation-contract.json`
- `schemas/universe.schema.json`
- `universe.json`

The same foundation governs registry, discovery, hierarchy semantics, native mapping, validation, and orchestration boundaries.

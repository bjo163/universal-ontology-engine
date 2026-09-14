# OX-DX Repository Map

This map separates **verified current repository state** from **future naming possibilities**.

## Verified current state

```text
OX-DX
│
└── universal-ontology-engine              CURRENT
    └── Universal Ontology Engine / Rust implementation
        ├── ontology-core
        ├── ontology-registry
        ├── ontology-graph
        ├── ontology-discovery
        ├── ontology-rust
        ├── ontology-language
        └── ontology-engine CLI package/binary
```

The existing repository is not renamed by this architecture.

## Future ecosystem map

Every item below is **PLANNED / CONDITIONAL**. The names are reserved directions, not claims that the repositories currently exist.

```text
OX-DX
│
├── universal-ontology-engine              CURRENT
│   └── Rust engine + current technical docs/spec references
│
├── ox-dx-web                              PLANNED / CONDITIONAL
│   └── public website implementation
│
├── ox-dx-docs                             PLANNED / CONDITIONAL
│   └── dedicated public documentation, only if separation is justified
│
├── ox-dx-sdk                              PLANNED / CONDITIONAL
│   └── supported developer SDK, only if a real SDK contract exists
│
└── ox-dx-cli                              PLANNED / CONDITIONAL
    └── future public CLI wrapper/migration, only if separate ownership is justified
```

## Default rule

**Do not create a repository because a box exists in this diagram.**

Create one only when:
- responsibility is distinct;
- ownership is clear;
- independent lifecycle is useful;
- maintenance cost is justified.

## Likely co-location

Many capabilities should remain in `universal-ontology-engine` while they are tightly coupled to the engine:

- Rust crates;
- normative ontology references;
- schemas;
- engine architecture;
- technical standards;
- tests;
- current CLI implementation;
- engine-specific documentation.

## Reserved, not registered

This document reserves naming direction only. It does not establish:
- GitHub repositories;
- package scopes;
- DNS;
- deployments;
- services.

**Planned means planned.**

# Universe Hierarchy

## Purpose

This document defines the single, language-agnostic ownership and implementation hierarchy governed by Universe Foundation. It is normative across the entire chain; no secondary foundation repository is required.

## Canonical hierarchy

```text
UNIVERSE
└── ECOSYSTEM
    └── ORGANIZATION (optional)
        └── DOMAIN (optional)
            └── PROJECT
                └── REPOSITORY
                    └── SOURCE
                        └── UNIT
                            └── MODULE
                                └── COMPONENT
                                    └── ELEMENT
                                        └── IMPLEMENTATION
```

## Layer semantics

| Layer | Meaning |
|---|---|
| UNIVERSE | Top-level managed boundary containing one or more ecosystems. |
| ECOSYSTEM | Coherent software/product/research boundary within a universe. |
| ORGANIZATION | Ownership or governance boundary. Optional. |
| DOMAIN | Business, technical, research, or product area. Optional. |
| PROJECT | Bounded product, system, initiative, or engineering effort. |
| REPOSITORY | Version-controlled source boundary. |
| SOURCE | Implementation area of a repository, regardless of native directory naming. |
| UNIT | Logical native implementation container inside Source. |
| MODULE | Logical grouping of related implementation inside a Unit. |
| COMPONENT | Cohesive implementation part with a defined responsibility. |
| ELEMENT | Meaningful construct inside a Component, such as a function, method, type, interface, handler, or constant. |
| IMPLEMENTATION | Concrete logic/behavior realizing an Element or Component. |

## Unit and lower layers

`UNIT` through `IMPLEMENTATION` are universal semantic concepts. They are not mandatory directory names and must not force artificial filesystem wrappers.

For example:

```text
UNIT
├── Rust      → crate
├── Node      → package / application / library
├── Go        → package / command / service
├── Python    → package / module
└── Java      → module / package / application
```

Common native directories such as `src`, `apps`, `packages`, `crates`, `libs`, `cmd`, `pkg`, `internal`, `modules`, and `components` remain valid. Native language constructs such as `class`, `struct`, `function`, `method`, `interface`, and `handler` are mappings to the universal vocabulary.

A conforming repository MUST NOT create `unit/`, `module/`, `component/`, `element/`, or `implementation/` solely for hierarchy compliance.

## Optionality and omission

`ORGANIZATION` and `DOMAIN` are optional. Their omission does not collapse the semantic meaning of the layers below them:

```text
ECOSYSTEM → PROJECT → REPOSITORY → SOURCE → UNIT → ...
```

## Supporting dimensions

Documentation, tests, examples, fixtures, tools, scripts, configuration, data, specifications, infrastructure, deployment, and assets are supporting concerns. They are not additional ownership levels in the canonical hierarchy.

## External systems

External systems are relationships, not children of the ownership hierarchy. A relationship may cross ecosystems or reach an external service without transferring ownership.

## Ownership rules

1. Universe Foundation is the single normative foundation for all canonical layers.
2. No ecosystem-level foundation repository is required or implied.
3. Each resource has one clear owning layer.
4. Identity is stable and semantic; local filesystem paths are operational metadata.
5. Native language/framework structure is preserved through explicit mappings.
6. Universal concepts MUST NOT be duplicated as alias directories when they have the same responsibility.
7. Cross-layer references SHOULD use stable IDs rather than path-derived identity.
8. Cross-ecosystem relationships MUST NOT redefine ownership.

# Universe Foundation Contract v0.3

## 1. Scope

This is the single normative foundation contract for the complete software hierarchy managed by a Universe.

It governs the chain from `UNIVERSE` through `IMPLEMENTATION` and does not delegate any canonical layer to another foundation repository.

## 2. Canonical hierarchy

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

`ORGANIZATION` and `DOMAIN` are optional. All other layers form the canonical continuous chain.

## 3. Foundation principle

`universe-foundation` is the only foundation contract required for conformance.

There is no normative dependency on `ecosystem-foundation` or any other lower-level foundation repository. An ecosystem may still contain its own documentation or governance, but those materials are subordinate to this universal contract and cannot redefine the canonical hierarchy.

## 4. Identity

Every canonical entity SHOULD have a stable semantic identifier within its parent scope.

Filesystem paths, directory names, repository names, branch names, and build artifacts are operational references unless explicitly declared as identifiers.

## 5. Layer responsibilities

- `UNIVERSE`: global boundary, registry, topology, discovery, cross-ecosystem coordination.
- `ECOSYSTEM`: coherent product, research, platform, or software boundary.
- `ORGANIZATION`: ownership/governance grouping when applicable.
- `DOMAIN`: bounded business, technical, research, or product area when applicable.
- `PROJECT`: bounded initiative, product, system, or engineering effort.
- `REPOSITORY`: version-controlled implementation boundary.
- `SOURCE`: repository implementation area.
- `UNIT`: logical native implementation container.
- `MODULE`: grouping of related implementation inside a Unit.
- `COMPONENT`: cohesive implementation responsibility.
- `ELEMENT`: smaller meaningful construct within a Component.
- `IMPLEMENTATION`: concrete logic or behavior that realizes an Element or Component.

## 6. Native implementation mapping

The universal vocabulary is semantic and language-agnostic. Native constructs remain valid and are mapped to the canonical layers.

```text
UNIT
├── Rust      → crate
├── Node      → package / application / library
├── Go        → package / command / service
├── Python    → package / module
└── Java      → module / package / application
```

Examples of lower mappings include:

```text
MODULE        → native module / namespace / source grouping
COMPONENT     → class / service / handler / subsystem / feature unit
ELEMENT       → function / method / type / interface / constant / handler
IMPLEMENTATION → concrete executable logic, data flow, or behavior
```

Profiles may refine mappings without changing the universal vocabulary.

## 7. Physical layout rule

Canonical semantic layers MUST NOT become mandatory directory names.

A repository may use native layouts such as:

```text
src/ apps/ packages/ crates/ libs/ cmd/ pkg/ internal/ modules/ components/
```

A repository MUST NOT create `unit/`, `module/`, `component/`, `element/`, or `implementation/` merely to satisfy the hierarchy.

## 8. Supporting concerns

Tests, examples, fixtures, tools, scripts, configuration, data, documentation, specifications, infrastructure, deployment, and assets are supporting dimensions. They are not extra canonical parent layers.

## 9. Registry and discovery

`universe.json` is the top-level registry. It MUST identify ecosystems using stable IDs and operational references.

An ecosystem MAY expose an `ecosystem.json` discovery manifest. Such a manifest is an optional metadata envelope and is not a second foundation contract.

## 10. Relationships

References between entities MUST preserve ownership. Cross-ecosystem relationships point to stable identifiers and do not transfer ownership.

External systems are relationships, not children of the canonical hierarchy.

## 11. Conformance

A conforming implementation MUST:

1. use the canonical vocabulary consistently;
2. preserve native project/repository structures through mappings;
3. avoid duplicate ownership layers and overlapping alias directories;
4. maintain stable identity independent of local paths;
5. validate machine-readable manifests against the current foundation contract;
6. treat this repository as the single normative foundation.

## 12. Change control

Changes to hierarchy, semantics, identity rules, or native mappings require an explicit contract update and corresponding validator/test changes before templates or adopters are changed.

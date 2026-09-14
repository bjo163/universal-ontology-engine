# Universe Foundation Contract v0.3

## 1. Scope

This is the **single normative foundation contract** for the complete software hierarchy managed by a Universe.

It governs identity, ownership, discovery, relationships, validation, and orchestration semantics from `UNIVERSE` down to concrete `IMPLEMENTATION`.

No second foundation repository is required for an Ecosystem, Project, Repository, Source, Unit, Module, Component, Element, or Implementation.

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

`ORGANIZATION` and `DOMAIN` are optional semantic layers. All other hierarchy levels are canonical concepts.

## 3. Identity

Every canonical node SHOULD have a stable identifier within its parent scope.

Filesystem paths, repository names, package names, and native language constructs are operational metadata unless explicitly declared as identity.

A stable identity MUST survive a local directory rename.

## 4. Ownership

Ownership flows strictly downward through the hierarchy.

```text
UNIVERSE → ECOSYSTEM → ORGANIZATION? → DOMAIN? → PROJECT → REPOSITORY → SOURCE → UNIT → MODULE → COMPONENT → ELEMENT → IMPLEMENTATION
```

A resource MUST have one clear owning layer.

Cross-cutting concerns such as tests, documentation, assets, configuration, data, deployment, and infrastructure are supporting dimensions; they do not create additional ownership layers.

## 5. Native compatibility

The foundation is language-agnostic. Native ecosystem concepts remain valid and are mapped to canonical semantic concepts.

```text
UNIT
├── Rust   → crate
├── Node   → package / application / library
├── Go     → package / command / service
├── Python → package / module
└── Java   → module / package / application
```

`MODULE`, `COMPONENT`, `ELEMENT`, and `IMPLEMENTATION` may map to different native constructs depending on language and framework.

The canonical terms MUST NOT be turned into mandatory directory names.

## 6. Repository conformance

A repository may declare a root manifest describing its position in the hierarchy. Native manifests remain where their toolchain requires them.

Examples include `Cargo.toml`, `package.json`, `go.mod`, `pyproject.toml`, and `pom.xml`.

Conformance MUST be achievable without restructuring an otherwise valid native repository merely to satisfy naming aesthetics.

## 7. Discovery

The Universe registry is the entry point for discovery.

Discovery walks downward using explicit stable identifiers and declared parent relationships. Filesystem scanning may discover candidates, but filesystem names alone MUST NOT define identity.

## 8. Relationships

Relationships may connect any compatible canonical nodes or external systems without changing ownership.

External systems are relationships, not children of the ownership tree.

## 9. Validation

Validation MUST check:

- contract version
- canonical hierarchy
- stable identifiers
- parent/child consistency
- uniqueness within scope
- safe operational paths
- absence of competing foundation contracts
- native mapping compatibility

## 10. Single-foundation rule

`universe-foundation` is the sole normative foundation repository.

An ecosystem MAY contain local governance, architecture, standards, or documentation, but none may redefine the canonical Universe hierarchy or claim to be a second foundation contract.

Legacy foundation repositories are compatibility/history artifacts only and are not part of the active foundation chain.

## 11. Change control

Changes to hierarchy, semantics, identity rules, or native mappings require an explicit contract update and corresponding validator/test changes before templates or adopters are changed.

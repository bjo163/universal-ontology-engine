# Universe Contract v0.2

## 1. Scope

This contract governs the top-level Universe layer and its relationship to independently owned Ecosystems.

It does not replace or duplicate the internal contract of any ecosystem.

## 2. Canonical model

```text
UNIVERSE
└── ECOSYSTEM*
    └── PROJECT / REPOSITORY / SOURCE / MODULE
```

`*` means one or more ecosystems may exist in a universe. The lower levels are owned by the ecosystem and its applicable foundation contract.

## 3. Universe identity

A universe MUST have:

- a stable `id`
- a human-readable `name`
- a contract version
- an ecosystem registry

The local filesystem path is operational metadata, not identity.

## 4. Ecosystem identity

Each registered ecosystem MUST have:

- a stable `id`
- a human-readable `name`
- a unique registry entry
- a path or source reference used for discovery

An ecosystem MAY declare a repository containing its own ecosystem-level manifest or foundation material.

## 5. Ownership boundary

Universe Foundation owns:

- universe identity
- ecosystem registry
- ecosystem discovery
- workspace topology
- cross-ecosystem relationships
- cross-ecosystem orchestration boundaries

Universe Foundation does NOT own:

- ecosystem domain semantics
- project semantics
- repository internals
- source code conventions
- module/component design
- language/toolchain choices

Those remain below the ecosystem boundary.

## 6. Workspace standard

A conforming local workspace SHOULD use:

```text
universe/
├── universe manifest
├── ecosystem-<id>/
│   ├── ecosystem manifest (when used)
│   └── <repository directories>
└── ecosystem-<id>/
```

Repositories do not need an extra `repositories/` wrapper. Native repository layouts remain governed by `ecosystem-foundation` or the repository's own native conventions.

A physical layout MAY differ when an orchestration tool requires it, provided registry identity remains stable and explicit.

## 7. Registry and discovery

The registry is the source of truth for ecosystem discovery within the universe.

Discovery MUST use the stable ecosystem `id` and explicit `path` or `source` reference. Filesystem naming alone MUST NOT be treated as identity.

A registry entry MAY include lifecycle, capability, contract, repository, organization, or other cross-ecosystem metadata.

## 8. Relationships

Cross-ecosystem relationships MUST point to ecosystem IDs or another stable identifier.

Relationships MUST NOT redefine ownership of the referenced ecosystem or its repositories.

External systems are represented as relationships and are not automatically children of the universe.

## 9. Foundation compatibility

An ecosystem MAY conform to `ecosystem-foundation`.

When it does, the relationship is:

```text
universe-foundation v0.2
        ↓
ecosystem-foundation v0.x
        ↓
ecosystem repositories
```

Universe Foundation defines the container and discovery contract; Ecosystem Foundation defines the structure below that boundary.

## 10. Non-goals

This contract does not define:

- package managers
- build systems
- programming languages
- CI providers
- cloud providers
- database technology
- application architecture
- repository naming conventions beyond stable registry identity
- project or repository internals

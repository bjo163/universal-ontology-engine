# Universe Contract v0.1

## 1. Scope

This contract governs the top-level Universe layer and its relationship to Ecosystems.

It does not replace or duplicate the internal contract of any ecosystem.

## 2. Canonical model

```text
UNIVERSE
└── ECOSYSTEM*
```

`*` means one or more ecosystems may exist in a universe.

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

An ecosystem MAY have additional metadata such as repository, organization, lifecycle, or capabilities.

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
│   ├── ecosystem manifest
│   └── repositories/
└── ecosystem-<id>/
```

A physical layout MAY differ when an orchestration tool requires it, provided registry identity remains stable and explicit.

## 7. Relationships

Cross-ecosystem relationships MUST point to ecosystem IDs or another stable identifier. They MUST NOT infer ownership from filesystem names alone.

External systems are represented as relationships and are not automatically children of the universe.

## 8. Compatibility

An ecosystem MAY conform to `ecosystem-foundation`.

When it does, the relationship is:

```text
universe-foundation v0.1
        ↓
ecosystem-foundation v0.x
        ↓
ecosystem repositories
```

The universe contract MUST remain compatible with ecosystem autonomy.

## 9. Non-goals

This contract does not define:

- package managers
- build systems
- programming languages
- CI providers
- cloud providers
- database technology
- application architecture
- repository naming conventions beyond stable registry identity

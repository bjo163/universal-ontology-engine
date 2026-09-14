# Universe Hierarchy

## Purpose

The hierarchy defines ownership boundaries above and across software ecosystems. It is intentionally language-agnostic.

## Canonical hierarchy

```text
UNIVERSE
└── ECOSYSTEM
    └── ORGANIZATION (optional / ecosystem-defined)
        └── DOMAIN (optional)
            └── PROJECT
                └── REPOSITORY
                    └── SOURCE
                        └── MODULE / COMPONENT
```

For Universe Foundation, only the `UNIVERSE → ECOSYSTEM` boundary is canonical. Everything below an ecosystem is governed by that ecosystem's foundation contract.

## Universe

A managed top-level boundary containing one or more ecosystems that are related operationally, organizationally, or strategically.

A universe owns cross-ecosystem registry, identity, discovery, workspace conventions, relationships, and orchestration boundaries.

## Ecosystem

A self-contained domain boundary with its own projects, repositories, semantics, and implementation choices.

An ecosystem must remain autonomous below its boundary and may adopt `ecosystem-foundation` or another compatible contract.

## External system

A system may interact with a universe without being owned by it. External systems are relationships, not children in the ownership hierarchy.

## Ownership rules

1. Universe Foundation MUST NOT absorb ecosystem-owned semantics.
2. Universe registry describes ecosystems; it does not redefine their internal structure.
3. An ecosystem MUST be independently identifiable and discoverable.
4. Cross-ecosystem relationships MUST reference stable ecosystem identifiers.
5. Workspace paths are local operational metadata and MUST NOT become identity.
6. Repository names MUST NOT be used as universe-level identity unless explicitly referenced by an ecosystem manifest.
7. A single resource MUST have one clear owning layer.

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
                        └── UNIT
                            └── MODULE
                                └── COMPONENT
                                    └── ELEMENT
                                        └── IMPLEMENTATION
```

For Universe Foundation, only the `UNIVERSE → ECOSYSTEM` boundary is owned here. Everything below an ecosystem is governed by the ecosystem contract while preserving this canonical vocabulary.

## Universal vocabulary

### Universe
A managed top-level boundary containing one or more ecosystems that are related operationally, organizationally, or strategically.

### Ecosystem
A self-contained boundary with its own projects, repositories, semantics, and implementation choices.

### Organization
The ownership and governance boundary within an ecosystem. Its exact administrative form is ecosystem-defined.

### Domain
A business, technical, research, or product area. Domain is optional.

### Project
A product, system, initiative, or bounded engineering effort.

### Repository
A version-controlled implementation unit.

### Source
The implementation area of a repository, regardless of its native filesystem naming.

### Unit
A native implementation container recognized as a logical unit inside Source. Examples include a Rust crate, a Node package, a Go package, a Python package, an application, a library, or a service.

`UNIT` is a universal semantic term, not a required directory name.

### Module
A logical grouping of related implementation within a Unit.

### Component
A cohesive implementation part with a defined responsibility. A component may be represented by different native constructs in different languages or frameworks.

### Element
A smaller meaningful implementation construct inside a Component, such as a function, method, type, interface, handler, or constant.

### Implementation
The concrete logic or behavior that realizes an Element or Component.

## Native mapping rule

Native terms remain valid inside repositories and tools:

```text
UNIT
├── Rust      → crate
├── Node      → package / application / library
├── Go        → package / command / service
├── Python    → package / module
└── Java      → module / package / application
```

These native terms are mappings, not replacements for canonical hierarchy vocabulary.

A repository MUST NOT be required to create physical directories named `unit/`, `module/`, `component/`, `element/`, or `implementation/` merely to conform to this hierarchy.

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
8. Universal terminology MUST remain separate from native filesystem and language terminology.

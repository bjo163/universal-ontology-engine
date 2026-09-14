# OX-DX Naming Conventions

This guide is for future names. It does not rename existing repositories, crates, ontology levels, commands, or contracts.

## Product naming hierarchy

```text
OX-DX
  ↓
Universal Ontology & Experience Engine
  ↓
Universal Ontology Engine
  ↓
Rust crates / tools
```

- **OX-DX** = public brand/system identity.
- **Universal Ontology & Experience Engine** = conceptual product descriptor.
- **Universal Ontology Engine** = Rust implementation in this repository.
- **Rust crates/tools** = implementation units with existing technical names.

## Future naming rules

Names should be:

- short;
- distinct;
- memorable;
- technical;
- pronounceable when practical;
- consistent with OX-DX;
- specific enough to search.

## Avoid names that

- imitate scientific authority without scientific meaning;
- borrow religious authority;
- use hype words as identity;
- duplicate or shadow an existing canonical ontology type;
- reuse an edge-kind name for an unrelated product;
- imply capability before implementation exists;
- create a second competing brand for the same thing.

## Technical namespaces

Canonical ontology TYPE names and edge kinds are reserved technical vocabulary.

A future tool named `BIT`, `CONTAINS`, or `OBSERVED_AT` would create avoidable ambiguity unless it is literally the corresponding technical surface.

## Versioned names

Put version numbers in contracts/releases, not permanent brand names, unless the version is itself the compatibility boundary.

Good: `Universal Ontology v1.0.0`.

Avoid: `OX-DX NextGen 2.0 Ultra`.

## Test

Before approving a new name, ask:

1. Does an existing OX-DX term already mean this?
2. Would a contributor mistake it for an ontology TYPE, KIND, or edge?
3. Does the name imply more maturity than the implementation?
4. Can it survive outside a launch campaign?

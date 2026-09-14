# OX-DX Brand Architecture

Master brand authority: `../OX-DX-BRAND-BIBLE-v1.0.md`

This file remains the canonical specialist for naming/ownership architecture.

This is the master naming and ownership reference for the OX-DX ecosystem.

It governs **how things are named and related**. It does not redefine ontology semantics or implementation architecture.

## Canonical identity stack

```text
BRAND
OX-DX
  │
  ▼
PRODUCT / SYSTEM DESCRIPTOR
Universal Ontology & Experience Engine
  │
  ▼
TECHNICAL ENGINE
Universal Ontology Engine
  │
  ▼
CURRENT IMPLEMENTATION
Rust workspace in universal-ontology-engine
```

These names are related. They are not interchangeable aliases.

| Layer | Canonical name | Meaning |
|---|---|---|
| Brand | **OX-DX** | Umbrella public identity users recognize. |
| Product/system | **Universal Ontology & Experience Engine** | Conceptual descriptor for the broader OX-DX system and experience horizon. |
| Engine | **Universal Ontology Engine** | Current technical computational implementation. |
| Repository | `universal-ontology-engine` | Current GitHub repository containing the Rust engine and normative technical material. |
| Package/crate | existing technical package names | Implementation components such as `ontology-graph`. |
| Website | **OX-DX** | Future public web property identity; no website is established by this document. |
| Documentation property | **OX-DX Docs** | Future public documentation identity; existing in-repo docs remain valid. |

**A repository name does not automatically define the public brand.**

## Core brand hierarchy

```text
OX-DX
│
├── Universal Ontology & Experience Engine
│   └── Universal Ontology Engine
│       ├── graph
│       ├── discovery
│       ├── language adapters
│       └── future runtime / resolution capabilities
│
├── OX-DX Specification
│   ├── Universal Ontology
│   └── schemas
│
├── OX-DX Tools
│   ├── CLI
│   ├── inspectors
│   └── utilities
│
├── OX-DX Interfaces
│   └── future web / API / visualization surfaces
│
├── OX-DX Documentation
│
└── OX-DX Community
```

This is a governance model, not a requirement to create one repository per box.

## Six operating layers

### Layer 1 — OX-DX Brand

The identity users recognize.

Owns:
- public identity;
- brand promise;
- visual identity;
- communication language;
- approved taglines.

Does not own:
- ontology definitions;
- schemas;
- graph semantics;
- code behavior.

### Layer 2 — Products / Systems

Major user-facing systems under OX-DX.

Current conceptual system:

**Universal Ontology & Experience Engine**

A product/system describes a coherent user-visible capability boundary. It must not duplicate another product's responsibility.

### Layer 3 — Engines

Core computational implementations.

Current engine:

**Universal Ontology Engine**

The engine performs computation and implements technical contracts. It does not redefine the brand or normative ontology by marketing language.

### Layer 4 — Tools

CLI commands, inspectors, generators, converters, and utilities.

Tools expose capabilities. They do not own the ontology.

Prefer descriptive, short, technically honest names such as:
`validate`, `levels`, `inspect`, `discover`, `parse`, and future `query` only when that capability exists.

### Layer 5 — Interfaces

Web, UI, API, visualization, and other experience surfaces.

Interfaces present or operate capabilities. They do not redefine technical semantics.

Future public extensions may use **OX-DX + clear noun**, for example **OX-DX Web** or **OX-DX Explorer**, only when a real interface with a distinct responsibility exists.

### Layer 6 — Infrastructure / Implementation

Crates, libraries, schemas, internal modules, generated artifacts, and implementation packages.

Names here should become **more technically precise**, not more branded.

Examples:
- `ontology-core`
- `ontology-graph`
- `ontology-discovery`
- `universal-ontology-v1.0.json`

## Current implementation truth

Verified workspace:

```text
universal-ontology-engine
└── crates/
    ├── ontology-core
    ├── ontology-registry
    ├── ontology-graph
    ├── ontology-discovery
    ├── ontology-rust
    ├── ontology-language
    └── ontology-cli/
        └── package + binary: ontology-engine
```

Do not rename these in a brand-governance task.

## Specification architecture

Public architecture label:

**OX-DX Specification**

Normative technical identity:

**Universal Ontology**

Canonical files remain:

- `specifications/universal-ontology-v1.0.json`
- `specifications/universal-ontology-v1.0.md`
- `schemas/universal-ontology.schema.json`

**OX-DX does not replace the technical specification name.**

Brand names the ecosystem. Universal Ontology names the normative ontology.

## Engine naming rule

Use:

- **OX-DX** when speaking about the overall brand/system identity.
- **Universal Ontology & Experience Engine** when speaking about the conceptual product/system.
- **Universal Ontology Engine** when speaking about the Rust technical engine.
- `ontology-engine` when speaking about the current CLI binary/package where exact implementation naming matters.

Avoid random alternation such as:
- “OX engine”;
- “Universal engine”;
- “DX engine”;
- “OX-DX engine”;

unless a future formally named component deliberately adopts such a name.

## Tool naming rule

Tools should generally be:
- descriptive;
- short;
- verb-oriented when appropriate;
- honest about capability.

OX-DX may act as the public namespace while technical commands remain descriptive.

Conceptual future public surface:

```text
ox-dx validate
ox-dx levels
ox-dx inspect
ox-dx discover
```

This is **direction only**.

Current implementation remains:

```text
ontology-engine validate
ontology-engine levels
ontology-engine inspect
ontology-engine discover
```

No CLI rename is authorized by this document.

## Interface naming rule

Use **OX-DX + clear noun** when a distinct public interface exists.

Potential future examples:

- OX-DX Web
- OX-DX Docs
- OX-DX Explorer
- OX-DX Console

These are naming patterns, not existing products.

Every interface must have:
1. one defined audience;
2. one defined responsibility;
3. one technical owner;
4. no duplicate ownership with another surface.

## Documentation naming

Existing technical documentation remains inside `universal-ontology-engine/docs/` when it is tightly coupled to engine code, standards, tests, or release behavior.

A future standalone public documentation property may be named **OX-DX Docs** and may use a future repository such as `ox-dx-docs` only when the separation has a real operational reason.

Do not move documentation merely to make the brand architecture look symmetrical.

## Community naming

Use **OX-DX Community** as the umbrella term for future discussions, contributor portals, forums, chat, or community programs.

Do not create a second community brand.

Community channels do not own technical contracts.

## Brand extension rule

Default public extension pattern:

**OX-DX + clear noun**

Good when justified:
- OX-DX Web
- OX-DX Docs
- OX-DX CLI
- OX-DX SDK

Avoid naming for marketing's sake:
- OX-DX AI
- OX-DX Pro
- OX-DX Ultra
- OX-DX Quantum
- OX-DX X
- OX-DX 360

A suffix is valid only when it corresponds to a real, separately owned responsibility.

## Prefix / suffix architecture

| Form | Use |
|---|---|
| **OX-DX** | Public umbrella brand. |
| `ox-dx-*` | Public ecosystem repository/package namespace when a separate public surface is justified. |
| `ontology-*` | Technical implementation namespace for engine components where descriptive naming is clearer. |
| `universal-ontology-*` | Normative ontology/specification lineage or closely related technical artifacts where established. |
| `-engine` | Core computation implementation. |
| `-cli` | Command-line interface/tool boundary. |
| `-web` | Public web property implementation. |
| `-docs` | Dedicated public documentation property. |
| `-sdk` | Public software-development kit when one actually exists. |

Do not force `ox-dx-` onto every internal component.

## One capability, one owner

Capability ownership is explicit:

```text
ontology definition   → Specification
ontology computation  → Engine
CLI interaction       → Tool
public presentation   → Interface / Web
documentation         → Documentation
visual identity       → Brand assets
community process     → Community
```

Prohibited ownership drift:

- website redefining ontology;
- CLI redefining ontology;
- docs redefining schema;
- marketing redefining technical semantics;
- multiple products claiming the same capability.

See `responsibility-matrix.md`.

## Version architecture

Keep these identities independent:

```text
OX-DX                         → brand identity; normally unversioned
Universal Ontology & Experience Engine → product/system identity
Universal Ontology Engine    → software release train
Universal Ontology           → normative ontology version
schemas                      → schema compatibility/version as defined by contract
```

At the repository audit on 2026-09-14:
- engine stable release in the Brand Bible audit snapshot: **v0.1.7**;
- ontology contract: **v1.0.0**.

Do not infer software maturity from the ontology's `1.0.0`, or ontology compatibility from the engine's `0.x` release.

See `versioning.md`.

## Domain architecture

Target public identity:

**OX-DX**

Target domain:

`ox-dx.com`

This is a **planning target only**. This architecture makes no claim that the domain is registered, delegated, active, deployed, or owned.

Potential future subdomains, only when required:
- `docs.ox-dx.com`
- `api.ox-dx.com`
- `app.ox-dx.com`

Do not create empty subdomain architecture in advance.

See `domain-architecture.md`.

## Quranic naming boundary

Quranic inspiration belongs primarily in philosophy, manifesto, worldview, reflection, and visual geometry.

Do **not**:
- casually use sacred names for tools, algorithms, services, or commercial products;
- manufacture Arabic names to make technology appear profound;
- claim divine authority through product names.

Technical naming remains precise and neutral.

## Gen-Z / grunge naming boundary

Gen-Z clarity and grunge attitude belong in voice, presentation, campaigns, and visual identity.

Foundational infrastructure must remain understandable.

Avoid meme names for:
- ontology;
- schema;
- graph;
- engine;
- identity;
- core libraries.

## Intelligent naming rule

**The more foundational the component, the more precise its name should be.**

```text
Brand:   OX-DX
System:  Universal Ontology & Experience Engine
Engine:  Universal Ontology Engine
Crate:   ontology-graph
File:    universal-ontology-v1.0.json
```

This is the default naming progression for the ecosystem.

## Governance test for every new name

Before creating a repository, package, product, tool, interface, or public label, answer:

1. What exact responsibility does it own?
2. Does an existing component already own that capability?
3. Is this a brand name, product name, technical implementation name, or repository identifier?
4. Is a new repository actually required?
5. Does the name collide with an ontology TYPE, KIND, edge, schema, or existing technical term?
6. Does it imply capability or maturity that is not implemented?
7. Is the name technically neutral with respect to Quranic inspiration?
8. Will existing users need aliases or migration?
9. Can the name survive without launch hype?
10. Is the result simpler than leaving the current name alone?

If the final answer to question 10 is **no**, do not rename.

**OX-DX must grow as a system, not as a pile of names.**

# Universe Foundation

Canonical foundation for the Universal Ontology and the systems that consume it.

> **Quran Inspired ALLAH my Beloved**
>
> This is the philosophical inspiration statement of the foundation. It is not a claim that the technical ontology is prescribed by the Qur'an.

## Canonical ontology

**Universal Ontology v1.0.0** defines a **7 Zones × 7 Levels = 49-level canonical resolution spine**.

The spine is a vocabulary and ordering system, not a mandatory filesystem tree. The actual model is a graph that distinguishes containment, references, semantic projection, runtime observation, and physical representation.

```text
ZONE 1  EXISTENCE
01 UNIVERSE
02 CREATION
03 ORDER
04 REALITY
05 REALM
06 WORLD
07 DOMAIN

ZONE 2  CONTEXT
08 ECOSYSTEM
09 ORGANIZATION
10 COMMUNITY
11 REGION
12 ENVIRONMENT
13 NETWORK
14 CONTEXT

ZONE 3  INTENT
15 PURPOSE
16 MISSION
17 OBJECTIVE
18 PROGRAM
19 PROJECT
20 PRODUCT
21 SYSTEM

ZONE 4  STRUCTURE
22 REPOSITORY
23 SOURCE
24 UNIT
25 MODULE
26 SUBSYSTEM
27 COMPONENT
28 ELEMENT

ZONE 5  SEMANTIC
29 SYMBOL
30 ENTITY
31 PROPERTY
32 RELATION
33 OPERATION
34 FUNCTION
35 BEHAVIOR

ZONE 6  DYNAMIC
36 STATE
37 EVENT
38 PROCESS
39 FLOW
40 TRANSITION
41 ACTION
42 EXECUTION

ZONE 7  REPRESENTATION
43 INSTRUCTION
44 EXPRESSION
45 VALUE
46 DATA
47 TOKEN
48 CHARACTER
49 BIT
```

The normative machine-readable registry is `specifications/universal-ontology-v1.0.json`; the normative explanation is `specifications/universal-ontology-v1.0.md`.

## Critical architectural rule

The 49 levels MUST NOT be implemented as 49 physical directories or as one rigid AST hierarchy.

```text
CONTAINMENT   = ownership / structure
PROJECTION    = one resource viewed at another ontology level
RELATION      = semantic or dynamic connection
REPRESENTATION = concrete encoding
```

This allows a Rust function to project into instructions, expressions, values, data, tokens, characters, and bits without claiming that all of those are literal child folders.

## Foundation vs engine

```text
universe-foundation
    = vocabulary + schemas + invariants + contracts

universal-ontology-engine (Rust)
    = discovery + parsing + graph + projection + runtime + binary/bit inspection
```

The engine is deliberately a separate implementation project. The foundation remains implementation-neutral.

## Identity

Every discovered resource SHOULD have a stable semantic identifier scoped to its owning boundary. Paths, filenames, offsets, and timestamps are provenance/observation metadata, not a substitute for stable identity.

## Native structures remain valid

The foundation never requires artificial ontology-named directories such as `unit/`, `module/`, `component/`, `element/`, or `implementation/`.

Examples of native mappings include:

```text
Rust       crate/module/item/function
Node       package/app/module/function
Go         package/cmd/type/function
Python     package/module/class/function
Java       module/package/class/method
```

These structures are evidence for ontology projection, not replacements for the canonical vocabulary.

## Compatibility

Older foundation contracts that describe the earlier 11/17-level hierarchy are retained only as migration/compatibility material. They MUST NOT be treated as a competing canonical ontology.

Migration MUST preserve existing stable IDs and repository paths.

## Engine target

The reference Rust engine is designed to traverse from managed universe boundaries to bounded machine representation, including bit-level inspection when the input format permits it.

See `standards/universal-ontology-engine.md` for the architecture.

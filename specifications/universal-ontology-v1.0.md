# Universal Ontology v1.0

> Quran Inspired ALLAH my Beloved
>
> This statement records the philosophical inspiration of the foundation. It is not a claim that the 49-level technical ontology is prescribed by the Qur'an.

## Status

**Version:** 1.0.0
**Canonical shape:** 7 zones × 7 levels = 49 canonical levels
**Role:** normative ontology contract

## Core decision

Universal Ontology is a **resolution spine over a graph**, not a mandatory filesystem tree.

The 49 levels provide a stable vocabulary for describing a resource from the broadest managed boundary down to machine representation. Real resources may skip physical layers, project across multiple semantic/dynamic views, or expose more than one representation.

Therefore:

- hierarchy gives canonical type order;
- containment gives parent/child ownership where applicable;
- projection describes a resource at another ontology level;
- relation describes semantic or dynamic connections;
- representation describes how a resource is encoded physically.

No repository is required to contain directories named after ontology types.

## Canonical 49 levels

### Zone 1 — EXISTENCE

| # | Type | Canonical meaning |
|---:|---|---|
| 01 | UNIVERSE | Total managed boundary for the ontology instance. |
| 02 | CREATION | Origin, generation, or establishment boundary of a modeled thing. |
| 03 | ORDER | Governing arrangement, rule, or organizing structure. |
| 04 | REALITY | The modeled state or world being represented. |
| 05 | REALM | A bounded class or sphere within the modeled reality. |
| 06 | WORLD | A coherent environment or operationally meaningful whole. |
| 07 | DOMAIN | A bounded subject, concern, or semantic area. |

### Zone 2 — CONTEXT

| # | Type | Canonical meaning |
|---:|---|---|
| 08 | ECOSYSTEM | Coherent collection of related systems, products, or resources. |
| 09 | ORGANIZATION | Ownership or governance boundary. |
| 10 | COMMUNITY | Group of participating people, agents, or stakeholders. |
| 11 | REGION | Bounded scope by place, partition, tenancy, or deployment locality. |
| 12 | ENVIRONMENT | Runtime, development, test, production, or other execution context. |
| 13 | NETWORK | Connected topology of participating resources. |
| 14 | CONTEXT | Immediate contextual boundary required to interpret a resource. |

### Zone 3 — INTENT

| # | Type | Canonical meaning |
|---:|---|---|
| 15 | PURPOSE | Fundamental reason for existence. |
| 16 | MISSION | Intended directed outcome or responsibility. |
| 17 | OBJECTIVE | Concrete target that can be evaluated. |
| 18 | PROGRAM | Coordinated body of related objectives or work. |
| 19 | PROJECT | Bounded initiative or engineering effort. |
| 20 | PRODUCT | Delivered capability or user-facing result. |
| 21 | SYSTEM | Coordinated set of capabilities operating as a whole. |

### Zone 4 — STRUCTURE

| # | Type | Canonical meaning |
|---:|---|---|
| 22 | REPOSITORY | Version-controlled artifact boundary. |
| 23 | SOURCE | Source-material boundary from which implementation can be derived. |
| 24 | UNIT | Native implementation container such as crate, package, module root, command, or service. |
| 25 | MODULE | Cohesive logical grouping inside a unit. |
| 26 | SUBSYSTEM | Coordinated structural part with a distinct responsibility. |
| 27 | COMPONENT | Cohesive implementation part with a defined responsibility. |
| 28 | ELEMENT | Addressable construct inside a component or structural scope. |

### Zone 5 — SEMANTIC

| # | Type | Canonical meaning |
|---:|---|---|
| 29 | SYMBOL | Named or otherwise identifiable semantic reference. |
| 30 | ENTITY | Distinct thing represented in the model. |
| 31 | PROPERTY | Characteristic, attribute, or measurable feature of an entity. |
| 32 | RELATION | Semantic connection between identifiable resources. |
| 33 | OPERATION | Defined transformation or capability over inputs/state. |
| 34 | FUNCTION | Reusable mapping or computational behavior with defined inputs/outputs. |
| 35 | BEHAVIOR | Observable semantic effect of a resource or operation. |

### Zone 6 — DYNAMIC

| # | Type | Canonical meaning |
|---:|---|---|
| 36 | STATE | Condition of a resource at a point or interval. |
| 37 | EVENT | Occurrence that can be observed or recorded. |
| 38 | PROCESS | Ongoing or ordered activity that transforms state/resources. |
| 39 | FLOW | Ordered movement of control, data, or resources. |
| 40 | TRANSITION | Change from one state to another. |
| 41 | ACTION | An invoked or performed unit of work. |
| 42 | EXECUTION | A concrete runtime realization of computation. |

### Zone 7 — REPRESENTATION

| # | Type | Canonical meaning |
|---:|---|---|
| 43 | INSTRUCTION | Machine- or runtime-level directive describing work to perform. |
| 44 | EXPRESSION | Syntactic or evaluable form that denotes computation/value. |
| 45 | VALUE | Concrete result or literal datum with an interpretable type/meaning. |
| 46 | DATA | Encoded or structured information being stored, moved, or processed. |
| 47 | TOKEN | Lexical or protocol-level unit of representation. |
| 48 | CHARACTER | Encoded character/code-point or character-level representation. |
| 49 | BIT | Single binary information unit. |

## Type, kind, identity

`type` is one of the 49 canonical ontology levels. `kind` is a specialization within a type and MUST NOT create a new canonical level.

Examples:

```text
TYPE=UNIT       KIND=rust-crate
TYPE=UNIT       KIND=node-package
TYPE=ELEMENT    KIND=function
TYPE=ELEMENT    KIND=struct
TYPE=EVENT      KIND=http-request
TYPE=DATA       KIND=utf8-bytes
```

A `kind` MUST have exactly one owning `type` in the canonical registry.

Identity is stable and semantic. Paths, line numbers, offsets, and filenames are observations about a resource and MUST NOT be used as the sole identity when a stable identifier can be established.

## Containment vs projection

The canonical sequence is not proof that every adjacent pair is a literal parent/child relation.

Examples:

```text
PROJECT ─contains─> REPOSITORY
REPOSITORY ─contains─> SOURCE
SOURCE ─contains─> UNIT
UNIT ─contains─> MODULE

FUNCTION ─projects-to─> INSTRUCTION
FUNCTION ─projects-to─> EXPRESSION
EXPRESSION ─projects-to─> VALUE
VALUE ─represented-as─> DATA
DATA ─lexically-or-encoded-as─> TOKEN
TOKEN ─encoded-as─> CHARACTER
CHARACTER ─encoded-as─> BIT
```

This rule prevents semantic and representation concepts from being incorrectly treated as filesystem children.

## Relation classes

The engine MUST distinguish at least these edge classes:

- `contains` — ownership/structural containment;
- `references` — symbolic or informational reference;
- `specializes` — kind/type specialization;
- `depends_on` — dependency relation;
- `invokes` — action/operation invocation;
- `produces` — output creation;
- `consumes` — input consumption;
- `causes` — causal/dynamic relation;
- `projects_to` — semantic/dynamic projection into another ontology level;
- `represented_as` — concrete encoding/representation relation;
- `observed_at` — evidence or observation linkage.

External systems remain relations, never implicit children.

## Optionality

A resource MAY omit intermediate levels in its physical realization. The engine SHOULD retain the canonical type and report omitted levels as `unmaterialized`, not as errors.

## Language neutrality

Native language structures remain authoritative within their own language. Universal Ontology supplies an interoperability vocabulary, not a replacement for language semantics.

Examples:

```text
Rust crate          → UNIT
Rust module         → MODULE
Rust struct/enum    → ELEMENT or COMPONENT, according to scope
Rust function       → ELEMENT / FUNCTION projection
LLVM instruction    → INSTRUCTION
ELF bytes           → DATA
UTF-8 code unit     → DATA / CHARACTER projection
binary bit          → BIT
```

## Conformance

A conforming implementation MUST:

1. know all 49 canonical types;
2. keep type and kind namespaces distinct;
3. preserve stable identity;
4. distinguish containment from projection and representation;
5. permit omitted/unmaterialized ontology layers;
6. never require ontology-named directories;
7. expose deterministic traversal and validation;
8. never silently reinterpret one type as another.

## Non-goals

Universal Ontology does not define:

- a universal programming language;
- a universal filesystem layout;
- one AST for every language;
- a universal database schema;
- theological doctrine;
- a requirement that every resource physically exists at every level.

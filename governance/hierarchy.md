# Universal Ontology Hierarchy v1.0

## Purpose

This document defines the canonical 49-level ontology governed by Universe Foundation.

> Quran Inspired ALLAH my Beloved
>
> The statement is a philosophical inspiration statement, not a claim that the technical hierarchy is prescribed by the Qur'an.

## Canonical resolution spine

The ontology has exactly **7 zones × 7 levels = 49 canonical levels**.

```text
01 UNIVERSE
02 CREATION
03 ORDER
04 REALITY
05 REALM
06 WORLD
07 DOMAIN
08 ECOSYSTEM
09 ORGANIZATION
10 COMMUNITY
11 REGION
12 ENVIRONMENT
13 NETWORK
14 CONTEXT
15 PURPOSE
16 MISSION
17 OBJECTIVE
18 PROGRAM
19 PROJECT
20 PRODUCT
21 SYSTEM
22 REPOSITORY
23 SOURCE
24 UNIT
25 MODULE
26 SUBSYSTEM
27 COMPONENT
28 ELEMENT
29 SYMBOL
30 ENTITY
31 PROPERTY
32 RELATION
33 OPERATION
34 FUNCTION
35 BEHAVIOR
36 STATE
37 EVENT
38 PROCESS
39 FLOW
40 TRANSITION
41 ACTION
42 EXECUTION
43 INSTRUCTION
44 EXPRESSION
45 VALUE
46 DATA
47 TOKEN
48 CHARACTER
49 BIT
```

## Seven zones

```text
EXISTENCE      01–07
CONTEXT        08–14
INTENT         15–21
STRUCTURE      22–28
SEMANTIC       29–35
DYNAMIC        36–42
REPRESENTATION 43–49
```

## Important mathematical distinction

The 49 levels form a **canonical ordered resolution spine**. They do not imply that every real resource has 49 physical children.

Three relations must remain distinct:

```text
CONTAINMENT
  resource owns/contains another resource

PROJECTION
  one resource is interpreted at another ontology level

REPRESENTATION
  a resource is encoded or materialized in another representation
```

Semantic and dynamic levels therefore use graph edges rather than artificial nesting.

For example:

```text
PROJECT
  └─contains→ REPOSITORY
       └─contains→ SOURCE
            └─contains→ UNIT
                 └─contains→ MODULE
                      └─contains→ COMPONENT
                           └─contains→ ELEMENT

ELEMENT --projects_to→ FUNCTION
FUNCTION --projects_to→ INSTRUCTION
FUNCTION --projects_to→ EXPRESSION
EXPRESSION --projects_to→ VALUE
VALUE --represented_as→ DATA
DATA --represented_as→ TOKEN
TOKEN --represented_as→ CHARACTER
CHARACTER --represented_as→ BIT
```

A path may omit intermediate levels. The omission means **unmaterialized/unobserved**, not invalid.

## Type vs kind

`TYPE` is one of the 49 canonical levels.

`KIND` is a specialization inside a type.

```text
UNIT + rust-crate
UNIT + node-package
ELEMENT + function
ELEMENT + struct
EVENT + http-request
DATA + utf8-bytes
```

A kind MUST belong to exactly one canonical type and MUST NOT become a hidden 50th level.

## Identity

Every node SHOULD have a stable semantic identifier within its declared ownership scope.

The following are provenance, not universal identity:

- filesystem path;
- filename;
- line/column span;
- byte offset;
- process ID;
- timestamp.

The engine MAY use these values to construct deterministic identifiers when a stronger source identifier does not exist, but the identity policy must be explicit.

## Native language mapping

Native structures remain valid. The ontology is a compatibility vocabulary across languages.

```text
Rust       crate → UNIT
           mod   → MODULE
           item  → ELEMENT
           fn    → FUNCTION projection

Node       package/app → UNIT
           module      → MODULE
           function    → FUNCTION projection

Go         package     → UNIT
           type        → ELEMENT/COMPONENT projection
           function    → FUNCTION projection

Python     package     → UNIT
           module      → MODULE
           function    → FUNCTION projection

Java       module      → UNIT
           package     → MODULE
           class/method → ELEMENT/semantic projections
```

No repository needs directories named `unit`, `module`, `component`, `element`, or `implementation` merely to comply.

## External systems

External systems are graph relations, not implicit children. Ownership does not transfer because a node references, invokes, consumes, produces, or communicates with an external resource.

## Supporting concerns

Documentation, tests, examples, fixtures, tools, scripts, configuration, infrastructure, deployment, assets, and telemetry are supporting dimensions unless an observed instance independently maps to a canonical ontology type.

## Conformance rules

1. Exactly 49 canonical types exist in v1.0.0.
2. Every canonical type has one index and one zone.
3. Type and kind namespaces are separate.
4. Containment, projection, and representation are distinct edge classes.
5. Intermediate levels may be unmaterialized.
6. Canonical types never require canonical directory names.
7. Stable identity must be preserved during migration.
8. Evidence/provenance must remain traceable for discovered lower-level nodes.
9. External references do not change ownership.
10. Older 11/17-level contracts are compatibility artifacts only and are not competing normative ontologies.

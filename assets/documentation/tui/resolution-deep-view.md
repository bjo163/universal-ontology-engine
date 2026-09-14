# OX-DX 49-Level Deep Inspection View

This view inspects one canonical level with context.

## Required fields

Show when supported:
- level number;
- TYPE name;
- zone;
- canonical definition;
- current object(s) at this TYPE;
- related evidence;
- typed relationships;
- provenance;
- materialization state.

Only canonical definitions from the normative ontology may appear as ontology descriptions.

## Example structure

```text
RESOLUTION  ◆ L22 REPOSITORY · Z4 STRUCTURE

Repository
Version-controlled artifact boundary.

EVIDENCE
  ◆ universal-ontology-engine
    source  repository discovery
    state   materialized

RELATIONS
  contains       → SOURCE (12)
  depends_on     → [only if actual edges exist]

PROVENANCE
  source         <actual source>
```

Do not use sample counts or relations in production unless actual data supplies them.

## Neighbor context

```text
L21 SYSTEM
◆ L22 REPOSITORY
L23 SOURCE
```

The canonical ordering provides resolution context.

It does not prove containment between adjacent TYPEs.

## Gap behavior

If L23 is unmaterialized:

```text
◆ L22 REPOSITORY
  L23 SOURCE        UNMATERIALIZED
  L24 UNIT          evidence available
```

Do not treat the gap as error.

## Source of definitions

Future implementation should read canonical definitions from the registry/specification rather than duplicating strings in TUI code.

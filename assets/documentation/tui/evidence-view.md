# OX-DX TUI Evidence View

**EVIDENCE FIRST.**

Priority:

1. native evidence;
2. classification;
3. relation;
4. provenance;
5. interpretation.

## Source-first composition

```text
EVIDENCE  ◆ focused observation

SOURCE
  41 | pub enum EdgeKind {
  42 |     Contains,
  43 |     References,
     |     ^^^^^^^^^^

CLASSIFICATION
  TYPE        ELEMENT
  KIND        enum
  STATUS      OBSERVED

RELATIONS
  observed_at → <source observation>

PROVENANCE
  file        crates/ontology-core/src/lib.rs
  span        41:1–...
```

Example is structural; production values must come from real evidence.

## Rules

- native source receives the largest readable area;
- interpretation never overwrites source;
- classification is adjacent/secondary;
- provenance remains visible/recoverable;
- relation type is explicit;
- uncertainty remains explicit.

## Compact mode

At 80×24:
- source excerpt;
- classification one-line summary;
- status;
- one provenance line;
- deeper details on demand.

## Selection

Selecting interpretation must not hide source.

## Unknown

If classification is not established:

```text
CLASSIFICATION
  UNKNOWN · no supported classification observed
```

Do not guess.

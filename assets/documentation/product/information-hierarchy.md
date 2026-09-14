# OX-DX Product Information Hierarchy

These levels are **UI information hierarchy only**.

They are not ontology levels.

## LEVEL 0 — Identity / orientation

Answers:
- Where am I?
- Which OX-DX surface is this?
- What scope/repository/context is active?

## LEVEL 1 — Current object or context

The primary object under inspection:
- node;
- source;
- relation;
- event;
- representation;
- context.

## LEVEL 2 — Evidence

What native or observed material supports what is shown?

Prioritize source-visible evidence before interpretive summaries.

## LEVEL 3 — Relationships

Expose:
- containment;
- projection;
- representation;
- observation;
- reference/dependency/etc. where technically available.

Do not flatten every relation into hierarchy.

## LEVEL 4 — Observation

Runtime or other observation overlays remain visually distinct from structural identity.

## LEVEL 5 — Representation

Show concrete representations/encodings separately from semantic identity.

## LEVEL 6 — Detailed resolution

Expose finer structural or representational detail while retaining a recoverable parent context.

## LEVEL 7 — Raw source / provenance

Raw source, span/range, origin, revision, adapter, timestamp, or other provenance when available.

## Navigation between levels

Users move through:
- **scope** — what context is active;
- **focus** — what object is primary;
- **trace** — where evidence/relations come from;
- **resolution** — how specific the inspection becomes.

Do not represent this hierarchy as a permanent vertical stack.

A task may surface Level 3 before Level 2 if the user starts from a relationship, but the evidence path must remain recoverable.

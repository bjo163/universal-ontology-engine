# OX-DX Provenance UI

**PROVENANCE IS NOT IDENTITY.**

Provenance communicates where, when, how, and from what source when those fields actually exist.

## Canonical visual behavior

Use a Provenance Rail or Tail:
- secondary visual weight;
- offset from primary identity geometry;
- explicit origin/source notation;
- deterministic interruption permitted.

Do not place the object's identity inside the provenance rail.

## Information priority

Recommended order:
1. source identifier, path, or URI;
2. revision/version when available;
3. span/range/location;
4. adapter/observer;
5. timestamp only when meaningful;
6. warnings/errors affecting interpretation.

Never fabricate absent fields.

## Multiple sources

Represent multiple provenance sources separately. Do not merge them into one origin if that destroys traceability.

## Identity rule

If provenance changes while semantic identity stays stable, the primary identity geometry should remain stable.

## Accessibility

Provenance must also be readable as text. A visual rail is enhancement, not the only carrier of source information.

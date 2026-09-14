# OX-DX UI Layers

The UI is layered by meaning, not by a fixed dashboard template.

```text
┌─────────────────────────────────────────┐
│ CONTEXT                                 │
├─────────────────────────────────────────┤
│ PRIMARY OBJECT                          │
├─────────────────────────────────────────┤
│ EVIDENCE / RELATIONS                    │
├─────────────────────────────────────────┤
│ OBSERVATION                             │
├─────────────────────────────────────────┤
│ REPRESENTATION                          │
├─────────────────────────────────────────┤
│ PROVENANCE / RAW DETAIL                 │
└─────────────────────────────────────────┘
```

This diagram describes conceptual priority only.

## Context

Repository, scope, selected resolution, route, filter, or active comparison.

## Primary object

The current inspection target.

## Evidence / relations

Source evidence and typed relationships supporting or connecting the primary object.

## Observation

Dynamic or other observation material, visually separated from structure.

## Representation

Encoding/concrete form when available.

## Provenance / raw detail

Where/how evidence was acquired.

## Layer rules

- Do not show every layer simultaneously by default.
- Layers expand according to task and screen capacity.
- Hidden context must remain recoverable.
- Observation overlays do not become containment.
- Provenance remains subordinate but reachable.
- Representation never replaces meaning.
- Raw source is not treated as identity merely because it is concrete.

## Common task profiles

**Explore:** context + object + relationships.

**Inspect:** object + evidence + provenance.

**Trace:** object + relation + source trail.

**Resolve:** object + evidence + finer representation.

**Verify:** claim/status + evidence + provenance + validation context.

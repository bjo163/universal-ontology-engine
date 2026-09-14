# Semantic Projection Graph Invariants v1

Bound to `oxdx.semantic-projection.rfc/v1` and issue #54.

## Edge domains

- `CONTAINS` -> structural ownership.
- `PROJECTS_TO` -> semantic projection.
- `OBSERVED_AT` -> runtime observation.
- `REPRESENTED_AS` -> representation.
- all other declared edges remain general semantic relations.

## Invariants

1. Structural parentage remains authoritative and is not rewritten by projection.
2. A `PROJECTS_TO`, `OBSERVED_AT`, or `REPRESENTED_AS` edge must not make its target a structural child of the source.
3. Semantic projection may skip canonical levels; no contiguous hierarchy is required.
4. Exact projection materialization keys `(source, target, rule_id, rule_version)` are idempotent.
5. Multiple rules may support the same source->target projection edge while retaining independent trace evidence.
6. Projection cycles are rejected deterministically.
7. Whole-graph validation returns ordered machine-readable violations so low-level graph construction can still be audited after the fact.
8. Existing Phase 1-6 graphs remain valid; the legacy `add_edge` API is not redefined by Phase 7.

Projection provenance/explainability remains owned by the semantic trace contract (#51); the graph only owns materialization and relationship invariants.

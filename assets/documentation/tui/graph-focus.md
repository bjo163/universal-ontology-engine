# OX-DX Graph Focus and Keyboard Exploration

Graph navigation is relation-aware.

## Focus center

A focused node is the visual anchor.

Neighbor ordering is deterministic in implementation, but this spec does not dictate a graph storage order.

## Actions

### Neighbor
Move to a visibly related node.

Use arrow keys according to rendered topology or a linear neighbor list.

### Expand
Reveal additional neighbors/edge groups.

### Collapse
Hide expanded neighborhood while keeping focus.

### Trace
Enter ordered trace of a selected edge/path.

### Path
Inspect current traversal history/path.

Do not call a traversal path a containment path unless every edge is `contains`.

### Structural parent

Use `parent` only when:
- Node.parent is present; or
- the relation is actual structural `contains` in the direction that supports parent/child wording.

### Child

Use only for actual structural containment.

### Other relations

Use:
- source;
- destination;
- neighbor;
- dependency;
- projection;
- representation;
- observation target.

## Keyboard proposal

- arrows: neighbor movement;
- Enter: inspect focused node;
- `t`: trace selected/focused relation;
- `e`: expand;
- `c`: collapse;
- Esc: return.

Optional vi aliases follow the global keymap.

## Dense graphs

Switch to grouped edge list rather than drawing unreadable topology.

Terminal clarity outranks visual graph literalism.

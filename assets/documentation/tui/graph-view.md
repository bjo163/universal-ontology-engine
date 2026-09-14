# OX-DX Terminal-Native Graph View

Do not reproduce a GUI node graph with ASCII dots.

Use spatial text topology.

## Core structure

```text
             references
        ┌ . . . . . . . .▷ SYMBOL
        │
◆ SOURCE ──contains──▷ UNIT ──contains──▷ MODULE
        │
        └──observed_at──▷ EVENT
```

ASCII fallback:

```text
             references
        + .. .. .. .. > SYMBOL
        |
* SOURCE --contains--> UNIT --contains--> MODULE
        |
        +--observed_at--> EVENT
```

This is a rendering grammar; only display actual edges.

## Focused node

Current node is the stable anchor.

Show:
- exact TYPE;
- name/id summary;
- selected/focused distinction;
- resolution level.

## Neighbor

Neighbors are grouped by actual edge kind, not by arbitrary visual direction.

## Direction

Direction follows edge `from → to`.

Do not reverse edge semantics because a layout looks better.

## Context

Always preserve:
- current node;
- current resolution;
- scope;
- path/history affordance.

## Edge kinds

Canonical engine edge classes:
- contains
- references
- specializes
- depends_on
- invokes
- produces
- consumes
- causes
- projects_to
- represented_as
- observed_at

## Parent/child wording

Use parent/child only for actual structural containment/Node parentage.

For other relations use:
- source/destination;
- neighbor;
- related node;
- projected view;
- representation;
- observation target.

## Dense mode

Collapse repeated edges by kind:

`contains → SOURCE ×12`

Only when exact count exists.

## Accessibility

Provide a linear relation list equivalent to spatial topology.

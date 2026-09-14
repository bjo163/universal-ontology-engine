# OX-DX Motion Language — Implementation Bridge

Brand Bible authority:

`../OX-DX-BRAND-BIBLE-v1.0.md`

Canonical visual grammar:

`../visual/motion-grammar.md`

## Axiom

**MOTION IS RESOLUTION.**

## Operational rule

**RESOLVE, DON'T DECORATE.**

## Canonical tokens

| Token | Duration | Use |
|---|---:|---|
| micro | 98 ms | compact response |
| witness | 196 ms | hover / focus / observation cue |
| resolve | 294 ms | relation / local resolution |
| field | 490 ms | major field transformation |

Resolution easing:

`cubic-bezier(0.21, 0.63, 0.28, 1)`

Canonical visual states:

`FIELD → RIFT → TRACE → PULSE → GRAIN`

These are visual states only, not engine phases or ontology levels.

## Required behavior

- preserve an anchor, axis, source, or break coordinate;
- avoid generic slide/fade as the primary identity behavior;
- avoid scale-from-zero, particles, ambient parallax, and decorative glitch loops;
- respect `prefers-reduced-motion: reduce`;
- never hide required information behind animation.

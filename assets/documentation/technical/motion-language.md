# OX-DX Motion Language

This technical document is the implementation bridge for motion.

The canonical visual-motion grammar is:

**`../visual/motion-grammar.md`**

## Core rule

**MOTION IS RESOLUTION.**

Motion should change continuity, persistence, density, provenance visibility, or witness state while preserving at least one spatial invariant.

## Canonical tokens

| Token | Duration | Use |
|---|---:|---|
| micro | 98 ms | compact response |
| witness | 196 ms | hover / focus / observation cue |
| resolve | 294 ms | relation / local resolution |
| field | 490 ms | major field transformation |

Resolution easing:

`cubic-bezier(0.21, 0.63, 0.28, 1)`

## Canonical visual states

`FIELD → RIFT → TRACE → PULSE → GRAIN`

These are visual states only, not engine phases or ontology levels.

## Required behavior

- preserve an anchor, axis, source, or break coordinate during transformation;
- avoid generic slide/fade as the primary motion language;
- avoid scale-from-zero, particles, ambient parallax, and decorative glitch loops;
- respect `prefers-reduced-motion: reduce`;
- never hide required information behind animation.

For detailed interaction behavior see `../visual/interaction-grammar.md`.

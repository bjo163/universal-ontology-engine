# OX-DX Graph Language

OX-DX avoids default dot-line-arrow diagrams.

## Node
Default node: cut-square/diamond witness with optional open frame fragment and label in a void slot.

## Relation — Persistence Rail
Use one or two parallel carriers. Direction is shown through increasing persistence, asymmetric endpoint cut, destination anchor alignment, or label placement after a central break. Arrowheads are reserved for cases where convention materially improves comprehension.

## Broken relation
Insufficient continuity uses a deliberate `2u` or `3u` void. Do not bridge unknown relation with a faint line.

## Relation tension
Related nodes may sit on unequal offsets; a rail resolves the displacement without forcing symmetry.

## Context
Context frames are open and offset. They do not imply ownership unless the technical diagram explicitly documents containment.

## Compact simplification
1. remove secondary labels;
2. remove trace tails;
3. collapse paired rails to one;
4. retain anchors + major relation.

Meaning must survive grayscale, high contrast, and no texture.

# OX-DX Graph UI

The graph is a primary product interaction model. It inherits the canonical OX-DX graph language instead of collapsing into dots, lines, and arrows.

## Node

Default product node:
- cut-square or diamond witness geometry where appropriate;
- open context frame instead of a closed card by default;
- concise label in a void slot;
- optional status marker that does not replace identity.

Dense technical layouts may use simpler rectangles when clarity improves.

## Edge

Default relation = **Persistence Rail**.

A relation should communicate type, source, destination, direction when relevant, and continuity or interruption where evidence is incomplete.

Arrowheads are allowed when convention improves comprehension, but they are not the entire graph language.

## Selection and focus

Selection adds witness geometry without changing identity.

Keyboard focus remains visibly different from selection.

## Context

The selected node should retain:
- local neighborhood;
- relation labels;
- parent or scope where relevant;
- a path back to evidence and provenance.

## Zoom vs resolution

Zoom changes visual scale.

Resolution changes descriptive detail.

Never conflate them silently.

## Filter

Filtering hides or deemphasizes visible subsets. It must never silently change graph semantics.

## Relation classes

When implementation data is available, preserve exact relation names such as:
`CONTAINS`, `PROJECTS_TO`, `REPRESENTED_AS`, `OBSERVED_AT`, `REFERENCES`, and `CAUSES`.

Do not invent decorative edge labels.

## Accessibility

Provide textual relation inspection, keyboard traversal, visible focus, and non-color-only relation/status cues. Essential information must not depend on hover.

# OX-DX Original Visual System Source

This directory contains the machine-readable token source for the Interrupted Evidence Field.

## Source files

- `original-visual-tokens.json` — implementation-friendly dimensions, colors, motion timings, and responsive boundaries.
- `ox-dx-master-geometry.svg` — approved pre-existing master geometry; unchanged by this system.

## Reproduction model

The new supporting SVG assets are deliberately authored from:
- integer 7 / 21 / 49 field geometry;
- 3 px optical offsets;
- 1 / 3 / 7 px stroke hierarchy;
- square/diamond witness anchors;
- measured interruption coordinates;
- deterministic path dropout.

No random-number generation, procedural noise filter, external font file, raster embedding, or gradient is required for the core primitives.

The SVG files themselves are editable canonical supporting sources.

## Semantic boundary

This source system defines graphics only. It does not define ontology TYPEs, KINDs, graph edges, engine phases, or implementation behavior.

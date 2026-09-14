# OX-DX TUI Status System

TUI status combines product status vocabulary with terminal-safe labels/glyphs.

This does not create engine enums.

## Semantic states

Potential future UI states:
- OBSERVED
- VERIFIED
- DERIVED
- PROJECTED
- UNVERIFIED
- UNKNOWN
- EXPERIMENTAL
- GATED
- PLANNED

Use only when supported by underlying context.

## Operational states

Processes:
- DISCOVERING
- INSPECTING
- PARSING
- MAPPING
- TRACING
- RESOLVING
- VERIFYING

These describe operations, not evidence certainty.

## Diagnostics

Separate:
- INFO
- NOTICE
- WARNING
- ERROR
- FATAL

## Rendering rule

Every semantic state uses:
- text label;
- glyph/structure when helpful;
- optional color.

## Examples

`◇ OBSERVED`

`✓ VERIFIED`

`⋯ UNKNOWN`

ASCII:

`o OBSERVED`

`OK VERIFIED`

`... UNKNOWN`

## Avoid

- one colored dot as status;
- `UNKNOWN` in error red;
- `VERIFIED` without scope/check;
- `OBSERVED` for inferred/projected results.

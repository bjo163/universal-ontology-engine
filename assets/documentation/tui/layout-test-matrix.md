# OX-DX Terminal Layout Test Matrix

| Size | Remains | Collapses | Hides | Scrollable |
|---|---|---|---|---|
| 80×24 | context, object, current level, primary evidence, status, command | provenance → summary; resolution → ±2 | graph secondary, metadata extras | primary/source |
| 100×30 | above + relation summary + compact provenance | graph → compact list | secondary diagnostics unless active | source/relations |
| 120×40 | evidence + provenance rail + current-zone spine | wide graph → medium neighborhood | tertiary metadata | source/graph |
| 160×50 | graph + evidence + provenance/trace + diagnostics summary | full level map on demand | very low-priority help | each dense region |
| 200×60 | full-depth context, comparison, extended graph/source | little; keep one dominant surface | decorative extras | large source/graph |

## Test rules

At every size verify:
- focus visible;
- selected state visible;
- current resolution visible;
- command/status accessible;
- no horizontal clipping of critical labels;
- exact edge kind available;
- source evidence recoverable;
- unknown ≠ error;
- ASCII fallback usable.

## 80×24

No more than:
- one dominant surface;
- one compact context/status area;
- one command line.

## 200×60

Do not fill space for the sake of density.

Use extra columns to expose relationships/provenance, not larger banners.

# OX-DX TUI Color Semantics

Color reinforces labels/glyphs.

It never creates technical meaning by itself.

| Role | Preferred truecolor | Low-color fallback | Non-color carrier |
|---|---|---|---|
| BASE | terminal/ Void | default | layout |
| STRUCTURE | White | bright white/default | line/glyph |
| FOCUS | Cyan | bright cyan / inverse | `>` focus marker |
| OBSERVATION | Cyan | cyan | `◇ OBSERVATION` |
| SIGNAL | Acid | green | cadence/glyph + label |
| WARNING | conventional yellow | yellow | `! WARNING` |
| ERROR | conventional red | red | `X ERROR` |
| UNKNOWN | Muted | gray/default | gap + UNKNOWN label |
| VERIFIED | Acid reinforcement | green | `✓/OK VERIFIED` |

Warning/error conventional terminal colors are accessibility/expectation fallbacks, not additions to the canonical brand palette.

## Focus vs observation

Both may use cyan in capable terminals.

They remain distinguishable because:
- focus uses input/focus marker;
- observation uses witness marker + OBSERVATION label.

## Verified vs signal

Both may use Acid/green reinforcement.

Verified requires explicit `VERIFIED` and named scope/check.

## Monochrome

All roles remain distinct using:
- symbol;
- label;
- spacing;
- border/persistence;
- inversion/bold.

## User theme

Prefer terminal default background where forced dark backgrounds reduce readability or user control.

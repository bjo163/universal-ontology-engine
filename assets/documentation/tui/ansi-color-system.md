# OX-DX ANSI Color System

Color is reinforcement.

The TUI must work in monochrome.

## Canonical palette

- Void `#07090A`
- Charcoal `#111418`
- Graphite `#20252B`
- White `#F5F7F8`
- Muted Steel `#99A2AA`
- Electric Cyan `#00D8FF`
- Acid `#8CFF00`
- Ultraviolet `#9C4DFF`
- Black `#000000`

## Truecolor

Use exact canonical values where contrast remains accessible.

## 256-color approximation

Recommended:
- Void → 232/233
- Charcoal → 234
- Graphite → 235/236
- White → 255
- Muted → 247
- Cyan → 45
- Acid → 118
- Ultraviolet → 99 or 135
- Black → 16

Exact terminal rendering varies.

## ANSI 16-color

Recommended semantic mapping:
- structure/text → bright white / default foreground
- observation/focus → bright cyan
- signal/verified reinforcement → bright green
- projection/context → magenta
- muted → bright black/gray
- warning → yellow
- error → red only as conventional reinforcement

Warning/error mappings are terminal convention fallbacks, not new OX-DX brand palette tokens.

## ANSI 8-color

Use:
- default foreground/background first;
- cyan for observation/focus where available;
- green for verified reinforcement;
- yellow warning;
- red error;
- magenta projection.

Never depend on these hues alone.

## Monochrome

Use:
- glyph;
- case/label;
- spacing;
- inversion;
- underline/bold selectively;
- open/broken boundary.

## High contrast

- prefer terminal default black/white or user theme;
- disable muted-only distinctions;
- avoid dim attribute for essential information;
- do not force background colors that fight user settings.

## Detection

Implementation may respect:
- terminal color capability;
- `NO_COLOR`;
- explicit TUI mode/flag;
- high-contrast configuration.

Do not infer semantic capability from color support.

# OX-DX TUI Border System

Borders communicate context and focus.

They are not decorative boxes.

## Primary boundary

Unicode:
- horizontal `─`
- vertical `│`
- intentional opening instead of full corners where possible.

ASCII:
- `-`
- `|`

Use for major inspection scope only.

## Secondary boundary

Unicode:
- `┄` or sparse `─` segments where terminal support is known.

ASCII:
- `.` / `-` segments.

Use lightly.

## Focus boundary

Unicode pattern:

```text
◆─ target ─────
│
```

ASCII:

```text
*> target -----
|
```

Focus marker is stronger than surrounding boundary.

## Observation boundary

Unicode:
- offset `◇` witness + partial `┆`/rail.

ASCII:
- `o` + `|`/colon rail.

Target geometry remains unchanged.

## Warning boundary

Unicode:
- `!` label + local interrupted boundary.

ASCII:
- `!`

No screen-wide colored box.

## Unknown boundary

Unicode:
- open edge + `?` is **not** default.
- preferred marker: `⋯` or explicit `UNKNOWN` label plus gap.

ASCII:
- `...` + `UNKNOWN`.

## Disabled boundary

Use muted/static boundary and explicit `DISABLED` or unavailable action text when needed.

## Construction rules

- leave at least one deliberate opening in OX-DX-native frames;
- do not box every panel;
- focus marker outranks border thickness;
- meaning survives ASCII;
- terminals without reliable box drawing fall back cleanly.

# OX-DX Terminal Framing Language

OX-DX framing is an interrupted evidence field adapted to character cells.

## Signature

A recognizable OX-DX terminal surface normally includes at least three:
- one stable anchor;
- one broken/partial boundary;
- one deliberate empty lane;
- one persistence change;
- one offset witness/trace.

## Primary field

Do not wrap every region in a complete rectangle.

Preferred:

```text
◆ EVIDENCE ─────────────
│
│  source ...
│
     ┄ provenance ...
```

ASCII:

```text
* EVIDENCE -------------
|
|  source ...
|
     .. provenance ...
```

## Measurement marks

Use compact level/span/count labels where real:
- `L22`
- `41:7–53:2`
- `12 edges`

Never generate decorative coordinates.

## Controlled asymmetry

A main surface may begin at column 1 while a provenance rail starts later.

Do not force symmetric split panes.

## Empty space

A blank lane may mean:
- boundary;
- context separation;
- unknown;
- visual pause.

Do not fill it with ASCII texture.

## Terminal safety

- no double-width decorative glyph dependency;
- no combining-character art;
- no terminal-specific ligatures;
- all framing has ASCII fallback.

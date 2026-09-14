# OX-DX TUI Source View

Source location is provenance.

It is not identity.

## Supported source fields

Show only when supplied:
- file;
- path;
- line;
- column;
- span/range;
- native syntax kind;
- language;
- source excerpt.

## Layout

```text
SOURCE  crates/ontology-core/src/lib.rs
span    41:1–53:2     syntax enum

 41 │ pub enum EdgeKind {
 42 │     Contains,
 43 │     References,
    │     ^^^^^^^^^^
```

ASCII fallback:

```text
SOURCE  crates/ontology-core/src/lib.rs
span    41:1-53:2     syntax enum

 41 | pub enum EdgeKind {
 42 |     Contains,
 43 |     References,
    |     ^^^^^^^^^^
```

## Highlight

Use:
- underline/marker;
- inversion where necessary;
- restrained observation color.

Do not colorize an entire source block solely for brand effect.

## Native syntax

If syntax highlighting exists later, it is secondary to:
- exact source text;
- selected span;
- evidence marker.

## Missing fields

Do not print fake:
- column 0;
- unknown timestamp placeholders;
- generated line numbers that were not part of source evidence.

Use explicit `not available` only when that distinction helps.

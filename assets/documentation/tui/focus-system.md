# OX-DX TUI Focus System

Focus must be obvious without turning the terminal into neon.

## Focus priority

Use in order:

1. focus marker;
2. border/rail emphasis;
3. weight/inversion;
4. restrained color.

Color is optional.

## Unicode example

```text
› ◆ REPOSITORY  universal-ontology-engine
  ┄ source Cargo.toml
```

ASCII:

```text
> * REPOSITORY  universal-ontology-engine
  .. source Cargo.toml
```

## Focused panel

Do not fully recolor the panel.

Strengthen:
- one boundary segment;
- section label;
- focus marker.

## Keyboard

Arrow/Tab navigation must provide visible focus.

Vi-style aliases may supplement.

## High contrast

Use:
- inversion or bold;
- `>` marker;
- explicit label.

## Focus vs semantic state

Focus never means:
- observed;
- selected;
- verified;
- error.

The data's semantic status remains separate.

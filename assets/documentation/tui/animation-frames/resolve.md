# TUI Animation Frames — RESOLVE

Use 3–4 frames only.

## Frame 0

```text
RESOLVING

SOURCE ┄  ┄  ┄ RELATION
```

ASCII:

```text
RESOLVING

SOURCE .  .  . RELATION
```

## Frame 1

```text
RESOLVING

SOURCE ┄────  ┄ RELATION
```

## Frame 2

```text
RESOLVING

SOURCE ───────▷ RELATION
             ◆
```

## Final

```text
RESOLVED

SOURCE ───────▷ RELATION
             ◆ supported detail
```

Only show `RESOLVED` if product semantics support that state.

Unsupported gaps remain open.

Reduced motion: render final state directly.

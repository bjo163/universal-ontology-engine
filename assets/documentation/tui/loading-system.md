# OX-DX TUI Loading / Process System

Process labels are only used when they describe real work.

Canonical vocabulary:

- DISCOVERING
- INSPECTING
- PARSING
- MAPPING
- TRACING
- RESOLVING
- VERIFYING

## No fake percentages

Only show percentage when the operation provides a meaningful measurable total.

## Visual process cue

Preferred:

```text
TRACING  · ·· ·
```

or:

```text
TRACING  source → relation
```

Cadence may update using a small 3-state local sequence.

Do not run a noisy infinite spinner.

## Static fallback

`TRACING…`

ASCII:

`TRACING...`

## Preserve context

Keep:
- target;
- prior evidence;
- current scope.

Mark only the updating region pending.

## Cancellation

If supported, show actual cancel key/action.

Do not display `Esc cancel` unless the implementation supports it.

## Reduced motion

Static process label only.

## Sound

Silent by default.

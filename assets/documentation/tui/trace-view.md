# OX-DX TUI Trace View

Trace follows evidence or relation.

Trace is not ownership.

## Trace anatomy

Show:
- source;
- relation;
- observation when relevant;
- path;
- resolution;
- provenance.

## Example

```text
TRACE · L28 ELEMENT

◆ function parse
┄ observed_at ▷ observation:syntax
┄ source      ▷ crates/.../lib.rs:120
┄ projects_to ▷ L34 FUNCTION
```

Only show edges actually present.

## Ownership distinction

Containment:

`A ─contains─▷ B`

Trace/provenance:

`A ┄ source ▷ B`

Observation:

`A ┄ observed_at ▷ B`

Do not render these identically.

## Path

For multi-hop trace:

```text
01 ◆ SOURCE
02 ┄ contains     ▷ UNIT
03 ┄ contains     ▷ MODULE
04 ┄ observed_at  ▷ EVENT
```

The path is an ordered inspection path, not automatically a containment path.

## Resolution

Show level changes as annotations.

Do not imply a level transition is an edge unless there is an actual edge.

## Compact mode

One hop at a time + history count.

## Sound

Default silent.

Optional minimal TRACE sound only for explicit user-triggered trace action.

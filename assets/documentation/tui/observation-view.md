# OX-DX TUI Observation View

**OBSERVATION IS NOT OWNERSHIP.**

Runtime/syntax/other observations remain separate from structure.

## Composition

```text
STRUCTURE
◆ ELEMENT  parse

        ◇ OBSERVATION
        ┄ kind       syntax
        ┄ relation   observed_at
        ┄ source     <actual source>
```

ASCII:

```text
STRUCTURE
* ELEMENT  parse

        o OBSERVATION
        . kind       syntax
        . relation   observed_at
        . source     <actual source>
```

## Rule

Do not indent observation as a structural child merely because it is visually nearby.

Use an offset witness/rail.

## Runtime

If an EXECUTION is observed at an ELEMENT:

`EXECUTION ─observed_at─▷ ELEMENT`

Render according to actual edge direction supplied by the graph.

Do not rewrite semantics for visual preference.

## Observation fields

Only when supported:
- observation kind;
- target;
- timestamp;
- source/collector;
- event/state data;
- provenance;
- verification state.

## Color

Cyan may reinforce observation.

The `OBSERVATION` label and witness marker remain mandatory in non-color mode.

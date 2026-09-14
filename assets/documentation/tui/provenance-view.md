# OX-DX TUI Provenance View

**PROVENANCE IS NOT IDENTITY.**

Show where/how evidence was observed without replacing semantic identity.

## Fields

When available:
- source/path/URI;
- revision;
- line/column/span;
- adapter/observer;
- timestamp;
- context;
- warnings affecting interpretation.

## Provenance Rail

Unicode:

```text
◇ provenance
┄ source   crates/ontology-core/src/lib.rs
┄ span     41:1–53:2
┄ observer deterministic_syntax_observation
```

ASCII:

```text
o provenance
. source   crates/ontology-core/src/lib.rs
. span     41:1-53:2
. observer deterministic_syntax_observation
```

The primary object remains outside this rail.

## Multiple sources

Render separate source groups.

Do not merge them into a single “origin” if traceability would be lost.

## Time

Only show time when actual observation/runtime data provides it.

No decorative timestamps.

## Compact mode

Show one provenance summary line plus `inspect provenance` action.

## Unknown

Unknown provenance is not unknown identity.

Label the missing field specifically.

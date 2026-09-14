# OX-DX TUI Error System

Errors answer:

1. what happened?
2. where?
3. why, if known?
4. what evidence remains?
5. what can the user do next?

## Canonical pattern

```text
ERROR · MALFORMED INPUT

what     malformed input observed
where    crates/example/src/lib.rs
span     42:7–42:19
why      <actual parser/validation detail>

evidence preserved
  42 | ...

next     inspect source · adjust input · retry
```

Do not invent a reason if it is unknown.

## Copy

Prefer:

`Malformed input observed.`

`Registry could not be loaded.`

`Source is unavailable in the current scope.`

Avoid:

`Something went wrong.`

## Scope

Local errors remain local.

Do not clear the whole screen for a row-level or panel-level error.

## Evidence

Never hide malformed input by replacing it with the error message.

## Unknown vs error

Unknown is not error.

Unverified is not error.

Unmaterialized is not error.

## Fatal

Fatal state must:
- preserve diagnostic text;
- show affected scope;
- allow safe quit/copy where possible.

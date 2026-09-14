# OX-DX TUI Prompt Language

The TUI prompt is a compact inspection coordinate.

It must communicate:
- scope;
- current resolution;
- current context;
- mode/state.

It should not mimic Bash, Zsh, Fish, Starship, or a path-only shell prompt.

## Canonical composition

Preferred wide form:

```text
OX·<scope>  L<nn>:<TYPE>  <mode>  <state>
›
```

Example:

```text
OX·repo  L22:REPOSITORY  INSPECT  OBSERVED
›
```

ASCII fallback:

```text
OX:repo  L22:REPOSITORY  INSPECT  OBSERVED
>
```

## Compact form

```text
OX·L22·INSPECT ›
```

ASCII:

`OX:L22:INSPECT >`

## Scope

Scope is semantic/user context, not necessarily a filesystem path.

Examples:
- `repo`
- `source`
- `node:<short-id>`

Do not show a path as identity unless the user is explicitly inspecting source location.

## Resolution

Use:
- `L01` … `L49`;
- exact canonical TYPE where space allows.

## Mode

Future TUI modes:
- DISCOVER
- INSPECT
- GRAPH
- TRACE
- OBSERVE
- RESOLVE
- VERIFY
- DIAGNOSTIC

These are interaction modes, not claims about current CLI implementation.

## State

Show only meaningful supported state:
- OBSERVED
- UNKNOWN
- VERIFIED
- ERROR
- etc.

No status is required for every prompt.

## Input marker

Unicode: `›`

ASCII: `>`

Input marker indicates where the user is, not data semantics.

## Rules

- no current working directory by default;
- no git branch decoration by default;
- no clock/battery/host clutter;
- no emoji;
- no powerline dependency;
- no Nerd Font requirement.

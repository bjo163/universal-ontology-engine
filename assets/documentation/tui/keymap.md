# OX-DX TUI Keymap

No existing TUI keymap is present in the repository at this audit point.

This is future design guidance.

## Global

| Action | Primary | Optional |
|---|---|---|
| help | `?` | `F1` |
| quit | `q` | — |
| close/return | `Esc` | `Backspace` where safe |
| command surface | `:` or `/` only if implementation distinguishes command/search | — |

Do not bind both command and search to the same key without a clear mode.

## Navigation

| Action | Primary | Optional vi |
|---|---|---|
| previous | Up | `k` |
| next | Down | `j` |
| broader/left | Left | `h` |
| finer/right | Right | `l` |
| inspect/activate | Enter | — |
| next focus region | Tab | — |
| previous focus region | Shift+Tab | — |

Arrow keys are mandatory.

## Selection

- Space: toggle/select where multi-selection is supported.
- Enter: inspect/activate current focus.
- Selection semantics depend on component.

## Resolution

- `[`: broader resolution shortcut.
- `]`: finer resolution shortcut.
- Home/End: first/last visible item, not automatically level 01/49 unless focus is on full level map.

## Graph

Suggested:
- arrow keys: neighbor traversal according to rendered topology/list;
- `t`: trace focused edge/node;
- `e`: expand;
- `c`: collapse;
- Enter: inspect.

Do not use `p` = parent unless focused relation is actual containment.

## Trace

- Up/Down: previous/next hop;
- Enter: inspect hop;
- `t`: return/toggle trace context if implemented.

## Expand/collapse

- Right / `e`: expand when collapsible.
- Left / `c`: collapse when collapsible.

These do not override resolution navigation unless component focus makes the action unambiguous.

## Conflict rule

Key behavior is scoped by focused component and visible help.

Never overload a key with two invisible meanings.

## Accessibility

Every core action must have non-vi primary access.

# OX-DX TUI Mouse Support

Mouse is optional.

Keyboard remains complete.

## Allowed supplements

### Click
- focus a visible row/node/action;
- select/inspect where click semantics are obvious.

### Scroll
- scroll focused/hovered scrollable region;
- never change ontology resolution silently.

### Drag
Avoid by default.

Only use if a future feature genuinely benefits, such as resizing an explicit split.

Do not drag graph nodes merely to imitate GUI graph tools.

## Hover

May preview context if terminal mouse reporting supports it.

Essential information must remain available without hover.

## Selection

Click selection must produce the same state as keyboard selection.

## Terminal compatibility

Mouse can be disabled:
- by terminal capability;
- user preference;
- remote/limited environments.

The TUI remains fully usable.

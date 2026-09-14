# OX-DX TUI Help System

Help is contextual, compact, and keyboard-first.

## Default help strip

Show only relevant actions:

```text
↑↓ move   Enter inspect   [ ] resolution   ? help   q quit
```

ASCII-safe.

## Context help

For Graph mode:

```text
GRAPH
↑↓/←→ neighbor     Enter inspect
t trace            e expand
c collapse         / filter
? all help         q quit
```

Only advertise implemented bindings.

## Full help

Full help is a scrollable surface grouped by:
- navigation;
- focus/selection;
- resolution;
- graph;
- trace;
- command;
- global.

## Discoverability

Arrow keys are always documented.

Optional vi-style aliases appear secondary.

## Close

Esc closes help if not needed for a higher-priority cancellation state.

## No documentation dump

Long conceptual docs remain outside TUI help.

Help tells the user:
- what they can do here;
- what key does it;
- what will happen.

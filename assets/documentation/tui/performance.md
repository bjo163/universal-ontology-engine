# OX-DX TUI Performance

Target:

**lightweight · responsive · stable**

## Render budget

Prefer event-driven redraw.

Do not redraw the full terminal continuously when nothing changed.

Suggested design target:
- idle: 0 continuous animation redraws;
- routine input: redraw only affected regions/full frame as Ratatui architecture requires, but avoid timers with no state change;
- animations: short bounded sequences.

## Update budget

Batch high-frequency data/observation updates when necessary.

Do not let background observations starve keyboard input.

## Animation budget

- 2–5 meaningful frames;
- intensity 0–2 in routine UI;
- disable on reduced-motion/slow-terminal mode;
- no infinite spinner loop as identity.

## Memory sensitivity

Avoid retaining:
- unbounded command history;
- unbounded diagnostic logs;
- full graph copies solely for presentation;
- huge source buffers without paging/streaming strategy.

Exact implementation policy belongs to future engineering.

## Resize

Debounce/coalesce rapid resize events if needed.

## Audio

Default silent; no performance dependency.

## Principle

When a sensory effect competes with input responsiveness, remove the effect.

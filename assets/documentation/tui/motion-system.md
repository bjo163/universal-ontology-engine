# OX-DX TUI Motion System

Terminal motion is local and low-overhead.

## Allowed

- cursor/focus movement;
- line reveal;
- small local state change;
- short persistence transition;
- bounded process cadence;
- verification settle.

## Avoid

- full screen clear/redraw effects as animation;
- rapid flashing;
- fake hacker typing;
- matrix rain;
- scan line across screen;
- constant spinner activity;
- animated background noise.

## Timing

Reuse sensory durations conceptually:
- 98 ms micro;
- 196 ms witness;
- 294 ms resolve;
- 490 ms field.

Terminal frame rates do not need to match millisecond values exactly if doing so causes unnecessary redraws.

Prefer 2–5 meaningful frames.

## Reduced motion

Render final state directly.

## Remote/slow terminals

Disable animation automatically or by preference when updates are costly.

## Invariant

During resolution animation preserve at least one:
- focus anchor;
- row;
- source line;
- relation endpoint;
- current level.

Motion must not make inspection harder.

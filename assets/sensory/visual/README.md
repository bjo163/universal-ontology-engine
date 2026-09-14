# OX-DX Reusable Visual SFX References v1.0

These SVGs are editable reference/state diagrams for future product/video implementations.

They are not finished UI components and do not define engine states.

## Files

- `trace/ox-dx-vfx-trace-v1.svg`
- `scan/ox-dx-vfx-scan-v1.svg`
- `reveal/ox-dx-vfx-reveal-v1.svg`
- `resolve/ox-dx-vfx-resolve-v1.svg`
- `observe/ox-dx-vfx-observe-v1.svg`
- `signal/ox-dx-vfx-signal-v1.svg`
- `distort/ox-dx-vfx-distort-v1.svg`
- `verify/ox-dx-vfx-verify-v1.svg`
- `transition/ox-dx-vfx-transition-v1.svg`

## Construction

All references use:
- canonical OX-DX palette;
- hard-edged geometry;
- diamond witness anchors;
- persistence/break logic;
- no gradient;
- no SVG filter/noise dependency;
- no embedded raster;
- no generic glow.

Several references show multiple states on one strip.

Those states are implementation storyboards, not simultaneous UI.

## Implementation

Future developers may reproduce the behaviors using:
- SVG stroke geometry;
- CSS;
- Web Animations API;
- framework animation primitives;
- native rendering;

provided the semantic behavior and accessibility fallback remain intact.

## Naming

`ox-dx-vfx-{event}-v{version}.svg`

Do not use `final`, `latest`, `new`, or `test2`.

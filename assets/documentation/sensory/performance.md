# OX-DX Sensory Performance Rules

Sensory design should be:

**fast · quiet · predictable**

## Effect budget

Routine UI:
- at most one primary sensory effect per interaction;
- intensity Level 0–2;
- local scope;
- no full-screen effect for local changes.

Major transitions:
- intensity up to Level 3 when useful.

Level 4–5:
- non-routine brand moments only.

## Animation budget

Prefer:
- transform/opacity only when semantically appropriate;
- SVG/CSS geometry changes;
- bounded regions;
- canonical durations.

Avoid:
- constant CPU animation;
- continuous canvas redraw;
- background particle systems;
- unnecessary WebGL;
- large blur/filter stacks;
- infinite glitch loops.

## Audio budget

Routine product:
- no ambient download required;
- short one-shots only when meaningful;
- duplicate/repeated events suppressed;
- UI must work fully with audio unavailable.

Do not preload a large SFX library merely for branding.

## Video budget

Do not use video for effects that can be expressed as reusable SVG/CSS geometry.

Reserve video for:
- authored brand film;
- social/release media;
- presentation;
- installation.

## Low-power fallback

Every effect defines:
- **PRIMARY** — full intended effect;
- **FALLBACK** — simplified SVG/CSS/text;
- **LOW-POWER** — static/short local geometry;
- **SILENT** — no audio.

## Failure behavior

If sensory assets fail to load:
- product meaning remains intact;
- controls remain usable;
- text/status remain correct;
- no blocked interaction.

Sensory assets are enhancement, not a runtime dependency for correctness.

# OX-DX Motion Language

## Core principle

**RESOLVE, DON'T DECORATE.**

Motion exists to explain changing resolution, relation, observation, or state. It should never become ambient spectacle.

## Motion sequence

`FRAGMENT → STRUCTURE → RELATION → OBSERVATION → RESOLUTION`

The sequence is conceptual, not a mandatory technical pipeline.

## Timing tokens

| Token | Duration | Use |
|---|---:|---|
| snap | 120 ms | tiny state confirmation / glyph response |
| quick | 180 ms | hover, focus, compact reveal |
| resolve | 280 ms | structural transition / relation appearance |
| deep | 420 ms | hero or section-level resolution reveal |

Recommended easing: `cubic-bezier(0.22, 1, 0.36, 1)` for resolution; use linear only for signal scanning where constant speed carries meaning.

## Entry

Fragments or low-opacity lines appear first, then align into the stable frame. The observation node appears last. Avoid scale-from-zero gimmicks.

## Exit

Reverse emphasis rather than explode the system: node fades, relations thin, frame releases. Keep exit shorter than entry.

## Hover

Use one controlled response: line-weight shift, 1–2 px translation, or accent-node activation. Do not combine all three.

## Scroll

Reveal resolution in discrete stages tied to content. No perpetual parallax and no decorative particle field.

## Loading

Prefer a bounded scan or staged frame resolution. The animation must communicate waiting without implying fake progress.

## Transition

Preserve spatial continuity. Relations should connect before a new detail layer becomes dominant.

## Reveal

Use masks / clipping or opacity to expose ordered geometry from noisy surface, reinforcing **ORDER INSIDE CHAOS**.

## Reduced motion

Respect `prefers-reduced-motion: reduce`. Replace motion sequences with immediate state changes or a single short opacity transition. No required information may exist only in animation.

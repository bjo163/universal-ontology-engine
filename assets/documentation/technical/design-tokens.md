# OX-DX Design Tokens

These tokens implement the **Interrupted Evidence Field** supporting visual system. They do not alter the approved master logo geometry.

## Spatial system

| Token | Value | Purpose |
|---|---:|---|
| `--ox-micro` | `3px` | optical correction / trace offset |
| `--ox-u` | `7px` | base construction unit |
| `--ox-2u` | `14px` | witness node / relation break |
| `--ox-3u` | `21px` | observation offset / meso gap |
| `--ox-5u` | `35px` | medium field separation |
| `--ox-field` | `49px` | seven-unit macro module |
| `--ox-field-2` | `98px` | double field |

Preferred spacing: `3, 7, 14, 21, 35, 49, 98px`.

The older 8 px supporting assets remain valid historical exports. New Interrupted Evidence Field assets use the 7 / 21 / 49 system. Do not redraw the master logo to fit these tokens.

## Stroke

| Token | Value |
|---|---:|
| `--ox-stroke-hair` | `1px` |
| `--ox-stroke-relation` | `3px` |
| `--ox-stroke-structure` | `7px` |

Hairlines are never the sole carrier of critical information.

## Geometry

| Token | Value |
|---|---|
| `--ox-corner` | `0px` |
| `--ox-cut` | `7px` diagonal functional cut |
| `--ox-anchor-sm` | `7px` |
| `--ox-anchor-md` | `14px` |
| `--ox-anchor-lg` | `21px` |
| `--ox-break-local` | `7px` |
| `--ox-break-relation` | `14px` |
| `--ox-break-unresolved` | `21px+` |

Default nodes are diamond/cut-square, not circles.

## Color

| Token | Value |
|---|---|
| `--ox-void` | `#07090A` |
| `--ox-charcoal` | `#111418` |
| `--ox-graphite` | `#20252B` |
| `--ox-white` | `#F5F7F8` |
| `--ox-muted` | `#99A2AA` |
| `--ox-cyan` | `#00D8FF` |
| `--ox-acid` | `#8CFF00` |
| `--ox-ultraviolet` | `#9C4DFF` |

Color roles:
- structure = black/white;
- observation = cyan + witness geometry;
- signal = acid + cadence;
- projection = ultraviolet + displaced geometry;
- uncertainty = structural break first, muted annotation second.

Monochrome is mandatory.

## Typography

Use existing accessible system stacks from `typography.md`.

- identity wordmark = canonical vector only;
- headline = system sans, 650–800 where supported;
- body = system sans, 400–500;
- evidence / marginal notation = system mono;
- technical labels = uppercase mono with controlled tracking.

Typography originality comes from field placement and interruption, not proprietary fonts.

## Surface

| Token | Value |
|---|---|
| `--ox-surface-dark` | `#07090A` |
| `--ox-surface-raised` | `#111418` |
| `--ox-surface-light` | `#F5F7F8` |
| `--ox-disturbance-0` | none |
| `--ox-disturbance-1` | trace-level |
| `--ox-disturbance-2` | editorial |
| `--ox-disturbance-3` | poster maximum |

Disturbance is deterministic dropout/erosion, not random full-canvas noise.

## Motion

| Token | Duration | Use |
|---|---:|---|
| `--ox-motion-micro` | `98ms` | compact response |
| `--ox-motion-witness` | `196ms` | focus / hover / witness |
| `--ox-motion-resolve` | `294ms` | local relation / resolution |
| `--ox-motion-field` | `490ms` | major field transition |

Resolution easing: `cubic-bezier(0.21, 0.63, 0.28, 1)`.

Reduced motion renders the final state directly.

## Sensory extension v1.0

The sensory system extends existing motion/visual tokens. It does not create a second timing scale.

### Motion duration aliases

| Sensory role | Canonical token |
|---|---|
| micro | `--ox-motion-micro` = 98ms |
| witness | `--ox-motion-witness` = 196ms |
| resolve | `--ox-motion-resolve` = 294ms |
| field | `--ox-motion-field` = 490ms |

### Transition easing

Canonical sensory transition easing remains:

`cubic-bezier(0.21, 0.63, 0.28, 1)`

### Visual / motion intensity

| Level | Name | Meaning |
|---:|---|---|
| 0 | NONE | static / silent |
| 1 | MICRO | routine focus/selection |
| 2 | SUBTLE | standard inspect/trace/observe |
| 3 | NOTICEABLE | resolve/verify/context shift |
| 4 | MAJOR | major product reveal/change |
| 5 | CINEMATIC | brand film/launch/installation |

Most product interactions use levels 0–2.

### Sound intensity

Sound uses the same 0–5 semantic intensity vocabulary.

These numbers are **not gain values** and do not define decibels.

Loudness is governed by the audio production specification, not by UI semantic intensity.

### Audio event priority

| Priority | Meaning |
|---|---|
| BACKGROUND | normally silent |
| CONTEXT | optional subtle cue |
| INTERACTION | user-triggered cue may sound |
| CRITICAL | attention required; still not automatically louder |

Priority controls whether sound is warranted, not how “important” the product claims the event is.

### Silence

`silent` is a canonical sensory state.

Unknown, loading, focus, passive updates, and repeated routine events may intentionally produce no sound.

### Accessibility

Every sensory token must have:
- reduced-motion behavior;
- silent behavior;
- high-contrast behavior;
- text/geometry alternative when sound/color is absent.

Machine-readable sensory aliases live in:

`assets/design-tokens/sensory.json`

## Breakpoints

| Token | Width |
|---|---:|
| `--ox-bp-compact` | `420px` |
| `--ox-bp-medium` | `840px` |
| `--ox-bp-wide` | `1260px` |
| `--ox-bp-ultra` | `1680px` |

Responsive behavior changes context/detail, not just scale.

## Implementation rules

1. Preserve the master logo.
2. Use at least one deliberate void in brand compositions.
3. Meaning must survive without accent color.
4. Do not invent fake technical labels.
5. Visual grammar terms are not ontology semantics.
6. Prefer SVG for geometric primitives.
7. Sensory effects communicate behavior; they do not create technical semantics.
8. Sound remains optional for comprehension.

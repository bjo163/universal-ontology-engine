# OX-DX SFX Library — SPEC_ONLY v1.0

No audio files are committed by this task. This document defines sound-production specifications.

## Shared rules

- Use the canonical sonic DNA: Grain, Breath/Noise, Rail, Witness, Settle.
- Reuse canonical duration families: 98 / 196 / 294 / 490 ms.
- Most routine events remain quiet or silent.
- Spatialization supplements meaning; it never carries meaning alone.
- Avoid digital beeps, lasers, notification chimes, cinematic risers, and trailer impacts.

| Event | Purpose | Sonic character | Target duration | Intensity | Frequency character | Spatial behavior | Usage |
|---|---|---|---:|---|---|---|---|
| DISCOVER | new evidence/source becomes available | Grain + short Breath, then restrained Rail | 294 ms | 2 | low-mid with light high detail | centered | meaningful discovery only |
| INSPECT | enter focused inspection | Witness transient + short dry tail | 196 ms | 1–2 | mid-focused, dry | centered | explicit inspect action |
| OBSERVE | observation begins/becomes visible | softer Witness + Breath | 196 ms | 1 | mid + soft noise | centered | non-invasive observation |
| TRACE | relationship/provenance followed | Grain → Breath → faint Rail | 294 ms | 2 | textured mid/low-mid | subtle directional movement optional | explicit trace action |
| CONNECT | supported relation appears/commits | paired Rail components settle together | 294 ms | 2 | low-mid dyad, minimal high | source-to-destination optional | relation creation/reveal |
| RESOLVE | detail/relation becomes clearer | unstable texture reduces into stable Rail | 294–490 ms | 2–3 | noise-to-resonance | centered | resolution transition |
| VERIFY | named verification succeeds | short Settle + tiny Witness confirmation | 196–294 ms | 2 | stable mid/low-mid | centered | successful verification only |
| UNKNOWN | unresolved state | normally **silent** | 0 ms | 0 | — | — | default unresolved state |
| WARNING | user attention required but not failure | restrained interrupted Witness | 196 ms | 2 | controlled mid, no alarm tone | centered | actionable warning |
| ERROR | operation failed | local broken transient + short low resonance | 196–294 ms | 2–3 | lower weight, dry | centered | meaningful failure |
| SUCCESS | non-verification successful completion | optional quiet Settle or silence | 196 ms | 1 | stable neutral | centered | only when confirmation adds value |
| LOADING | operation in progress | normally **silent** | 0 ms | 0 | — | — | use text/visual process state |
| TRANSITION | major context changes | soft layer separation + settle | 294–490 ms | 2 | broad but restrained | subtle width optional | major context only |
| OPEN | explicit inspection surface opens | tiny dry Witness | 98 ms | 1 | mid transient | centered | sparingly |
| CLOSE | explicit surface closes | reversed/reduced Witness without “whoosh” | 98 ms | 1 | mid transient | centered | sparingly |
| SELECT | commit focus/selection | micro Grain | 98 ms | 1 | dry upper-mid, low energy | centered | optional |
| FOCUS | keyboard/pointer focus | **silent by default** | 0 ms | 0 | — | — | visual focus is sufficient |

## Family logic

### Witness family
INSPECT · OBSERVE · OPEN · SELECT

Shared element: precise dry transient with minimal resonant tail.

### Relation family
DISCOVER · TRACE · CONNECT

Shared element: grain/noise that gains organized persistence.

### Settlement family
RESOLVE · VERIFY · SUCCESS

Shared element: instability decreases and stable resonance remains.

### Friction family
WARNING · ERROR

Shared element: an interruption inside the same sonic material, not a new siren universe.

## Repetition

Repeated events should suppress duplicate audio when a sound would become chatter.

Product implementations should implement rate limiting/debouncing for high-frequency sensory events.

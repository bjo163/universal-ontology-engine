# OX-DX Product Status System

This file defines **future product vocabulary**.

It does not assert that every status exists as a current engine state.

Use a status only when the underlying model or product context can support it.

| Status | Meaning | Visual cue | Color role |
|---|---|---|---|
| OBSERVED | Directly observed from available source/input | witness anchor + label | Cyan may reinforce |
| VERIFIED | Named validation passed at known scope | verification cut + label | Acid may reinforce |
| DERIVED | Computed through a defined transformation | offset/split derivation mark + label | neutral/muted |
| PROJECTED | Explicit justified alternate/canonical view | displaced frame/bridge + label | Ultraviolet may reinforce |
| UNVERIFIED | Result exists but named verification is not established | open witness + break + label | muted |
| UNKNOWN | Current evidence cannot establish value/relation | deliberate void/break + label | muted |
| EXPERIMENTAL | Exploratory and non-canonical/non-stable | explicit label + experimental marker | contextual accent |
| GATED | Intentionally blocked behind acceptance conditions | interrupted boundary + label | restrained attention |
| PLANNED | Roadmap intent, not implemented capability | outline/ghosted geometry + label | muted |

## Status construction rule

Every status marker uses at least two of:
- text label;
- icon or geometry;
- persistence/pattern;
- color accent.

Color alone is forbidden.

## VERIFIED

Always expose verification scope when ambiguity matters.

## PROJECTED

Projection must never look like source replacement.

## UNKNOWN

Unknown is not necessarily an error.

## Technical boundary

Repository maturity language remains authoritative for current technical claims.

Product surfaces must map technical states deliberately instead of assuming these UI labels are one-to-one engine enums.

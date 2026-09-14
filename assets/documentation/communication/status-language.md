# OX-DX Status Language

Brand Bible v1.0 is the master brand authority. This file remains the specialist for technical maturity wording.

Technical maturity words are claims. Use them only when repository evidence supports them.

## Allowed statuses

### EXPERIMENTAL

A concept or implementation exists for exploration. Compatibility, completeness, or contract stability is not promised.

### IN DEVELOPMENT

Active implementation is underway. The work is not yet represented as complete.

### IMPLEMENTED

The defined capability exists in code. This does not automatically mean hardened, production-ready, or exhaustively verified.

### HARDENED

The capability is implemented and protected by the repository's stated invariants, tests, and gates at the claimed scope.

### GATED

Progress exists, but the next capability boundary is intentionally blocked until explicit acceptance evidence passes.

### PLANNED

The capability is part of the roadmap but is not implemented as a complete subsystem.

### VERIFIED

A specific claim has passed the named validation or evidence check. Always state what was verified when ambiguity is possible.

## Avoid

Do not use:

- almost done;
- basically complete;
- production-ish;
- fully universal;
- AI-ready;
- enterprise ready;
- battle-tested;

unless the repository defines and proves an equivalent claim.

## Current repository truth at audit time

- Universal Ontology contract: **v1.0.0**.
- Stable implementation release in the Brand Bible audit snapshot (2026-09-14 07:14 UTC): **v0.1.7**.
- Latest preceding prerelease in that snapshot: **v0.1.7-dev.21**. `main/Cargo.toml` reports **0.1.7**.
- README/engineering status describes Phases 1–6 as **HARDENED**, Phase 7 as **GATED**, and later phases as **PLANNED**.

Release versions change faster than communication policy. For live status, trust generated repository metadata and GitHub Releases instead of copying these audit-time values into general marketing copy.

## Status sentence pattern

Use:

> Rust syntax observation is **HARDENED** at the repository's defined Phase 5 scope.

Not:

> Rust is solved.

Use:

> Runtime observation is **PLANNED**.

Not:

> Runtime intelligence is coming soon.

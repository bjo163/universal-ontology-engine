# OX-DX Microcopy

Microcopy should feel operational: short, precise, calm.

## Actions

Prefer:

- Explore
- Inspect
- Observe
- Resolve
- Trace
- Read Evidence
- View Graph
- View Specification
- View Source
- Go Deeper
- Validate
- Compare

Avoid:

- Get Started Now!!!
- Unlock Magic
- Supercharge
- Revolutionize
- Discover Everything

## Navigation

Use nouns for destinations and verbs for actions.

Good: `Specification`, `Architecture`, `Graph`, `Evidence`, `Releases`.

Good action: `Inspect Level`, `View Source`.

Avoid vague destinations such as `Solutions`, `Magic`, or `Platform` unless they have a concrete defined meaning.

## Status

Use the exact vocabulary from `status-language.md`.

Examples:

- `HARDENED`
- `GATED`
- `PLANNED`
- `VERIFIED`

Do not use “almost done” or “basically complete.”

## Errors

State what failed and what evidence is available.

Good:

> Registry validation failed: expected 49 canonical levels; observed 48.

Good:

> Unsupported language: ruby. No parser observation was produced.

Avoid:

> Something went wrong.

## Loading

Good:

> Inspecting repository evidence…

Good:

> Resolving graph view…

Do not imply measurable progress unless progress is actually measured.

## Empty states

Good:

> No supported evidence observed.

Good:

> No relation established from the current evidence.

Avoid:

> Nothing exists here.

## CLI output

CLI copy should remain factual and script-friendly. Preserve existing command names and machine-readable modes. Error text should name the failed operation and return an appropriate non-zero exit code.

## GitHub labels

Prefer concrete maturity or work-state labels: `status:gated`, `status:planned`, `type:bug`, `type:docs`, `area:graph` when such a label system is adopted.

Do not create labels that imply unsupported capability.

## Documentation links

Prefer:

- View specification
- Read architecture
- Inspect implementation
- See validation
- Read evidence

# OX-DX TUI Startup Experience

## Principle

Startup is restrained.

It communicates real readiness, not theatrical boot behavior.

## Canonical rhythm

`signal → identity → system state → ready`

## Current repository truth

Normative ontology:
- Universal Ontology **1.0.0**
- 49 canonical levels

Current CLI commands observed in repository source:
- `levels`
- `inspect`
- `validate`
- `discover`
- `parse`
- `self`

The TUI does not claim future modes are implemented.

## Runtime startup wording

Only show values after the TUI has actually established them.

Example:

```text
OX-DX
Universal Ontology & Experience Engine

ontology   1.0.0 · 49 levels
registry   specifications/universal-ontology-v1.0.json
mode       evidence-first inspection

READY
```

If registry loading has not completed:

```text
OX-DX
ontology   1.0.0
registry   loading…
```

Do not print `READY` until the required initialization step has succeeded.

## Failure

```text
OX-DX

registry   ERROR
source     specifications/universal-ontology-v1.0.json
detail     failed to load registry

startup stopped
```

Use the actual error text/scope available.

## Motion

Full sensory:
- optional 98 ms identity reveal;
- optional 196 ms state reveal.

Reduced motion:
- render final startup state immediately.

Silent:
- default.

## Avoid

- large ASCII logo;
- fake memory/CPU checks;
- fake packet scanning;
- random hexadecimal;
- matrix rain;
- “access granted” language;
- fake engine version;
- stale release version hardcoded into the visual spec.

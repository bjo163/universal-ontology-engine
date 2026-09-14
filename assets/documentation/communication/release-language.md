# OX-DX Release Language

Release communication is factual, compact, and explicit about remaining boundaries.

## Canonical structure

### WHAT CHANGED

List concrete shipped changes.

### WHY IT MATTERS

Explain the engineering consequence, not the marketing excitement.

### WHAT IS NOW VERIFIED

Name the tests, gates, invariants, or evidence that support the release claim.

### WHAT REMAINS

State gated, planned, experimental, or intentionally excluded scope.

## Stable release

Tone: calm, precise, final for the stated version.

Use “stable” only for the repository's stable release channel. Do not expand “stable” into claims of universal production fitness unless separately proven.

## Development release

Tone: inspectable and provisional.

State the prerelease identifier. Highlight what changed since the previous development release and what is not yet promoted.

## Milestone

Describe the acceptance boundary that was crossed. A milestone is not automatically a release.

## Phase completion

State:
- phase scope;
- acceptance criteria;
- verification evidence;
- next gate.

Do not say a later phase exists merely because its vocabulary exists in the ontology.

## Breaking change

Lead with the break. Name affected contracts, commands, formats, or consumers. Provide migration guidance when available.

## Bug fix

Name the incorrect behavior, the corrected behavior, and the verification.

## Security change

Describe impact and remediation without exposing unsafe exploit detail unnecessarily. Do not use security language for ordinary correctness fixes.

## Example

### WHAT CHANGED

Cross-language syntax observation now emits deterministic evidence for the supported adapters.

### WHY IT MATTERS

Language-specific syntax can be compared without turning parser output into a replacement ontology.

### WHAT IS NOW VERIFIED

Ordering and observation output are deterministic at the stated adapter scope.

### WHAT REMAINS

Compiler-grade semantic analysis and broader semantic projection remain outside this capability.

## Version discipline

Keep ontology version and implementation release version separate:

- Universal Ontology: `1.0.0`.
- Engine/repository release: independent `0.x` train.

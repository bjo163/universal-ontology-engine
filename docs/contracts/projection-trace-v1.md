# Projection Trace Contract v1

Contract ID: `oxdx.semantic-projection.trace/v1`

This contract is bound to `oxdx.semantic-projection.rfc/v1` and implements the explainability requirements of issue #51.

## Required properties

- The native/source node ID is preserved exactly.
- Repository, source identifier, and source span are optional only when unavailable from observation evidence.
- Native language, native kind, and metadata remain machine-readable evidence.
- Every evaluated rule is identified by stable rule ID and version.
- Normalized evidence is additive; it never replaces native evidence.
- A successful projection records target TYPE/KIND and `PROJECTS_TO`.
- An unresolved projection records deterministic unresolved codes and candidate targets.
- Unresolved rule IDs must have known rule versions; missing versions fail the trace contract.
- Multiple versions for one rule ID fail the trace contract.
- Record ordering uses deterministic ordered collections; identical inputs serialize byte-identically with the same serializer/version.

Human-readable explanations may be derived from this record, but prose is never authoritative over the machine trace.

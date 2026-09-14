# Semantic Projection Conformance Corpus v1

Corpus schema: `universal-ontology-engine/semantic-conformance-v1`

Behavioral boundary: `oxdx.semantic-projection.rfc/v1`

Canonical issue: #55.

## Purpose

This corpus is a deterministic, network-free proof fixture for the Phase 7 contract chain:

`native evidence -> normalization -> projection rule -> resolution -> projection trace -> graph materialization`

The fixture is not a production semantic rule catalog. Conformance-only rules exercise the public Phase 7 contracts without granting new language semantics.

## Required coverage

- positive normalization/projection cases for Rust, TypeScript, JavaScript, Python, Go, Java, and Kotlin;
- cross-language callable equivalence only through explicit normalization classes;
- language-specific distinction such as callable declarations vs callable bindings/method declarations;
- duplicate names in distinct scopes retaining distinct source IDs;
- unsupported observable syntax producing no projection;
- malformed-but-observable evidence producing no fabricated semantic target;
- insufficient required evidence remaining unresolved;
- competing targets remaining ambiguous independent of rule order;
- explicit conflicts remaining conflicting;
- graph materialization that skips canonical levels without fabricating intermediate structural nodes;
- repeated suite execution producing byte-identical ordered output;
- a language-specific normalization extension not altering unrelated language output.

## Authority rules

1. Native evidence remains authoritative and is preserved in traces.
2. Expected positive cases identify the native evidence, normalization rule, normalization class, projection rule/version, target canonical TYPE, and `PROJECTS_TO` edge.
3. Every negative fixture contains an explicit machine outcome plus a human rationale; the rationale is documentation, not truth input.
4. Unknown/ambiguous/conflicting cases never materialize an accepted projection merely to make them visible.
5. Fixture execution is local and must not fetch packages, repositories, or semantic meaning from the network.
6. A fixture cannot hardcode around a missing production contract. Contract gaps discovered by conformance must be fixed separately before the suite may pass; #68 is the first example of this rule.

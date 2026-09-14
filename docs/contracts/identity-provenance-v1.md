# OX-DX Identity & Provenance Contract v1

Status: **MVP EXIT / Phase 1–6**

## Core rule

Identity answers **what evidence-backed thing is this?** Provenance answers **where was it observed?** They may be related, but provenance is not allowed to become the sole semantic identity.

## Stable under irrelevant movement

For a named syntax observation under the same structural parent, inserting blank lines or comments before the declaration must not change its semantic node ID. This is already enforced for Rust AST and TypeScript syntax observations.

Repeated scans of identical committed Rust, TypeScript, and Python fixtures must remain byte-identical.

## Parent-scoped identity

Two same-named declarations under different structural parents are distinct because their parent scopes differ. A name is not globally unique.

Within one parent scope, duplicate native observations are disambiguated deterministically by occurrence. Anonymous observations use deterministic ordinals because no stronger native identifier is available.

## Legitimate identity changes

The MVP does not promise identity preservation across every file rename or move. Structural file-boundary IDs intentionally include evidence-backed relative paths. Moving a declaration to a different structural parent can therefore be a legitimate identity change.

A source span changing while the named declaration remains in the same structural parent is not, by itself, an identity change.

## Unicode and case

Native names are preserved as emitted by the adapter. OX-DX does not lowercase semantic identifiers to hide host-filesystem differences. Unicode identifiers supported by a native adapter remain native evidence.

## Provenance

`SourceSpan` records source line/column provenance when the adapter can provide it. It is not included as the sole basis of named semantic identity.

## Required proofs

The Phase 1–6 exit gate relies on:

- Rust semantic ID line-movement regression test in `ontology-discovery`;
- TypeScript named-ID line-movement regression test in `syntax_projection`;
- committed Rust/TypeScript/Python repeated-scan byte-determinism E2E tests;
- deterministic graph storage/traversal ordering;
- explicit `PROJECTS_TO` edges that do not mutate structural parent ownership.

Any future change that strengthens cross-file identity must preserve this provenance/identity separation and must not retroactively claim guarantees that Phase 1–6 did not prove.

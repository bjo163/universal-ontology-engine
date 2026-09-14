# OX-DX Community Language

The OX-DX community should be curious, precise, respectful, and evidence-driven.

## Core principle

**Disagreement should attack assumptions, not people.**

## Evidence

When making a claim, show the source when practical: specification, schema, code, test, reproduction, graph output, CLI output, or observed repository state.

Preferred:

> The current parser emits this as syntax evidence. I do not see evidence yet for the semantic projection.

Avoid:

> Obviously this is a FUNCTION.

## Proposals

Label proposals as proposals.

State:
- the problem;
- the evidence;
- the proposed distinction;
- the affected contract;
- the acceptance criteria.

A proposal does not become canonical because it is elegant.

## Ontology changes

Ontology changes require stronger discipline than ordinary implementation changes.

Ask:
- Does an existing TYPE already cover the concept?
- Is this actually a KIND?
- Is this a relation rather than a level?
- What evidence requires the change?
- What compatibility boundary changes?

Do not use brand language as proof for ontology changes.

## Technical disagreements

Prefer:

> This interpretation conflates provenance with identity because the proposed ID depends on source line numbers.

Avoid:

> Your design is wrong.

Explain the invariant and the consequence.

## Experiments

Use `EXPERIMENTAL` explicitly. Experiments may violate future assumptions while exploring a question; they must not silently become canonical.

## Rejected ideas

Record why an idea was rejected:
- insufficient evidence;
- duplicates existing terminology;
- violates an invariant;
- creates false hierarchy;
- exceeds current scope;
- useful but deferred.

Reject the assumption, not the contributor.

## Review culture

Good review questions:

- What evidence supports this classification?
- Is this containment or another relation?
- Does identity survive path or line movement?
- What is implemented versus planned?
- Which source of truth governs this claim?

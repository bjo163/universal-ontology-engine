# OX-DX Intelligent Language

Sophisticated communication does not need academic fog.

OX-DX uses a three-step reasoning pattern:

1. **State the observation.**
2. **State the distinction.**
3. **State the consequence.**

## Example

A source span tells us where something was observed.

It does not define what the thing is.

Therefore provenance and identity remain separate.

## Another example

An `EXECUTION` can be associated with a target.

Association is not structural ownership.

Therefore the relationship is modeled explicitly with `OBSERVED_AT`, not by forcing execution into the target's containment tree.

## Writing rules

- Put the observable fact first.
- Name the distinction directly.
- Use the repository's canonical terminology.
- State the consequence in engineering terms.
- Separate current implementation from roadmap.
- Prefer one concrete example to three abstract synonyms.
- Do not use complexity as a status symbol.

## Documentation progression

OX-DX documentation should move:

```text
CONCEPT
  ↓
DISTINCTION
  ↓
EXAMPLE
  ↓
IMPLEMENTATION
  ↓
REFERENCE
```

Use progressive disclosure.

**Simple first. Deep later.**

A newcomer should understand the boundary before reading the complete normative contract. An expert should always be able to reach the contract, source, or test that supports the claim.

## Precision without bureaucracy

Bad:

> In accordance with the holistic multi-dimensional ontological framework, all source-level phenomena are comprehensively understood.

Better:

> Source evidence is observed first. Projection is explicit. Missing evidence stays missing.

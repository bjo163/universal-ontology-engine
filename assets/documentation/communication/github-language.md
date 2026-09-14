# OX-DX GitHub Communication

GitHub is engineering evidence, not a marketing stage.

## Issues

Every substantive issue should describe:

1. **Problem** — what is wrong, missing, or ambiguous?
2. **Evidence** — what repository state, test, reproduction, or contract establishes the problem?
3. **Scope** — what may change and what must not?
4. **Acceptance criteria** — what observable state means the issue is complete?
5. **Verification** — how will completion be proven?

Avoid vague issues such as “make ontology smarter.”

## Pull requests

A PR should describe:

1. **Change** — what changed?
2. **Reason** — why was it necessary?
3. **Impact** — what contracts or behavior are affected?
4. **Validation** — what tests/gates/evidence passed?
5. **Remainder** — what is intentionally not solved?

Do not use a PR body to claim capability that the diff does not implement.

## Discussions

Use discussions for proposals, questions, design exploration, and unresolved assumptions. Distinguish proposal language from accepted contract language.

Preferred:

> Proposal: treat this as a new KIND under ELEMENT. Evidence and registry ownership still need review.

Avoid:

> This is now the ontology.

## Releases

Follow `release-language.md`. Link changes to merged evidence and state what remains.

## Commit messages

Prefer conventional, scoped messages:

- `feat(graph): ...`
- `fix(discovery): ...`
- `docs(communication): ...`
- `test(registry): ...`
- `chore(release): ...`

The subject should describe the change, not the emotion around it.

## Project description

Use the canonical one-line definition or a shorter approved one-liner from `one-liners.md`.

## Review language

Critique assumptions, invariants, evidence, and consequences.

Do not attack people.

Good:

> This mapping relies on a filename heuristic. What evidence establishes TYPE=MODULE here?

Bad:

> You do not understand the ontology.

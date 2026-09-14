# OX-DX Error States

Errors are precise observations about failed operations.

Distinguish:

## INVALID INPUT
Input violates a known contract.

Example:
> Invalid input: expected a canonical TYPE identifier.

## UNSUPPORTED FORMAT
Input exists but the current product cannot inspect it.

Example:
> Unsupported format. No representation reader is available for this input.

## MALFORMED EVIDENCE
Input was observed but cannot be parsed or validated as expected.

Example:
> Malformed input observed. Source context is preserved below.

## UNAVAILABLE SOURCE
The referenced source cannot currently be accessed.

Do not convert unavailable into unknown semantic meaning.

## UNKNOWN STATE
Evidence does not establish the requested value.

Unknown is not always an operational error.

## INTERNAL FAILURE
The product failed independently of source validity.

Example:
> Inspection failed internally. The source evidence was not modified.

## Error anatomy

Where possible show:
- operation;
- target;
- failure class;
- evidence/context retained;
- safe retry or alternate action;
- technical detail disclosure.

## Avoid

> Something went wrong.

unless a lower-level environment prevents any more specific statement.

## Evidence preservation

Errors must not discard source/provenance needed for diagnosis.

Do not expose secrets or sensitive internals merely for “technical” styling.

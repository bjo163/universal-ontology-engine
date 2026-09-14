# Semantic Resolution Policy v1

Contract: `oxdx.semantic-projection.resolution/v1`  
RFC: `oxdx.semantic-projection.rfc/v1`  
Issues: #53, #49

Uncertainty is a first-class deterministic result. Supported unresolved codes are:

- `unsupported`
- `insufficient`
- `ambiguous`
- `conflicting`
- `invalid_evidence`

Rule order never decides truth. Candidate ordering is stable and deduplicated. Multiple rules may support the same target. Distinct justified targets are `ambiguous`. Explicitly incompatible rule pairs are `conflicting`. A non-`PROJECTS_TO` projection candidate is invalid evidence.

Unknown is not a process error by default and no generic confidence score is defined.

Executable policy: `crates/ontology-semantic/src/resolution.rs`.

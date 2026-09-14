# Semantic Projection Rule Contract v1

Contract: `oxdx.semantic-projection.rules/v1`  
RFC: `oxdx.semantic-projection.rfc/v1`  
Issues: #50, #49

A projection rule is an inspectable, versioned declaration. It consumes only declared evidence, names required and optional evidence/fields, identifies one canonical semantic target, and emits `PROJECTS_TO`.

Rules MUST NOT mutate native evidence, use hidden precedence, or depend on registration/traversal order. Missing required evidence resolves to `insufficient` or `invalid_evidence`; incompatible evidence is `unsupported`.

Stable identity is `(rule_id, rule_version)`. Any rule change that can alter output for identical evidence requires a visible rule-version change.

Executable contract: `crates/ontology-semantic/src/rules.rs`.

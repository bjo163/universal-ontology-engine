# Cross-Language Normalization Contract v1

Contract: `oxdx.semantic-projection.normalization/v1`  
RFC: `oxdx.semantic-projection.rfc/v1`  
Issues: #52, #44, #49

Normalization is deterministic comparison evidence, not ontology truth. It preserves language, native kind, and canonical source type. A normalization rule matches exact declared native evidence and emits a narrow comparison class.

Foundation classes are intentionally conservative: declared type, callable declaration, callable binding, method declaration, named value binding, property binding, and module-boundary statement.

No lexical fallback exists. Unsupported or ambiguous constructs remain unnormalized. In particular the current Go `declaration` observation is not normalized as a module-boundary statement because the Phase 6 adapter groups package/import/var/const evidence too broadly.

Executable table and contract: `crates/ontology-semantic/src/normalization.rs`.

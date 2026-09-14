# RFC: Phase 7 Semantic Projection v1

Status: **ACCEPTED CANDIDATE**  
Contract ID: `oxdx.semantic-projection.rfc/v1`  
Target release: `v0.2.0`  
Issue: #49  
Depends on: #48 `GO — PHASE 7 MAY OPEN`  
Baseline: stable `v0.1.10` Phase 1–6 contract freeze

## 1. Decision

Phase 7 introduces a deterministic **semantic projection** layer over the native evidence graph produced by the hardened Phase 1–6 engine.

Semantic projection interprets inspectable native evidence through versioned rules and may emit an explicit `PROJECTS_TO` relation toward a canonical semantic view. It does **not** rewrite native syntax, structural ownership, source provenance, runtime observations, or representation/encoding facts.

A projection exists only when the evidence and the active versioned rule contract justify it. If the engine cannot justify exactly what the rule requires, the correct output is an inspectable unresolved result rather than an invented semantic edge.

## 2. Why Phase 7 exists

Phase 1–6 can discover and preserve repository/native-language evidence without requiring every language to use the same syntax. That is necessary but insufficient for consumers that need to ask whether different native constructs represent the same canonical semantic idea.

Phase 7 creates that canonical semantic view while preserving the distinction between:

1. what the source actually contains;
2. how a language adapter described it;
3. how optional normalization relates comparable evidence;
4. what a versioned projection rule concludes;
5. why that conclusion was produced.

The central design rule is:

> Native evidence is authoritative. Projection is derived, explicit, versioned, and reversible by inspection.

## 3. Existing contracts that remain authoritative

Phase 7 is additive. The following Phase 1–6 rules are not weakened or silently redefined:

- each ontology node has exactly one canonical `TYPE`;
- `KIND` is a specialization/native distinction owned by one canonical type;
- `parent` / `CONTAINS` express structural ownership only;
- `PROJECTS_TO` is semantic projection and is distinct from containment;
- graph relations do not implicitly become tree children;
- `REPRESENTED_AS` is representation/encoding, not semantic ownership;
- source spans and other provenance are evidence, not node identity;
- skipped intermediate canonical levels are allowed when native evidence does not justify them;
- discovery is read-only with respect to the inspected workspace;
- unsupported or malformed evidence remains observable rather than being fabricated into a richer structure.

Phase 7 MUST remain compatible with valid `v0.1.10` graphs. A Phase 1–6 graph requires no migration merely because Phase 7 exists.

## 4. Terminology

### 4.1 Native evidence

Facts emitted by repository discovery or a language adapter that describe what was directly observed in the source/workspace.

Examples include a native syntax kind, canonical type mapping, native metadata, node identity, source location, and graph relations already justified by discovery.

Native evidence is immutable input to projection.

### 4.2 Normalized evidence

An optional deterministic comparison form produced under the separate cross-language normalization contract (#52).

Normalization may make evidence comparable but never deletes, replaces, or becomes more authoritative than the native evidence that produced it.

### 4.3 Projection rule

A versioned, inspectable rule defined by the rule contract (#50). A rule declares its accepted evidence, required fields, target semantic type/kind, emitted relation, and unresolved behavior.

### 4.4 Projection candidate

A rule evaluation result that is sufficiently formed to participate in resolution but is not yet necessarily accepted as graph truth.

Candidates are not graph edges by themselves.

### 4.5 Accepted projection

A deterministic conclusion that passes the rule contract and ambiguity/conflict policy and is materialized as an explicit semantic projection, normally using `PROJECTS_TO`.

### 4.6 Projection trace

Machine-readable explainability data defined in #51. It records the source evidence, rule/version, any normalized evidence, candidate decision, target, and unresolved reason where applicable.

### 4.7 Unresolved result

A deterministic non-projection outcome. Phase 7 recognizes at least:

- `unsupported`
- `insufficient`
- `ambiguous`
- `conflicting`
- `invalid_evidence`

An unresolved result is not a projection edge.

## 5. Goals

Phase 7 MUST provide a foundation that can:

1. evaluate semantic rules deterministically;
2. preserve exact native evidence and provenance;
3. compare justified cross-language evidence without pretending syntax is identical;
4. represent uncertainty without guessing;
5. explain every accepted, rejected, or unresolved projection;
6. add semantic graph edges without corrupting structural ownership;
7. expose projection only as an explicit opt-in execution mode until the `v0.2.0` exit gate passes;
8. remain measurable against the frozen `v0.1.10` performance baseline.

## 6. Non-goals

Phase 7 MUST NOT absorb any of the following scopes:

- full runtime observation or execution tracing (Phase 8);
- binary/encoding readers (Phase 9);
- token/character representation semantics (Phase 10);
- bounded bit reading (Phase 11);
- general query/certification engine work (Phase 12);
- UI, TUI, dashboard, product-shell, or visualization work;
- compiler-grade name resolution unless separately proven and versioned;
- macro expansion, build execution, dependency installation, or arbitrary code execution;
- hidden AI/LLM inference as an authority for ontology truth;
- probabilistic confidence scores without a separately specified measurable basis;
- filling missing canonical levels merely to make a hierarchy contiguous;
- converting repository paths, filenames, comments, naming style, or lexical similarity into semantic truth unless an explicit rule declares the evidence and its limitations.

## 7. Eligible inputs

A projection evaluation may consume only evidence that is explicitly represented by the engine and declared by the active projection rule.

Eligible input classes are:

1. **Canonical node facts**
   - stable native node ID;
   - canonical `TYPE`;
   - native/registered `KIND` where available.

2. **Native syntax facts**
   - language identifier;
   - adapter-emitted native syntax kind;
   - deterministic adapter metadata explicitly covered by the adapter capability contract.

3. **Graph evidence**
   - existing Phase 1–6 structural or semantic-neutral relations when a rule explicitly requires them;
   - structural ownership may be inspected but MUST NOT be rewritten by projection.

4. **Provenance**
   - repository/source identity;
   - source span/path/line metadata where available;
   - provenance may support explainability but does not by itself create semantic identity.

5. **Normalized evidence**
   - only after #52 defines a versioned normalization contract;
   - normalization must retain a link back to the exact native evidence.

Ineligible input includes hidden process state, wall-clock iteration order, network-fetched meaning, unversioned heuristics, UI state, or data not represented in the projection trace.

## 8. Minimum evidence rule

There is no universal shortcut such as “a syntax node exists, therefore project it.” Each projection rule MUST declare its own minimum evidence.

Every accepted projection MUST be able to identify at least:

- source/native node ID;
- source canonical `TYPE`;
- native evidence kind(s) used;
- active registry/ontology contract version;
- projection rule ID and rule version;
- target canonical `TYPE` and target `KIND` when a kind is asserted;
- emitted edge kind;
- provenance references that were available and used;
- deterministic resolution outcome.

If a required field is absent, the rule MUST return `insufficient` or `invalid_evidence` as defined by #53. It MUST NOT infer the missing fact from naming conventions or iteration order.

## 9. Projection pipeline

The logical Phase 7 pipeline is:

```text
Phase 1–6 native graph
        │
        ▼
eligible evidence extraction
        │
        ├──── optional normalized evidence (#52)
        │
        ▼
versioned rule evaluation (#50)
        │
        ▼
projection candidates
        │
        ▼
ambiguity/conflict resolution (#53)
        │
        ├── accepted ──────> PROJECTS_TO + projection trace (#51)
        │
        └── unresolved ────> unresolved trace, no projection edge
```

The pipeline MUST NOT mutate the native graph while evaluating candidates.

## 10. Output boundary

Phase 7 may produce two classes of output.

### 10.1 Accepted semantic projection

An accepted projection consists of:

- source native node reference;
- semantic target node/reference;
- explicit semantic edge, normally `PROJECTS_TO`;
- projection trace;
- rule/version identity;
- any normalization contract/version identity used.

### 10.2 Unresolved projection outcome

An unresolved outcome consists of:

- source native node reference;
- deterministic outcome code;
- candidate targets/rules where relevant;
- exact evidence references;
- reason data suitable for machine processing;
- rule and normalization versions involved.

An unresolved outcome MUST NOT be represented as `PROJECTS_TO` merely to make it visible. Visibility belongs in trace/result data, not in a false semantic edge.

## 11. Core invariants

The following invariants are normative.

### SP-01 — Native authority

Projection MUST NOT modify, delete, relabel, or replace native evidence.

### SP-02 — Structural isolation

`parent` and `CONTAINS` remain structural ownership only. A projection target MUST NOT become a structural child because of semantic interpretation.

### SP-03 — Explicit projection

Semantic interpretation that becomes graph truth MUST use an explicit projection relation, normally `PROJECTS_TO`.

### SP-04 — Representation isolation

`REPRESENTED_AS` MUST NOT be used as a substitute for semantic projection.

### SP-05 — Runtime isolation

Execution/runtime observation relations such as `OBSERVED_AT` remain outside Phase 7 semantic ownership.

### SP-06 — Evidence completeness

A rule may accept only when all evidence it declares as required is present and valid.

### SP-07 — Determinism

Identical native evidence + identical normalized evidence + identical ontology/registry version + identical rule set/version MUST yield identical ordered projection results and traces.

### SP-08 — Rule visibility

No hidden heuristic may create accepted semantic graph truth.

### SP-09 — Uncertainty preservation

Unsupported, insufficient, ambiguous, and conflicting evidence MUST remain distinguishable. Unknown is not an error by default and MUST NOT silently collapse to the first candidate.

### SP-10 — Provenance preservation

Every accepted or unresolved result MUST remain traceable to the exact evidence that produced it.

### SP-11 — Adapter non-ownership

Language adapters describe native evidence. They do not own canonical ontology semantics and MUST NOT directly declare accepted semantic projection truth outside the projection-rule layer.

### SP-12 — Cross-language restraint

Lexical or syntactic similarity across languages is not semantic equivalence. Cross-language equivalence requires the explicit normalization contract and applicable projection rules.

### SP-13 — No hierarchy fabrication

Projection does not require contiguous ontology levels and MUST NOT invent intermediate nodes.

### SP-14 — Idempotent materialization

Re-evaluating the same source/rule/version/target combination MUST NOT create duplicate accepted projections.

### SP-15 — Stable resolution

Rule registration order, filesystem traversal order, hash-map iteration order, or concurrency scheduling MUST NOT decide semantic truth.

### SP-16 — Read-only source behavior

Projection may produce engine output artifacts but MUST NOT mutate the inspected workspace/source tree.

## 12. Competing interpretations

Phase 7 explicitly permits more than one candidate interpretation during evaluation.

The default resolution policy is conservative:

- exactly one justified candidate after deterministic resolution → accepted projection;
- no applicable rule → `unsupported`;
- applicable rule but missing required evidence → `insufficient`;
- multiple independently valid targets without a deterministic contract-backed winner → `ambiguous`;
- applicable rules assert mutually incompatible conclusions → `conflicting`;
- malformed required evidence → `invalid_evidence`.

A rule contract MAY define a deterministic precedence relation only when that precedence is itself explicit, versioned, inspectable, and semantically justified. “First rule wins” is prohibited.

Candidate lists in unresolved output MUST use stable ordering. #50 and #53 own the exact machine schema and ordering keys.

## 13. Confidence scores

Phase 7 v1 defines **no generic confidence score**.

A confidence number without a measurable, reproducible interpretation would hide uncertainty rather than model it. Any future score requires its own contract defining:

- what is measured;
- how the value is computed;
- how evidence changes the value;
- what reproducibility guarantees exist;
- why the score is preferable to categorical unresolved states.

Until such a contract exists, implementations MUST NOT emit arbitrary semantic confidence values.

## 14. Explainability requirement

Every rule evaluation that reaches the public Phase 7 output surface MUST be explainable using machine data.

The trace contract in #51 MUST be able to answer at minimum:

- Which source/native node was evaluated?
- Which evidence fields were used?
- Which evidence fields were missing or rejected?
- Which normalization record/version was used, if any?
- Which projection rule ID/version ran?
- Which candidate target(s) were produced?
- Why was one target accepted, or why was the result unresolved?
- Which graph edge was emitted, if any?

Human-readable prose may be generated from this trace, but prose MUST NOT be the only provenance mechanism.

## 15. Versioning

There are four independent version identities in Phase 7:

1. ontology/registry contract version;
2. this semantic projection RFC contract version;
3. normalization contract/version (#52), when used;
4. projection rule ID/version (#50).

An accepted projection trace MUST record enough version information to reproduce the semantic decision.

### 15.1 RFC compatibility

`oxdx.semantic-projection.rfc/v1` is the behavioral boundary for Phase 7 foundation work.

A change is breaking and requires a new RFC major version when it changes, for example:

- what may count as eligible evidence;
- structural-vs-semantic ownership boundaries;
- accepted unresolved categories in an incompatible way;
- whether hidden/non-traceable inputs may influence truth;
- default rules for competing interpretations;
- whether projection may mutate native evidence.

Clarifications that do not change observable contract behavior may update documentation without changing the major contract ID.

### 15.2 Rule versioning

Rule versioning is owned by #50. Rule changes that can alter accepted target/output for the same evidence MUST be version-visible in trace data.

## 16. Cross-language semantics boundary

Phase 7 may compare constructs from multiple languages, but it does not assume they mean the same thing.

The required sequence is:

```text
native adapter evidence
        ↓
explicit normalization candidate (#52)
        ↓
versioned projection rule (#50)
        ↓
resolution (#53)
```

The engine MUST preserve language ID and original native kind throughout this chain.

Unsupported compiler-grade semantics remain out of scope unless a future capability contract demonstrates them explicitly.

## 17. Graph semantics boundary

The semantic projection layer may add graph evidence but MUST preserve relation domains.

- structural ownership → `parent` / `CONTAINS`;
- semantic projection → `PROJECTS_TO`;
- runtime observation → runtime/observation relations such as `OBSERVED_AT`;
- representation/encoding → `REPRESENTED_AS`;
- other graph relations retain their existing explicit meanings.

Adding a semantic edge MUST NOT mutate the structural tree.

#54 owns executable graph validation, deterministic deduplication, and cycle policy under these boundaries.

## 18. Projection target rules

A projection target represents a canonical semantic view justified by a rule.

A target MUST:

- have a canonical `TYPE`;
- have a `KIND` only when the rule can justify that specialization;
- be distinguishable from the source/native node unless the graph model explicitly identifies the same canonical entity without erasing native provenance;
- never be created solely because a canonical intermediate level exists;
- remain explainable from source evidence and rule identity.

#50 owns exact target construction/reference rules.

## 19. Failure and error behavior

Semantic uncertainty and operational failure are distinct.

The following are expected semantic outcomes and SHOULD NOT by themselves be process errors:

- `unsupported`
- `insufficient`
- `ambiguous`
- `conflicting`

The following may be operational/validation failures depending on the exact contract established in #50/#53:

- structurally invalid projection rule document;
- impossible/unknown canonical target type;
- corrupted trace/version identity;
- invalid graph edge domain;
- malformed evidence that violates a required machine schema.

The existing CLI/error contract remains authoritative for process exit behavior until #56 explicitly exposes Phase 7 execution.

## 20. Deterministic ordering

Phase 7 outputs MUST be stable independent of traversal or registration order.

The exact machine comparator is delegated to #50/#53, but it MUST be composed only of stable values such as:

- source/native stable ID;
- projection rule ID;
- projection rule version;
- canonical target type ordinal/ID;
- canonical target kind ID;
- deterministic evidence/normalization identity.

Unstable memory addresses, timestamps, thread scheduling, filesystem enumeration order, or hash-map iteration order are prohibited ordering inputs.

## 21. Security and safety boundary

Phase 7 semantic projection is analysis, not execution.

Projection MUST NOT require:

- running project code;
- executing build scripts;
- installing dependencies;
- following arbitrary symlinks outside existing discovery safety boundaries;
- network access to infer meaning;
- mutating source files;
- invoking language compilers in ways that execute project-controlled code unless a future, separately reviewed phase explicitly introduces such capability.

The Phase 1–6 hostile-workspace/read-only contract remains in force.

## 22. Opt-in requirement

Until #58 certifies the `v0.2.0` semantic foundation, semantic projection MUST be exposed only as an explicit opt-in execution mode defined by #56.

Existing discovery behavior and its frozen JSON/CLI contracts MUST NOT silently begin returning semantic projections by default.

This preserves compatibility for Phase 1–6 consumers.

## 23. Performance boundary

Phase 7 may add computation, but its cost must be independently measurable.

#57 MUST compare semantic projection overhead against the frozen `v0.1.10` baseline using reproducible fixtures. A semantic feature is not allowed to hide pathological behavior behind a broader end-to-end timing number.

## 24. External conformance

Phase 7 must eventually prove behavior on both committed fixtures and at least the pinned external-reference specimen used by the MVP exit process.

External evidence MUST be pinned to a reproducible repository revision. Moving default branches are not acceptable conformance inputs for the `v0.2.0` certification gate.

## 25. Explicitly prohibited inference

Unless a future rule contract explicitly introduces evidence that satisfies this RFC, the following are prohibited as standalone reasons for accepted semantic projection:

- “the name looks like a service”;
- “the file is under a `service/` directory”;
- “this function is probably a handler”;
- “these two language keywords look equivalent”;
- “most frameworks use this naming convention”;
- “an LLM classified it as X”;
- “the first matching rule said X”;
- “there is a missing ontology level, so create one”;
- “the source span is nearby, therefore it is the same entity”;
- “a runtime behavior is likely from the static syntax”;
- “a representation token looks semantically meaningful.”

These may become evidence only through an explicit, versioned, independently auditable contract that preserves their limitations.

## 26. Ownership map for follow-up issues

This RFC fixes the boundary; later issues own the concrete contracts and implementation.

| Issue | Owner | Must preserve this RFC |
|---|---|---|
| #50 | versioned projection rule schema/evaluator contract | yes |
| #51 | projection trace and explainability representation | yes |
| #52 | cross-language normalization contract | yes |
| #53 | unresolved/ambiguity/conflict machine policy | yes |
| #54 | graph invariants, deduplication, projection-cycle handling | yes |
| #55 | conformance fixtures | yes |
| #56 | explicit opt-in CLI/execution surface | yes |
| #57 | semantic projection performance regression budget | yes |
| #58 | `v0.2.0` semantic foundation certification | yes |

## 27. Dependency and implementation order

After this RFC is accepted:

```text
#49 RFC accepted
   │
   ├── #50 rule contract
   ├── #52 normalization contract
   └── #53 uncertainty policy

#50 ──> #51 explainability
#50 ──> #54 graph invariants

#50 + #51 + #52 + #53 + #54
              │
              ▼
          #55 fixtures
              │
              ▼
          #56 opt-in mode
              │
              ▼
          #57 perf budget
              │
              ▼
          #58 certification
```

#50, #52, and #53 may proceed in parallel only after this RFC is accepted because their ownership boundaries are independent under this document.

## 28. Acceptance criteria for this RFC

This RFC is accepted when all of the following are true:

- [x] inputs eligible for projection are explicitly bounded;
- [x] output classes are defined;
- [x] structural/native/projection/runtime/representation boundaries are explicit;
- [x] minimum evidence behavior is defined without inventing a universal heuristic;
- [x] ambiguity, conflict, unsupported, insufficient, and invalid evidence are allowed as deterministic non-projection outcomes;
- [x] provenance and explainability are mandatory;
- [x] rule and contract version identities are required;
- [x] deterministic evaluation requirements are explicit;
- [x] cross-language normalization is separate from native adapters and projection rules;
- [x] confidence scores are intentionally excluded from v1;
- [x] runtime, representation, query/certification, and UI/TUI are explicit non-goals;
- [x] Phase 1–6 contracts remain valid without migration;
- [x] follow-up ownership for #50–#58 is explicit.

## 29. Acceptance consequence

Once this document is merged into `dev` with validation green and #49 is closed as completed:

- the Phase 7 boundary is frozen as `oxdx.semantic-projection.rfc/v1`;
- #50, #52, and #53 are unblocked and may proceed in parallel;
- no implementation may violate the invariants above without revising this RFC contract through an explicit compatibility decision;
- passing #49 does **not** certify `v0.2.0`; only #58 can do that.

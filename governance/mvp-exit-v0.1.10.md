# OX-DX v0.1.10 — MVP Exit Evidence Ledger

Status: **CANDIDATE — #48 GO/NO-GO still required**

This ledger maps Target A issues to concrete contracts and executable proof. It does not open Phase 7 by itself.

| Issue | Exit requirement | Evidence |
|---|---|---|
| #38 | public CLI compatibility | `docs/contracts/cli-v1.md`, `crates/ontology-cli/tests/mvp_exit_cli.rs`, CLI smoke |
| #39 | versioned discovery JSON | `schemas/discovery-output-v1.schema.json`, fixture E2E + CI smoke |
| #40 | stable error/exit semantics | `docs/contracts/cli-v1.md`, `mvp_exit_cli.rs` exit `1/2` tests, engine invariant class `3` in canonical discovery |
| #41 | path/workspace boundary | `docs/contracts/discovery-safety-v1.md`, Linux safety tests, Windows path-contract CI |
| #42 | read-only/hostile-workspace safety | `preflight.rs`: symlink rejection, 2 MiB/file, 100k source count, depth 256; E2E safety tests |
| #43 | deterministic identity/provenance | `docs/contracts/identity-provenance-v1.md`, Rust + TypeScript line-movement regressions, repeated byte-identical MVP fixtures |
| #44 | exact language capabilities | `specifications/language-capabilities-v1.json` |
| #45 | reproducible performance baseline | `tools/mvp_benchmark.py`, `governance/performance-baseline-v0.1.10.json`, `tools/check_mvp_performance.py` |
| #46 | external-reference conformance | `specifications/external-reference-matrix-v1.json`, pinned `rocksoul-ui` external smoke |
| #47 | release provenance | `docs/contracts/release-provenance-v1.md`, committed `Cargo.lock`, release/back-sync workflows |

## Frozen Phase 1–6 machine boundaries

- ontology contract: `1.0.0`;
- discovery output schema: `universal-ontology-engine/discovery-v1`;
- self schema: `universal-ontology-engine/self-v1`;
- structural discovery mode: `structural`;
- native Rust mode: `rust-ast`;
- cross-language syntax mode: `syntax-projection`;
- public discovery remains read-only;
- no generic source tree may fabricate `UNIT` or `MODULE`;
- syntax projection uses explicit `PROJECTS_TO` and does not change structural parent ownership;
- Phase 7 remains opt-out by absence: it is not implemented/opened by this release.

## Safety ceilings

- symbolic links: rejected by public discovery preflight;
- source size: 2 MiB per source;
- source count: 100,000;
- hard traversal ceiling: 256 levels;
- explicit caller `--max-depth` may only lower the traversal ceiling.

## Performance baseline policy

The committed measurements are runner-specific baselines, not an SLA. CI detects severe regressions using a broad `3x` median-time or output-size threshold on committed local fixtures. External/network specimens remain separate from deterministic local fixture proof.

## Final gate

After this candidate is merged, DEV prerelease and stable `v0.1.10` must complete with validation, pinned external reference smoke, generated docs, and stable back-sync. Only then may #48 publish one of:

- `GO — PHASE 7 MAY OPEN`; or
- `NO-GO — PHASE 7 REMAINS GATED` with blocker references.

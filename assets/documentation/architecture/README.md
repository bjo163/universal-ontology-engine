# OX-DX Brand Architecture

This directory is the canonical governance layer for how OX-DX names brands, products, engines, specifications, tools, interfaces, repositories, packages, documentation, domains, releases, and future extensions.

## Start here

**Primary reference:** `brand-architecture.md`

Specialized policies:

- `repository-naming.md` — repository identity and `ox-dx-*` policy.
- `package-naming.md` — Rust crates, packages, libraries, and CLI naming.
- `product-naming.md` — product, tool, interface, documentation, and extension rules.
- `responsibility-matrix.md` — one owner per capability.
- `repository-map.md` — verified current repository plus explicitly planned future repositories.
- `versioning.md` — brand/product/engine/ontology/schema/release version boundaries.
- `domain-architecture.md` — future public-domain naming.
- `naming-anti-patterns.md` — patterns that create collision, hype, or semantic drift.

## Authority

For naming/governance questions, use this order:

1. existing normative technical names and contracts;
2. `brand-architecture.md`;
3. specialized architecture policy in this directory;
4. communication naming summary in `../communication/naming-conventions.md`;
5. campaign/editorial copy.

Brand architecture does not redefine ontology semantics. Technical truth remains governed by the specification, schemas, engineering standards, implementation, tests, and release evidence.

## Existing names are protected

This architecture does **not** rename:

- `universal-ontology-engine`;
- existing Rust crates;
- `ontology-engine` CLI binary;
- ontology TYPE names;
- edge kinds;
- normative specification/schema filenames;
- phases.

Future consistency must not erase useful technical history.

**One brand. Clear ownership. No semantic drift.**

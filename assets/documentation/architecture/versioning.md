# OX-DX Versioning & Product Identifiers

Version numbers describe compatibility/release boundaries. They must not blur brand, product, engine, ontology, and schema identity.

## Version layers

| Layer | Identifier | Version rule |
|---|---|---|
| Brand | **OX-DX** | Normally unversioned. A brand identity does not inherit a technical version automatically. |
| Product/system | **Universal Ontology & Experience Engine** | Version only if a future product contract requires one. Do not borrow engine/ontology version casually. |
| Engine | **Universal Ontology Engine** | Follows repository software release train. |
| Ontology | **Universal Ontology** | Normative ontology contract version. |
| Schema | specific schema artifact | Version according to its compatibility contract/filename/schema metadata. |
| Tool/interface | specific tool/product | Own version only when independently shipped. |

## Current verified snapshot

At the repository audit on **2026-09-14**:

- latest stable engine release: **v0.1.4**;
- root `main/Cargo.toml`: **0.1.4**;
- Universal Ontology contract: **v1.0.0**.

These are separate facts.

## Essential distinction

Do not say:

> OX-DX is v1.0.0

merely because Universal Ontology is v1.0.0.

Do not say:

> the ontology is v0.1.4

because the Rust engine release is v0.1.4.

Preferred:

> Universal Ontology v1.0.0 is implemented by the Universal Ontology Engine, whose software releases follow an independent 0.x train.

## Stable release

Use normal machine-friendly semantic release identifiers already established by the repository, for example:

`v0.1.4`

A stable channel means stable within the repository's release policy. It does not automatically mean every roadmap capability is complete.

## Development release

Use machine-friendly prerelease identifiers following repository automation conventions, for example:

`v0.1.4-dev.15`

Do not invent branded release names for ordinary development builds.

## Experimental product

Mark the product/capability status explicitly:

**EXPERIMENTAL**

Then give its technical version separately if one exists.

Do not encode experimentation only through a clever codename.

## Milestone

A milestone names an acceptance boundary or program checkpoint.

A milestone is not automatically:
- a software release;
- a product version;
- an ontology version.

## Schema versioning

Schema identity should be explicit and technical.

Do not infer schema compatibility from the OX-DX brand or engine version.

Where normative files already have established names, preserve them.

## Deprecation and migration

Naming/version changes follow these rules:

1. never rename casually;
2. document old → new mapping;
3. preserve aliases/compatibility where practical;
4. mark deprecated identifiers explicitly;
5. update links, examples, package metadata, and automation;
6. define a migration window when external consumers are affected;
7. retain historical release references;
8. do not erase the technical lineage merely to make branding symmetrical.

## Volatile version data

General brand documentation should avoid hardcoding “latest” versions unless the text is explicitly a dated audit snapshot.

For live truth, prefer generated release metadata and repository state.

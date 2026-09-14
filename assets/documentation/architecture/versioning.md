# OX-DX Versioning & Product Identifiers

Master brand authority:

`../OX-DX-BRAND-BIBLE-v1.0.md`

This file is the canonical specialist for version boundaries.

## Version layers

| Layer | Identifier | Version rule |
|---|---|---|
| Brand identity | **OX-DX** | Normally unversioned as a public name. |
| Brand system | **OX-DX Brand Bible** | Versioned; current canonical system is **v1.0**. |
| Product/system | **Universal Ontology & Experience Engine** | Version only if a future product contract requires one. |
| Engine | **Universal Ontology Engine** | Independent software release train. |
| Ontology | **Universal Ontology** | Normative ontology contract version. |
| Schema | specific schema artifact | Version according to schema compatibility/contract. |
| Tool/interface | specific independently shipped surface | Own version only if it has a real independent lifecycle. |

## Current audit snapshot

On 2026-09-14 07:14 UTC:

- **OX-DX Brand Bible:** v1.0;
- **Universal Ontology:** v1.0.0;
- **Universal Ontology Engine stable release observed:** v0.1.7;
- **main Cargo version:** 0.1.7;
- **latest preceding prerelease observed:** v0.1.7-dev.21.

The engine version is volatile. Live truth comes from GitHub Releases/generated repository status.

## Do not conflate

Incorrect:

> OX-DX is v1.0.0.

Incorrect:

> The ontology is v0.1.7.

Preferred:

> OX-DX Brand Bible v1.0 defines the stable brand system. Universal Ontology v1.0.0 is the normative ontology contract. Universal Ontology Engine releases follow an independent software release train.

## Brand-system versioning

### v1.x
Backward-compatible evolution:
- clarification;
- accessibility improvement;
- additive application guidance;
- new compliant examples;
- non-breaking governance refinements.

### v2.0
Potentially breaking:
- master identity geometry;
- core descriptor/hierarchy;
- primary brand promise;
- major voice architecture;
- major palette/visual-grammar replacement.

## Stable engine release

Use machine-friendly semantic identifiers such as:

`v0.1.7`

Stable means stable within repository release policy, not completion of every roadmap phase.

## Development release

Use repository automation conventions such as:

`v0.1.7-dev.21`

Do not create branded codenames for ordinary development builds.

## Experimental products

Mark maturity explicitly:

**EXPERIMENTAL**

Then state the technical version separately if one exists.

## Milestones

A milestone is an acceptance/program boundary.

It is not automatically:
- a brand version;
- software release;
- product version;
- ontology version.

## Migration

Version/naming changes must:
1. document old → new;
2. preserve compatibility/aliases when practical;
3. update references and automation;
4. state breaking impact;
5. define migration path;
6. retain historical release lineage.

General brand documentation should not hardcode “latest engine” except in a clearly dated audit snapshot.

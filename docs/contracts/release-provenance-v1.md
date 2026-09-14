# OX-DX Release Provenance Contract v1

Status: **MVP EXIT / Phase 1–6**

The Universal Ontology contract version and implementation release version are intentionally independent:

- ontology contract: `1.0.0`;
- Rust implementation: `0.x` release train.

## Reproducible dependency policy

- `Cargo.lock` is committed because the repository ships runnable binaries/workflows.
- release verification uses locked dependency resolution;
- a stable release must be built/tested from the exact tree that receives the stable version bump;
- changing ontology registry content without an explicit ontology contract change is prohibited.

## Release lineage

A normal release lineage is:

```text
feature/* PR
   ↓
dev merge SHA
   ↓
DEV version-bump SHA
   ↓
DEV prerelease tag
   ↓
dev → main promotion merge SHA
   ↓
stable version-bump SHA
   ↓
stable tag
   ↓
generated docs/status commit(s)
   ↓
main → dev back-sync
```

The stable Git tag MUST point at the stable version-bump commit, not merely the promotion merge parent.

## Required release facts

A stable release must make the following independently inspectable:

- implementation version;
- ontology contract version;
- stable tag;
- source commit SHA;
- committed `Cargo.lock` state;
- canonical registry path/version;
- validation result;
- self-inspection result;
- generated repository status after release.

## Generated documentation

Generated README/status/changelog commits may occur after the stable tag. They MUST NOT alter executable source for the tagged release and MUST report the tagged implementation version accurately. Stable Back Sync reconciles those generated commits back to `dev`.

## Mismatch policy

Release preparation MUST fail or remain unreleased when package version, planned tag, lockfile, or canonical registry state is inconsistent. A release workflow must never force-push over competing main changes; it must fetch/rebase/retry or stop.

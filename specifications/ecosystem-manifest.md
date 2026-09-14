# Ecosystem Manifest Contract v0.1

## Purpose

Define the minimal envelope an ecosystem exposes to Universe Foundation for identity and discovery.

This manifest is not the ecosystem's internal project/repository contract. It is a boundary document.

## Canonical location

A conforming ecosystem SHOULD expose `ecosystem.json` at its ecosystem root when managed by Universe Foundation.

## Required fields

```json
{
  "contract_version": "0.1",
  "id": "rocksoul",
  "name": "ROCKSOUL",
  "foundation": "ecosystem-foundation"
}
```

- `contract_version`: version of this envelope contract.
- `id`: stable ecosystem identifier; lowercase kebab-case.
- `name`: human-readable ecosystem name.
- `foundation`: ecosystem-level contract identifier when one is adopted.

## Optional fields

An ecosystem MAY expose:

- `description`
- `version`
- `repository`
- `organization`
- `lifecycle`
- `capabilities`
- `links`

Universe Foundation MUST treat unknown optional fields as ecosystem-owned metadata.

## Ownership

Universe Foundation MAY read this manifest for discovery and identity checks.

Universe Foundation MUST NOT use this envelope to redefine ecosystem-internal ownership, project structure, repository structure, source layout, or implementation conventions.

## Identity rule

The `id` is the stable cross-boundary identity. Filesystem path and repository directory name are operational metadata and may change.

## Compatibility

An ecosystem may adopt this envelope without adopting `ecosystem-foundation`. When `foundation` is `ecosystem-foundation`, the universe layer must still delegate all internal semantics to the ecosystem contract.

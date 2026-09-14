# Universe Foundation

Canonical foundation for organizing multiple software ecosystems under one managed universe/workspace.

## Scope

Universe Foundation defines only the layer above an ecosystem:

```text
UNIVERSE
└── ECOSYSTEM
    └── ... ecosystem-owned structure
```

It owns cross-ecosystem identity, discovery, registry, workspace layout, relationships, and orchestration boundaries.

It does **not** own project, repository, source, module, domain semantics, or implementation conventions inside an ecosystem. Those belong to `ecosystem-foundation` and the ecosystem itself.

## Canonical layers

```text
UNIVERSE
    ↓
ECOSYSTEM
    ↓
ORGANIZATION / DOMAIN / PROJECT / REPOSITORY / SOURCE / MODULE
```

`DOMAIN` is optional. The lower layers are governed by the ecosystem contract.

## Repository layout

```text
universe-foundation/
├── governance/
├── specifications/
├── standards/
├── schemas/
├── templates/
├── tools/
├── tests/
└── README.md
```

## Contract

Current contract version: **0.1**.

See:

- `governance/hierarchy.md`
- `specifications/universe-contract.md`
- `specifications/universe-contract.json`
- `schemas/universe.schema.json`

## Relationship to Ecosystem Foundation

`universe-foundation` is the parent-level contract.

`ecosystem-foundation` remains the canonical contract for structure inside an ecosystem.

```text
universe-foundation
        ↓
ecosystem-foundation
        ↓
ecosystem-* repositories
```

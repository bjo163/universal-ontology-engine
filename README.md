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

## Contracts

Universe contract: **0.2**

Ecosystem manifest envelope: **0.1**

See:

- `governance/hierarchy.md`
- `specifications/universe-contract.md`
- `specifications/universe-contract.json`
- `specifications/universe-contract.instance.json`
- `specifications/ecosystem-manifest.md`
- `schemas/universe.schema.json`
- `schemas/ecosystem-manifest.schema.json`
- `standards/workspace.md`

## Registry and CLI

The canonical universe registry is `universe.json`.

```bash
python tools/universe_cli.py list
python tools/universe_cli.py inspect rocksoul
python tools/universe_cli.py validate
python tools/universe_cli.py discover X:\REPO\universe
python tools/universe_cli.py status X:\REPO\universe
python tools/universe_cli.py doctor X:\REPO\universe
```

Use `--json` for launcher/orchestrator integration.

## Relationship to Ecosystem Foundation

`universe-foundation` is the parent-level contract.

`ecosystem-foundation` remains the canonical contract for structure inside an ecosystem.

The ecosystem manifest is only the boundary envelope needed for identity and discovery. It does not redefine the ecosystem's internal hierarchy.

```text
universe-foundation v0.2
        ↓
ecosystem manifest envelope v0.1
        ↓
ecosystem-foundation v0.x (optional)
        ↓
ecosystem-owned repositories
```

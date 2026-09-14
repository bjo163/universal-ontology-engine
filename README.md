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
├── universe.json
└── README.md
```

## Contract

Current contract version: **0.2**.

See:

- `governance/hierarchy.md`
- `specifications/universe-contract.md`
- `specifications/universe-contract.json`
- `specifications/universe-contract.instance.json`
- `schemas/universe.schema.json`
- `standards/workspace.md`

## Canonical registry

`universe.json` is the operational Universe registry used by tooling. The stable ecosystem `id` is the identity; `path` is workspace metadata.

The current reference registry contains ROCKSOUL, MoonWitness, CRAYON, and ISP.

## Registry / discovery CLI

The dependency-free Python CLI is `tools/universe_cli.py`.

```text
python tools/universe_cli.py list
python tools/universe_cli.py inspect rocksoul
python tools/universe_cli.py validate
python tools/universe_cli.py discover X:\\REPO\\universe
python tools/universe_cli.py status X:\\REPO\\universe
```

Add `--json` for machine-readable output:

```text
python tools/universe_cli.py --json list
python tools/universe_cli.py --json status X:\\REPO\\universe
```

`list` reads the registry only. `discover` scans the workspace for `ecosystem-*` directories and reports registered, missing, and unknown entries. `status` combines registry entries with local filesystem/Git presence.

## Relationship to Ecosystem Foundation

`universe-foundation` is the parent-level contract.

`ecosystem-foundation` remains the canonical contract for structure inside an ecosystem.

```text
universe-foundation v0.2
        ↓
ecosystem-foundation v0.x
        ↓
ecosystem repositories
```

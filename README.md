# Universe Foundation

Canonical foundation for the complete software hierarchy. One foundation governs the full chain from the Universe boundary down to concrete Implementation.

## Canonical hierarchy

```text
UNIVERSE
└── ECOSYSTEM
    └── ORGANIZATION (optional)
        └── DOMAIN (optional)
            └── PROJECT
                └── REPOSITORY
                    └── SOURCE
                        └── UNIT
                            └── MODULE
                                └── COMPONENT
                                    └── ELEMENT
                                        └── IMPLEMENTATION
```

This is one continuous semantic hierarchy. There is no required `ecosystem-foundation` repository, sub-foundation, or second governance layer.

`DOMAIN` and `ORGANIZATION` may be omitted when they do not apply. The remaining concepts retain their meaning without requiring artificial wrapper layers.

## What this foundation owns

Universe Foundation defines and validates:

- identity and ownership boundaries for every canonical layer;
- discovery and workspace topology;
- repository and source boundaries;
- universal internal implementation vocabulary;
- native-language/framework mappings;
- relationships between layers and across ecosystems;
- validation, health, and orchestration contracts.

A concrete repository may use native structures such as `src`, `apps`, `packages`, `crates`, `cmd`, `pkg`, `internal`, `modules`, and `components`. These are implementation mappings, not mandatory universal directory names.

## Native mapping

```text
UNIT
├── Rust      → crate
├── Node      → package / application / library
├── Go        → package / command / service
├── Python    → package / module
└── Java      → module / package / application
```

Likewise, `function`, `method`, `type`, `class`, `handler`, and similar language constructs are mapped to `ELEMENT` or `IMPLEMENTATION` according to the relevant profile.

The contract MUST NOT force a repository to create physical directories named `unit/`, `module/`, `component/`, `element/`, or `implementation/`.

## Registry

The canonical top-level registry is `universe.json`. It identifies ecosystems and their operational locations. Internal ecosystem/project/repository details are discovered from those boundaries rather than by introducing another foundation repository.

An `ecosystem.json` file may be used as an optional lightweight discovery manifest at an ecosystem root. It is metadata, not a second foundation contract.

## Repository layout

```text
universe-foundation/
├── governance/
├── specifications/
├── standards/
├── schemas/
├── tools/
├── tests/
└── README.md
```

## Specifications

- `governance/hierarchy.md` — normative semantic hierarchy and ownership rules.
- `specifications/universe-contract.md` — complete foundation contract.
- `specifications/universe-contract.json` — machine-readable contract schema.
- `specifications/universe-contract.instance.json` — reference instance.
- `specifications/foundation-contract.md` — universal lower-layer semantics and native mapping.
- `specifications/foundation-contract.json` — machine-readable hierarchy contract.
- `schemas/universe.schema.json` — registry schema.
- `standards/workspace.md` — workspace and discovery conventions.

## CLI

```bash
python tools/universe_cli.py list
python tools/universe_cli.py inspect rocksoul
python tools/universe_cli.py validate
python tools/universe_cli.py discover X:\REPO\universe
python tools/universe_cli.py status X:\REPO\universe
python tools/universe_cli.py doctor X:\REPO\universe
```

Use `--json` for launcher/orchestrator integration.

## Foundation chain

The complete model is intentionally a single chain:

```text
universe-foundation
        ↓
UNIVERSE
        ↓
ECOSYSTEM
        ↓
ORGANIZATION?
        ↓
DOMAIN?
        ↓
PROJECT
        ↓
REPOSITORY
        ↓
SOURCE
        ↓
UNIT
        ↓
MODULE
        ↓
COMPONENT
        ↓
ELEMENT
        ↓
IMPLEMENTATION
```

No lower-level foundation repository is required. Native repositories remain implementation containers governed by this same universal contract.
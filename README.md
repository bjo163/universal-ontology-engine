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

This is one continuous semantic hierarchy. There is no required ecosystem-foundation repository, sub-foundation, or second normative contract.

## What this foundation owns

Universe Foundation defines and validates:

- identity and ownership boundaries for every canonical layer;
- discovery and workspace topology;
- project, repository, and source boundaries;
- universal implementation vocabulary from Unit through Implementation;
- native language/framework mappings;
- relationships within and across ecosystems;
- validation, health, and orchestration contracts.

Native structures remain valid. `src`, `apps`, `packages`, `crates`, `cmd`, `pkg`, `internal`, `modules`, and `components` are implementation mappings, not mandatory universal directories.

## Universal identity

Every canonical node SHOULD have a stable identifier within its parent scope. Paths and directory names are operational metadata unless explicitly declared as identity.

The generic node contract is `schemas/foundation-node.schema.json`.

## Registry

`universe.json` is the top-level registry. It identifies ecosystems and their operational locations. Lower-level entities are discovered through explicit parent/child references rather than a second foundation contract.

An `ecosystem.json` file MAY be used as an optional local discovery manifest, but it is metadata only and MUST NOT redefine the canonical hierarchy.

## Foundation contract

The normative contract is v0.3 and is represented by:

- `governance/hierarchy.md` — canonical hierarchy and ownership rules.
- `specifications/universe-contract.md` — complete normative contract.
- `specifications/universe-contract.json` — machine-readable contract schema.
- `specifications/universe-contract.instance.json` — reference contract instance.
- `specifications/foundation-contract.md` — universal lower-layer semantics and mappings.
- `specifications/foundation-contract.json` — machine-readable foundation schema.
- `specifications/hierarchy-model.md` — canonical node/parent/discovery model.
- `schemas/universe.schema.json` — Universe registry schema.
- `schemas/foundation-node.schema.json` — generic canonical node schema.

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

## Native mapping

```text
UNIT
├── Rust      → crate
├── Node      → package / application / library
├── Go        → package / command / service
├── Python    → package / module
└── Java      → module / package / application
```

Lower constructs such as module, class, struct, function, method, interface, handler, and concrete logic are mapped semantically to `MODULE`, `COMPONENT`, `ELEMENT`, and `IMPLEMENTATION` as appropriate for the language/framework.

The foundation never requires artificial `unit/`, `module/`, `component/`, `element/`, or `implementation/` directories.

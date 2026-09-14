# Universe CLI Standard

## Purpose

Define the stable, dependency-free command surface for inspecting a Universe registry and its local workspace.

## Command surface

```text
universe list
universe inspect <ecosystem-id>
universe validate
universe discover <workspace>
universe status <workspace>
universe doctor <workspace>
python tools/universe-hierarchy.py <workspace> --json
```

`--manifest <path>` selects an alternate Universe manifest for registry commands. `--json` emits machine-readable output intended for launchers and automation.

## Hierarchy discovery

`universe-hierarchy.py` is read-only and maps native workspace/repository structures to the semantic chain:

```text
UNIVERSE → ECOSYSTEM → ORGANIZATION? → DOMAIN? → PROJECT → REPOSITORY → SOURCE → UNIT → MODULE → COMPONENT → ELEMENT → IMPLEMENTATION
```

It MUST preserve native directories and report lower-level mappings as semantic classifications. It MUST NOT create or rename directories.

## Ownership

The CLI may read the Universe registry and local filesystem state. It MUST NOT modify an ecosystem repository as part of read-only commands.

`doctor` is a consistency check, not a repair command. It verifies registry validity, registered workspace directories, `ecosystem.json` identity when present, and unknown `ecosystem-*` directories.

## Identity rule

The registry `ecosystems[].id` is authoritative for Universe-level identity. A local `ecosystem.json` may confirm that identity, but a directory name alone MUST NOT become identity.

## Exit codes

- `0` = successful command / healthy validation
- `1` = validation or consistency failure

## Stability

Existing registry commands remain backward compatible. Hierarchy discovery is additive and dependency-free.

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
```

`--manifest <path>` selects an alternate Universe manifest. `--json` emits machine-readable output intended for launchers and automation.

## Ownership

The CLI may read the Universe registry and local filesystem state. It MUST NOT modify an ecosystem repository as part of read-only commands.

`doctor` is a consistency check, not a repair command. It verifies registry validity, registered workspace directories, `ecosystem.json` identity when present, and unknown `ecosystem-*` directories.

## Identity rule

The registry `ecosystems[].id` is authoritative for Universe-level identity. A local `ecosystem.json` may confirm that identity, but a directory name alone MUST NOT become identity.

## Exit codes

- `0` = successful command / healthy validation
- `1` = validation or consistency failure

## Stability

The command names above are the v0.2 CLI contract. Future commands MAY be added, but existing command semantics SHOULD remain backward compatible within a contract major version.

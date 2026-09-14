# OX-DX CLI Compatibility Contract v1

Status: **MVP EXIT / Phase 1–6**  
Applies to implementation release `v0.1.10` and later `0.1.x` releases unless explicitly superseded.

## Stable command surface

| Command | Stability | stdout | Notes |
|---|---|---|---|
| `validate` | stable | human text | validates canonical registry |
| `levels` | stable | human text | canonical ordered level list |
| `inspect <1..49>` | stable | human text | one canonical level |
| `self [root] --json` | stable machine surface | JSON only | self-health snapshot |
| `discover <workspace>` | stable machine surface | JSON only | structural discovery |
| `discover <workspace> --rust-ast` | legacy-compatible stable | JSON only | native Rust syntax projection |
| `discover <workspace> --syntax` | stable canonical syntax surface | JSON only | per-file syntax adapters |
| `parse <language> <file>` | stable machine surface | JSON only | single-file adapter evidence |

`--rust-ast` and `--syntax` are mutually exclusive. `--rust-ast` remains supported for backwards compatibility; new cross-language integrations should prefer `--syntax`.

## Global registry resolution

`--registry PATH` has explicit command-line precedence. When omitted, the CLI uses `specifications/universal-ontology-v1.0.json` relative to the current process working directory. Consumers running outside the repository MUST pass an explicit registry path.

## Machine-output rule

Commands documented as machine surfaces MUST NOT emit informational prose to stdout. Diagnostics belong on stderr. JSON field additions are allowed only under the discovery schema compatibility rules; removing, renaming, or changing required-field meaning is breaking.

## Stable process exit codes

| Code | Class | Meaning |
|---:|---|---|
| `0` | success | requested operation completed |
| `2` | input/configuration | invalid level/language/path/registry, unreadable requested input, Clap usage failure |
| `3` | engine/invariant | graph invariant, internal projection, or machine-output failure |
| `4` | health gate | self inspection completed but reported degraded/unhealthy state |

Malformed source that is contractually preserved as an observation is NOT by itself a process failure.

## Deprecation policy

A stable command/flag may be deprecated only by:
1. documenting the replacement;
2. preserving the old behavior for at least the remainder of the current minor line where feasible;
3. marking any removal or meaning change as a breaking machine-contract change.

## Compatibility boundaries

This contract freezes the Phase 1–6 command surface. It does **not** authorize Phase 7 semantic projection. Future semantic projection must be explicit opt-in and separately versioned.

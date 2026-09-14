# OX-DX Discovery Safety & Path Contract v1

Status: **MVP EXIT / Phase 1–6**

## Read-only guarantee

Discovery does not write into the selected target workspace. Reports, CI artifacts, and repository automation are produced outside the scanned target.

## Logical path form

Observation paths are workspace-relative when they belong to the selected workspace. Machine output normalizes separators to `/`. Source location remains provenance and is not treated as semantic identity.

## Public CLI preflight

Before any structural, Rust AST, or cross-language syntax scan, the public CLI checks the selected workspace.

The Phase 1–6 contract is intentionally strict:

- workspace root must be a real directory;
- symbolic links encountered in the scanned tree are rejected rather than followed;
- generated/ignored directories are excluded consistently;
- source files larger than **2 MiB** are rejected before parsing;
- more than **100,000 source files** are rejected;
- traversal has a hard ceiling of **256 levels**;
- caller `--max-depth` may lower that ceiling but cannot raise it.

These limits are resource-boundary decisions, not ontology claims. Rejected input does not cause synthetic ontology nodes to be created.

## Expected behavior

| Condition | Result |
|---|---|
| missing workspace | input/configuration failure |
| non-directory workspace | input/configuration failure |
| unreadable path | input/configuration/I/O failure |
| symbolic link | preflight failure; not followed |
| oversized source | preflight failure; not parsed |
| excessive source count | preflight failure |
| malformed Rust source inside an accepted tree | parse-error observation; graph remains valid |
| unsupported syntax construct | no invented semantic node |
| ignored/generated directory | skipped deterministically |
| sparse repository | only evidence-backed levels materialize |

## Identity and platform rules

Structural file-boundary IDs may include normalized workspace-relative paths. Named syntax identities are parent-scoped and are not derived solely from line or column. `SourceSpan` remains provenance.

The emitted logical separator is `/` on all hosts. Host filesystem case behavior is respected; OX-DX does not silently lowercase semantic identifiers.

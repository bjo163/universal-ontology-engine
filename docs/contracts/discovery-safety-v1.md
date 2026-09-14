# OX-DX Discovery Safety & Path Contract v1

Status: **MVP EXIT / Phase 1–6**

## Read-only guarantee

Discovery MUST NOT write into the selected target workspace. Generated reports, CI artifacts, release metadata, and repository automation operate outside the scanned target or inside the engine repository itself.

## Logical path form

Paths emitted by discovery observations are workspace-relative where the observed path is inside the selected workspace. Separators are normalized to `/` for machine output. Provenance remains tied to the observed source path; normalization MUST NOT be used to invent semantic identity.

## Workspace boundary

- traversal starts at the selected workspace root;
- symlinked directories/files are not followed by MVP discovery;
- ignored/generated directories remain excluded after path normalization;
- `.` and `..` resolution is delegated to the host filesystem before traversal decisions where canonicalization is required;
- traversal MUST NOT escape the selected workspace through links/junction-style indirection;
- default discovery does not execute target code.

## Resource limits

MVP source parsing applies a per-source safety ceiling of **2 MiB**. A source file above that threshold remains structurally observable as a file boundary but syntax parsing is skipped and recorded as an explicit observation. This is a safety limit, not a semantic statement.

`--max-depth` constrains recursive discovery depth. Reaching a configured limit terminates that branch of traversal without manufacturing intermediate ontology nodes.

## Hostile or messy workspaces

Expected behavior:

| Condition | Result |
|---|---|
| missing workspace | input/configuration failure |
| workspace is not a directory | input/configuration failure |
| unreadable directory | input/configuration/I/O failure |
| malformed Rust syntax | parse-error observation, graph remains valid |
| unsupported syntax construct | no invented semantic node |
| oversized source | explicit `source-skipped` observation |
| symlink file/directory | skipped; no traversal outside root |
| ignored/generated directory | skipped deterministically |
| sparse repository | only evidence-backed levels are materialized |

## Identity vs provenance

Workspace-relative paths may participate in structural IDs for file-boundary nodes. Semantic IDs for named syntax observations are parent-scoped and MUST NOT be derived solely from source line/column. `SourceSpan` is provenance, not identity.

## Platform expectations

The logical output contract uses `/` separators independent of host separator. Windows drive letters/UNC input paths are host concerns; once a path is inside the workspace, emitted observation paths use workspace-relative logical form. Case sensitivity follows the host filesystem for discovery; OX-DX does not silently lowercase semantic identifiers.

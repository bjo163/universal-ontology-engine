# Hierarchy Discovery Standard

## Purpose

Define how Universe Foundation maps a real workspace and native repositories onto the universal semantic hierarchy without requiring canonical directory names.

## Canonical chain

```text
UNIVERSE → ECOSYSTEM → ORGANIZATION? → DOMAIN? → PROJECT → REPOSITORY → SOURCE → UNIT → MODULE → COMPONENT → ELEMENT → IMPLEMENTATION
```

## Discovery principles

1. Discovery is additive and read-only by default.
2. Native repository structure is preserved.
3. Semantic levels are classifications, not mandatory directories.
4. An explicit manifest wins over filename heuristics.
5. Native manifests and language metadata are used as evidence.
6. Ambiguous lower-level constructs MUST be reported with confidence rather than invented as facts.
7. External systems are represented as relationships.

## Evidence precedence

```text
explicit foundation metadata
    > repository metadata
    > native build/workspace manifest
    > recognized directory/file conventions
    > conservative heuristic
```

## Default mappings

### Repository → Source
Recognized source roots include `src/`, `app/`, `apps/`, `packages/`, `crates/`, `cmd/`, `internal/`, `lib/`, and language-specific roots.

### Source → Unit
Use workspace/package/module manifests first. Otherwise classify obvious native containers such as Rust crates, Node packages/workspaces, Go commands/packages, Python packages, and Java modules.

### Unit → Module
Use explicit language modules/namespaces or stable source groupings.

### Module → Component
Use cohesive feature/service/class/subsystem boundaries when confidently detectable.

### Component → Element
Use functions, methods, types, interfaces, handlers, constants, and analogous constructs.

### Element → Implementation
Represent the concrete implementation location or construct. This layer may be a file span, symbol span, generated AST node, or other implementation reference rather than a directory.

## Output contract

Discovery emits a tree of nodes using `schemas/hierarchy-node.schema.json`.

Each node contains at least:

```json
{
  "level": "UNIT",
  "id": "stable-or-scoped-id",
  "name": "native-name",
  "path": "relative/path"
}
```

Optional fields identify `native_type`, `language`, `metadata`, and nested `children`.

## Safety

Discovery MUST ignore generated and dependency trees by default, including `.git/`, `node_modules/`, `.next/`, `target/`, `dist/`, `build/`, coverage output, and similar generated paths. An explicit override may opt into them.

Discovery MUST never mutate source repositories.

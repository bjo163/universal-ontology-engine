# Hierarchy Discovery Standard v0.3

## Purpose

Discovery maps a native workspace into the universal semantic hierarchy without modifying repository layout.

```text
UNIVERSE → ECOSYSTEM → ORGANIZATION? → DOMAIN? → PROJECT → REPOSITORY → SOURCE → UNIT → MODULE → COMPONENT → ELEMENT → IMPLEMENTATION
```

## Evidence precedence

Discovery MUST prefer evidence in this order:

1. explicit hierarchy metadata supplied by the repository/tool;
2. repository manifests and workspace metadata;
3. native language/framework structure;
4. directory and file structure;
5. conservative heuristics.

A lower-precedence heuristic MUST NOT overwrite a higher-confidence explicit classification.

## Symbol-aware discovery

For recognized source files, `ELEMENT` SHOULD represent a real language construct rather than the file itself.

The reference parser recognizes high-confidence declarations for Rust, TypeScript, JavaScript, Go, Python, Java, and Kotlin using only the standard library.

Each parser-backed Element SHOULD expose:

```json
{
  "metadata": {
    "symbol": "App",
    "kind": "function",
    "confidence": "high",
    "evidence": "language-parser",
    "span": {
      "line_start": 1,
      "line_end": 3
    }
  }
}
```

`IMPLEMENTATION` SHOULD point to the same concrete symbol span and retain the native language kind.

The parser is intentionally conservative. It is not an AST replacement and MUST NOT claim syntax-tree precision it cannot establish.

## Native mapping

Native structures remain unchanged. Examples:

```text
src/                 → SOURCE
crate/package/app    → UNIT
module/namespace     → MODULE
class/service/feature → COMPONENT
function/method/type → ELEMENT
body/logic           → IMPLEMENTATION
```

The discovery engine MUST NOT create `unit/`, `module/`, `component/`, `element/`, or `implementation/` directories to satisfy the semantic hierarchy.

## Generated and dependency output

Dependency, build, cache, and generated directories SHOULD be excluded by default, including `.git`, `node_modules`, `target`, `dist`, `build`, `.next`, coverage directories, and Python virtual environments/caches.

## Confidence

Every heuristic or parser-derived classification SHOULD include confidence/evidence metadata. `high` means the native construct was directly recognized; `medium` means native structure strongly suggests the classification; `low` means a fallback grouping was required.

## Read-only requirement

Discovery is observational. It MUST NOT modify source code, manifests, directory names, or repository metadata.

## Stable identity

Node IDs MUST remain stable for an unchanged semantic path. Local path names are evidence and operational metadata; they are not universal identity by themselves.

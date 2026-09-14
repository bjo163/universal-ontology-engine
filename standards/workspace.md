# Universe Workspace Standard

## Purpose

Define the recommended local filesystem topology for working with multiple independent ecosystems.

## Canonical local layout

```text
<workspace-root>/
├── universe manifest
├── ecosystem-<id>/
│   ├── ecosystem manifest (optional)
│   ├── <repository>/
│   └── <repository>/
├── ecosystem-<id>/
└── ecosystem-<id>/
```

The workspace root is operational. It is not itself a Git repository unless the operator explicitly chooses to make it one.

## Naming

`ecosystem-<id>` is a recommended physical directory name, not the identity contract.

The stable ecosystem identity is the registry `id`. A path MAY change without changing ecosystem identity.

Repository directory names are governed by the repository and ecosystem conventions. Universe Foundation does not rename repositories.

## Separation of responsibility

Universe Foundation manages:

- workspace discovery
- ecosystem registry
- ecosystem-to-ecosystem relationships
- cross-ecosystem orchestration boundaries

Ecosystem Foundation manages the hierarchy below an ecosystem boundary.

A repository manages its own source layout, native tooling, build system, and implementation details.

## Recommended example

```text
X:\REPO\universe\
├── ecosystem-rocksoul\
│   ├── rocksoul-ui\
│   ├── rocksoul-web\
│   └── rocksoul-assets\
├── ecosystem-moonwitness\
│   ├── moonwitness-v2\
│   └── mlv-uos\
├── ecosystem-crayon\
│   └── crayon\
└── ecosystem-isp\
    ├── avatar\
    └── tis\
```

This is an operational example only. It does not assert that all listed directories or repositories currently exist.

## Anti-patterns

Do not make Universe Foundation the owner of repository internals.

Do not derive identity solely from a folder name.

Do not require every ecosystem to use the same programming language, package manager, CI provider, or repository layout.

Do not add a `repositories/` wrapper solely because the universe contract exists; direct repository directories are the default.

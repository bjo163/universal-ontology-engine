# OX-DX Package, Crate & CLI Naming

Brand belongs above implementation names.

Technical components should be technically descriptive.

## Verified Rust workspace

Current packages:

| Workspace path | Cargo package |
|---|---|
| `crates/ontology-core` | `ontology-core` |
| `crates/ontology-registry` | `ontology-registry` |
| `crates/ontology-graph` | `ontology-graph` |
| `crates/ontology-discovery` | `ontology-discovery` |
| `crates/ontology-rust` | `ontology-rust` |
| `crates/ontology-language` | `ontology-language` |
| `crates/ontology-cli` | `ontology-engine` |

Current CLI binary:

`ontology-engine`

These names remain unchanged.

## Rust crates

Prefer:

`ontology-<responsibility>`

when the crate is part of the technical engine and the responsibility is specific.

Good:
- `ontology-core`
- `ontology-graph`
- `ontology-discovery`

Do not force every crate to become `ox-dx-*`.

A crate is an implementation boundary, not automatically a public product.

## NPM / other packages

For a future public package:

- use `@ox-dx/<responsibility>` if a scoped public ecosystem package is appropriate and actually established;
- otherwise use a technically descriptive package name consistent with its ecosystem.

This is a naming policy, not a claim that an `@ox-dx` package scope currently exists.

Avoid publishing two names for the same library merely to satisfy branding.

## Libraries

Library names should describe what they do.

Good:
- graph;
- registry;
- discovery;
- language adapter;
- SDK only if it is genuinely a supported developer kit.

Avoid:
- `core-pro-ultra`;
- `quantum-engine`;
- `magic-runtime`;
- generic `ai-core` without a precise technical contract.

## CLI naming

### Current implementation

Package/binary:

`ontology-engine`

Current subcommands include:
- `validate`
- `levels`
- `inspect`
- `discover`
- `parse`
- `self`

Do not rename these in a brand architecture task.

### Future public direction

A future public wrapper or deliberate binary migration may expose:

`ox-dx <subcommand>`

Example direction:

```text
ox-dx validate
ox-dx levels
ox-dx inspect
ox-dx discover
```

Status: **PLANNED NAMING DIRECTION ONLY**

A future migration must separately define compatibility, aliases, packaging, release behavior, and deprecation.

## CLI verbs

Prefer existing, precise verbs.

A new verb should:
1. describe one action;
2. avoid marketing language;
3. not collide with an ontology TYPE in a confusing way;
4. exist only when the underlying capability exists.

Do not add a `query` command merely because the word appears in a roadmap.

## Brand command vs implementation binary

**OX-DX** is the brand.

`ox-dx` may become a public command namespace in the future.

`ontology-engine` is the current implementation binary.

Keep these distinctions explicit until a migration is approved.
